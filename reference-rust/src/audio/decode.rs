use std::f32::consts::PI;

use rustfft::{num_complex::Complex, FftPlanner};

use crate::error::AILLError;

use super::constants::*;

/// Decodes PCM audio back into AILL wire-format bytes.
pub struct AcousticDecoder {
    sample_rate: u32,
}

/// A detected symbol: which half (hi/lo) and what nibble value.
#[derive(Debug, Clone, Copy)]
struct Symbol {
    half: Half,
    value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Half {
    Hi,
    Lo,
}

impl AcousticDecoder {
    pub fn new() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
        }
    }

    pub fn with_sample_rate(sample_rate: u32) -> Self {
        assert!(
            sample_rate >= MIN_SAMPLE_RATE,
            "Sample rate {} too low (minimum {}): Nyquist must exceed highest carrier",
            sample_rate, MIN_SAMPLE_RATE
        );
        Self { sample_rate }
    }

    /// Decode PCM f32 samples into wire bytes.
    pub fn decode(&self, samples: &[f32]) -> Result<Vec<u8>, AILLError> {
        if samples.len() < FFT_SIZE {
            return Err(AILLError::InvalidStructure(
                "Audio too short for FFT analysis".into(),
            ));
        }

        // Precompute Hann window and FFT plan
        let window: Vec<f32> = (0..FFT_SIZE)
            .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / FFT_SIZE as f32).cos()))
            .collect();
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);

        // Phase 1: Find sync chirp — returns the sample offset where data begins
        let data_start_sample = self.find_sync(samples, &window, &fft)?;

        // Phase 2: Compute adaptive threshold by scanning the data region
        let tone_threshold = self.compute_tone_threshold(
            samples, data_start_sample, &window, &fft,
        );

        // Phase 3: Decode symbols at exact frame intervals from sync point
        let symbols = self.decode_symbols_fixed(
            samples, data_start_sample, tone_threshold, &window, &fft,
        );

        // Phase 4: Reassemble bytes
        let bytes = reassemble_bytes(&symbols);
        if bytes.is_empty() {
            return Err(AILLError::InvalidStructure(
                "No bytes recovered from audio".into(),
            ));
        }

        Ok(bytes)
    }

    /// Find the sync chirp and return the sample offset where data begins.
    fn find_sync(
        &self,
        samples: &[f32],
        window: &[f32],
        fft: &std::sync::Arc<dyn rustfft::Fft<f32>>,
    ) -> Result<usize, AILLError> {
        let sr = self.sample_rate as f32;
        let hop = (0.008 * sr).round() as usize; // 8ms hop for finer sync resolution

        // Collect band energies for all windows
        let mut lo_energies: Vec<(usize, f32)> = Vec::new();
        let mut hi_energies: Vec<(usize, f32)> = Vec::new();

        let mut pos = 0;
        while pos + FFT_SIZE <= samples.len() {
            let magnitudes = self.compute_magnitudes(&samples[pos..pos + FFT_SIZE], window, fft);
            let lo = band_energy(&magnitudes, SYNC_LO_BAND.0, SYNC_LO_BAND.1, sr);
            let hi = band_energy(&magnitudes, SYNC_HI_BAND.0, SYNC_HI_BAND.1, sr);
            lo_energies.push((pos, lo));
            hi_energies.push((pos, hi));
            pos += hop;
        }

        if lo_energies.is_empty() {
            return Err(AILLError::InvalidStructure("No analyzable frames".into()));
        }

        // Find peak lo-band energy (chirp start region)
        let max_lo = lo_energies.iter().map(|&(_, e)| e).fold(0.0f32, f32::max);
        let max_hi = hi_energies.iter().map(|&(_, e)| e).fold(0.0f32, f32::max);

        if max_lo < 1e-7 || max_hi < 1e-7 {
            return Err(AILLError::InvalidStructure(
                "No significant energy — cannot find sync chirp".into(),
            ));
        }

        let lo_thresh = max_lo * 0.3;
        let hi_thresh = max_hi * 0.3;

        // Find chirp start: lo-band rises while hi-band is low
        let chirp_start_idx = lo_energies
            .iter()
            .zip(hi_energies.iter())
            .position(|(&(_, lo), &(_, hi))| lo > lo_thresh && hi < hi_thresh)
            .ok_or_else(|| {
                AILLError::InvalidStructure("Could not detect sync chirp start".into())
            })?;

        let chirp_start_pos = lo_energies[chirp_start_idx].0;

        // Find chirp end: hi-band rises after sufficient elapsed time
        let min_elapsed = (SYNC_MIN_ELAPSED_MS / 1000.0 * sr) as usize;
        let max_elapsed = (SYNC_MAX_ELAPSED_MS / 1000.0 * sr) as usize;

        let chirp_end_idx = hi_energies[chirp_start_idx..]
            .iter()
            .position(|&(pos, hi)| {
                let elapsed = pos.saturating_sub(chirp_start_pos);
                hi > hi_thresh && elapsed > min_elapsed && elapsed < max_elapsed
            })
            .map(|i| chirp_start_idx + i)
            .ok_or_else(|| {
                AILLError::InvalidStructure("Could not detect sync chirp end".into())
            })?;

        // Use the detected chirp end position for a more accurate data_start.
        // The hi-band detection fires when the chirp sweeps through 1400-1900Hz,
        // which is near the end of the chirp. Add a small margin for the chirp
        // to finish and the guard silence before the first data symbol.
        let chirp_end_pos = hi_energies[chirp_end_idx].0 + FFT_SIZE / 2;
        let sync_based = chirp_start_pos + (SYNC_DURATION * sr).round() as usize;
        // Use the later of the two estimates to avoid overlapping with the chirp tail
        let data_start = sync_based.max(chirp_end_pos);

        Ok(data_start)
    }

    /// Compute an adaptive tone detection threshold by scanning data region.
    fn compute_tone_threshold(
        &self,
        samples: &[f32],
        data_start: usize,
        window: &[f32],
        fft: &std::sync::Arc<dyn rustfft::Fft<f32>>,
    ) -> f32 {
        let sr = self.sample_rate as f32;
        let frame_samples = (FRAME_TIME * sr).round() as usize;
        let sym_center_offset = (SYMBOL_DURATION * sr / 2.0).round() as usize;

        let mut all_mags: Vec<f32> = Vec::new();

        // Sample a few symbols to estimate signal levels
        for n in 0..20 {
            let center = data_start + n * frame_samples + sym_center_offset;
            let start = center.saturating_sub(FFT_SIZE / 2);
            if start + FFT_SIZE > samples.len() {
                break;
            }

            let magnitudes = self.compute_magnitudes(&samples[start..start + FFT_SIZE], window, fft);
            for &freq in &CARRIER_FREQS {
                all_mags.push(get_bin_mag(&magnitudes, freq, sr));
            }
        }

        if all_mags.is_empty() {
            return ABS_THRESHOLD;
        }

        all_mags.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = all_mags.len();

        let median = all_mags[len / 2];
        let p85 = all_mags[(len as f32 * 0.85).min((len - 1) as f32) as usize];

        // Threshold separating inactive carriers (noise/leakage) from active carriers.
        // With bimodal data (active vs inactive carriers), median lands in the low
        // cluster and p85 in the high cluster. The threshold is placed between them.
        // For sparse or uniform data (e.g., all-zero bytes), p85 won't separate
        // well from median, so we fall back to ABS_THRESHOLD.
        if p85 > median * 3.0 && median > 0.0 {
            // Geometric mean of median and p85, biased toward median to avoid
            // false positives from spectral leakage
            (median * 2.0 + p85) / 4.0
        } else if p85 > ABS_THRESHOLD * 2.0 {
            // Some signal present but distribution is tight — use fraction of p85
            p85 * 0.4
        } else {
            ABS_THRESHOLD
        }
        .max(ABS_THRESHOLD)
    }

    /// Decode data symbols at fixed frame intervals from the sync point.
    ///
    /// Two-pass approach:
    /// 1. Scan all frames, recording detected tones and marking silent slots
    /// 2. Determine data extent from first frame to end chirp (or end of audio)
    /// 3. Assign hi/lo half by position parity; silent slots get nibble value 0
    ///
    /// NOTE: Silent nibble handling diverges from the JS web demo protocol.
    /// The JS real-time decoder skips frames with no detected tones and relies on
    /// timing to re-sync. This offline decoder instead assigns silent frames a
    /// nibble value of 0 based on position parity (even=Hi, odd=Lo), which is
    /// correct for the encoder's output (0x00 nibbles produce silence) but may
    /// differ in behavior for degraded or noisy signals.
    fn decode_symbols_fixed(
        &self,
        samples: &[f32],
        data_start: usize,
        threshold: f32,
        window: &[f32],
        fft: &std::sync::Arc<dyn rustfft::Fft<f32>>,
    ) -> Vec<Symbol> {
        let sr = self.sample_rate as f32;
        let frame_samples = (FRAME_TIME * sr).round() as usize;
        let sym_center_offset = (SYMBOL_DURATION * sr / 2.0).round() as usize;

        // Pass 1: Analyze all frame positions, detect tones and end chirp
        let mut frame_results: Vec<Option<Symbol>> = Vec::new();

        for n in 0..MAX_DECODE_FRAMES {
            let center = data_start + n * frame_samples + sym_center_offset;
            let start = center.saturating_sub(FFT_SIZE / 2);
            if start + FFT_SIZE > samples.len() {
                break;
            }

            let magnitudes =
                self.compute_magnitudes(&samples[start..start + FFT_SIZE], window, fft);
            let hi_band = band_energy(&magnitudes, SYNC_HI_BAND.0, SYNC_HI_BAND.1, sr);

            let mut carrier_mags = [0.0f32; NUM_CARRIERS];
            for i in 0..NUM_CARRIERS {
                carrier_mags[i] = get_bin_mag(&magnitudes, CARRIER_FREQS[i], sr);
            }

            // End chirp detection: broadband hi-band energy without strong carrier tones
            if frame_results.len() > 2 {
                let max_carrier = carrier_mags.iter().copied().fold(0.0f32, f32::max);
                // End chirp produces broadband energy in 1400-1900Hz.
                // A data tone produces narrowband energy at specific carriers.
                // If hi_band is strong but carriers aren't much stronger, it's a chirp.
                if hi_band > threshold && max_carrier < threshold * 1.5 {
                    break;
                }
            }

            frame_results.push(decode_tone_symbol(&carrier_mags, threshold));
        }

        // Pass 2: Find the last frame that has a detected tone.
        // Everything after that is trailing silence / end chirp leakage.
        let last_tone_idx = frame_results
            .iter()
            .rposition(|r| r.is_some())
            .unwrap_or(0);

        // Trim to data extent: from first frame to just past the last detected tone.
        // We need one more frame after the last tone if it's a hi nibble
        // (the lo nibble might be 0).
        let data_end = if last_tone_idx + 1 < frame_results.len() {
            // Include one more frame (could be silent lo nibble of last byte)
            // Only if last_tone_idx is even (hi nibble), meaning lo nibble is next
            if last_tone_idx % 2 == 0 {
                last_tone_idx + 2
            } else {
                last_tone_idx + 1
            }
        } else {
            frame_results.len()
        };

        // Pass 3: Build symbols with position-parity hi/lo assignment
        let mut symbols = Vec::new();
        for (n, result) in frame_results[..data_end].iter().enumerate() {
            match result {
                Some(sym) => symbols.push(*sym),
                None => {
                    // Silent slot = nibble value 0, half determined by position
                    let half = if n % 2 == 0 { Half::Hi } else { Half::Lo };
                    symbols.push(Symbol { half, value: 0 });
                }
            }
        }

        symbols
    }

    /// Run FFT on a windowed frame and return magnitude spectrum.
    fn compute_magnitudes(
        &self,
        frame: &[f32],
        window: &[f32],
        fft: &std::sync::Arc<dyn rustfft::Fft<f32>>,
    ) -> Vec<f32> {
        let mut buffer: Vec<Complex<f32>> = frame
            .iter()
            .zip(window.iter())
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        fft.process(&mut buffer);

        let n = FFT_SIZE / 2;
        let scale = 2.0 / FFT_SIZE as f32;
        buffer[..n]
            .iter()
            .map(|c| c.norm() * scale)
            .collect()
    }
}

impl Default for AcousticDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Hz to FFT bin index.
fn freq_to_bin(freq: f32, sample_rate: f32) -> usize {
    (freq * FFT_SIZE as f32 / sample_rate).round() as usize
}

/// Average magnitude in a frequency band.
fn band_energy(magnitudes: &[f32], lo_hz: f32, hi_hz: f32, sample_rate: f32) -> f32 {
    let a = freq_to_bin(lo_hz, sample_rate);
    let b = freq_to_bin(hi_hz, sample_rate);
    let a = a.min(magnitudes.len().saturating_sub(1));
    let b = b.min(magnitudes.len().saturating_sub(1));
    if b < a {
        return 0.0;
    }
    let sum: f32 = magnitudes[a..=b].iter().sum();
    sum / (b - a + 1) as f32
}

/// Get peak magnitude at a carrier frequency (target bin + neighbors).
fn get_bin_mag(magnitudes: &[f32], freq: f32, sample_rate: f32) -> f32 {
    let bin = freq_to_bin(freq, sample_rate);
    let mut m = magnitudes.get(bin).copied().unwrap_or(0.0);
    if bin > 0 {
        m = m.max(magnitudes.get(bin - 1).copied().unwrap_or(0.0));
    }
    m = m.max(magnitudes.get(bin + 1).copied().unwrap_or(0.0));
    m
}

/// Detect which carriers are active and return a Symbol, or None if silence.
fn decode_tone_symbol(carrier_mags: &[f32; NUM_CARRIERS], threshold: f32) -> Option<Symbol> {
    let mut active: u8 = 0;
    let mut lo_any = false;
    let mut hi_any = false;

    for i in 0..NUM_CARRIERS {
        if carrier_mags[i] > threshold {
            active |= 1 << i;
            if i < 4 {
                lo_any = true;
            } else {
                hi_any = true;
            }
        }
    }

    if !lo_any && !hi_any {
        return None;
    }

    let (half, nibble) = if hi_any && !lo_any {
        let mut n: u8 = 0;
        for b in 0..BITS_PER_NIBBLE {
            if active & (1 << (b + HI_CARRIER_OFFSET)) != 0 {
                n |= 1 << b;
            }
        }
        (Half::Hi, n)
    } else if lo_any && !hi_any {
        let mut n: u8 = 0;
        for b in 0..BITS_PER_NIBBLE {
            if active & (1 << (b + LO_CARRIER_OFFSET)) != 0 {
                n |= 1 << b;
            }
        }
        (Half::Lo, n)
    } else {
        // Both bands active — pick the stronger one
        let lo_strength: f32 = carrier_mags[..4].iter().sum();
        let hi_strength: f32 = carrier_mags[4..].iter().sum();

        if hi_strength > lo_strength {
            let mut n: u8 = 0;
            for b in 0..BITS_PER_NIBBLE {
                if active & (1 << (b + HI_CARRIER_OFFSET)) != 0 {
                    n |= 1 << b;
                }
            }
            (Half::Hi, n)
        } else {
            let mut n: u8 = 0;
            for b in 0..BITS_PER_NIBBLE {
                if active & (1 << (b + LO_CARRIER_OFFSET)) != 0 {
                    n |= 1 << b;
                }
            }
            (Half::Lo, n)
        }
    };

    Some(Symbol {
        half,
        value: nibble,
    })
}

/// Reassemble paired symbols into bytes.
fn reassemble_bytes(symbols: &[Symbol]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut i = 0;

    while i + 1 < symbols.len() {
        let s1 = &symbols[i];
        let s2 = &symbols[i + 1];

        if s1.half == Half::Hi && s2.half == Half::Lo {
            bytes.push((s1.value << 4) | s2.value);
            i += 2;
        } else if s1.half == Half::Lo && s2.half == Half::Hi {
            bytes.push((s2.value << 4) | s1.value);
            i += 2;
        } else {
            i += 1; // skip mismatched symbol
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_to_bin() {
        let sr = 48000.0;
        let bin = freq_to_bin(600.0, sr);
        assert_eq!(bin, 51);
    }

    #[test]
    fn test_reassemble_normal_order() {
        let symbols = vec![
            Symbol { half: Half::Hi, value: 0x4 },
            Symbol { half: Half::Lo, value: 0x2 },
        ];
        let bytes = reassemble_bytes(&symbols);
        assert_eq!(bytes, vec![0x42]);
    }

    #[test]
    fn test_reassemble_reversed_order() {
        let symbols = vec![
            Symbol { half: Half::Lo, value: 0x2 },
            Symbol { half: Half::Hi, value: 0x4 },
        ];
        let bytes = reassemble_bytes(&symbols);
        assert_eq!(bytes, vec![0x42]);
    }

    #[test]
    fn test_reassemble_skip_mismatch() {
        let symbols = vec![
            Symbol { half: Half::Hi, value: 0xA },
            Symbol { half: Half::Hi, value: 0xB },
            Symbol { half: Half::Lo, value: 0x3 },
        ];
        let bytes = reassemble_bytes(&symbols);
        assert_eq!(bytes, vec![0xB3]);
    }
}
