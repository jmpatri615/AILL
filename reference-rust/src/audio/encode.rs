use std::f32::consts::PI;

use crate::error::AILLError;

use super::constants::*;

/// Result of acoustic encoding: PCM samples + metadata.
pub struct EncodedAudio {
    /// Mono f32 PCM samples in [-1.0, 1.0].
    pub samples: Vec<f32>,
    /// Sample rate used during synthesis.
    pub sample_rate: u32,
    /// Total duration in seconds.
    pub duration: f32,
}

/// Encodes AILL wire-format bytes into acoustic PCM audio.
pub struct AcousticEncoder {
    sample_rate: u32,
}

impl AcousticEncoder {
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

    /// Encode wire bytes into PCM audio.
    pub fn encode(&self, wire_bytes: &[u8]) -> Result<EncodedAudio, AILLError> {
        if wire_bytes.is_empty() {
            return Err(AILLError::EncoderError("Empty input".into()));
        }
        if wire_bytes.len() > MAX_ENCODE_BYTES {
            return Err(AILLError::EncoderError(format!(
                "Input too large ({} bytes, maximum {})",
                wire_bytes.len(),
                MAX_ENCODE_BYTES
            )));
        }

        let sr = self.sample_rate as f32;
        let duration = SYNC_DURATION + (wire_bytes.len() as f32 * 2.0 * FRAME_TIME) + END_DURATION;
        let total_samples = (duration * sr).ceil() as usize;
        let mut samples = vec![0.0f32; total_samples];

        let mut offset = 0usize;

        // 1. Sync chirp (rising: 300 → 1800 Hz)
        offset = self.write_chirp(
            &mut samples,
            offset,
            SYNC_FREQ_START,
            SYNC_FREQ_END,
            SYNC_DURATION,
        );

        // 2. Data symbols: each byte → hi nibble then lo nibble
        for &byte in wire_bytes {
            let hi = (byte >> 4) & 0x0F;
            let lo = byte & 0x0F;
            offset = self.write_symbol(&mut samples, offset, hi, HI_CARRIER_OFFSET);
            offset = self.write_symbol(&mut samples, offset, lo, LO_CARRIER_OFFSET);
        }

        // 3. End chirp (falling: 1800 → 300 Hz)
        self.write_chirp(
            &mut samples,
            offset,
            END_FREQ_START,
            END_FREQ_END,
            END_DURATION,
        );

        Ok(EncodedAudio {
            samples,
            sample_rate: self.sample_rate,
            duration,
        })
    }

    /// Write a linear frequency sweep (chirp) with linear attack/release envelope.
    /// Returns the sample offset after the chirp.
    fn write_chirp(
        &self,
        samples: &mut [f32],
        start: usize,
        f0: f32,
        f1: f32,
        duration: f32,
    ) -> usize {
        let sr = self.sample_rate as f32;
        let num_samples = (duration * sr).round() as usize;
        let attack_samples = ((CHIRP_ATTACK * sr).round() as usize).max(1);
        let release_samples = ((CHIRP_RELEASE * sr).round() as usize).max(1);

        for i in 0..num_samples {
            if start + i >= samples.len() {
                break;
            }
            let t = i as f32 / sr;

            // Phase-correct linear chirp: φ(t) = 2π(f₀t + (f₁-f₀)t²/(2d))
            let phase = 2.0 * PI * (f0 * t + (f1 - f0) * t * t / (2.0 * duration));
            let signal = phase.sin();

            // Envelope: linear attack/release
            let env = if i < attack_samples {
                i as f32 / attack_samples as f32
            } else if i >= num_samples - release_samples {
                (num_samples - 1 - i) as f32 / release_samples as f32
            } else {
                1.0
            };

            samples[start + i] += signal * env * MASTER_GAIN;
        }

        start + num_samples
    }

    /// Write a data symbol: activate carriers for set bits in the nibble.
    /// `carrier_offset` is 0 for lo-nibble (600-900Hz) or 4 for hi-nibble (1000-1300Hz).
    /// Returns the sample offset after the full frame (symbol + guard).
    fn write_symbol(
        &self,
        samples: &mut [f32],
        start: usize,
        nibble: u8,
        carrier_offset: usize,
    ) -> usize {
        let sr = self.sample_rate as f32;
        let sym_samples = (SYMBOL_DURATION * sr).round() as usize;
        let frame_samples = (FRAME_TIME * sr).round() as usize;
        let attack_samples = ((TONE_ATTACK * sr).round() as usize).max(1);
        let release_samples = ((TONE_RELEASE * sr).round() as usize).max(1);

        for bit in 0..BITS_PER_NIBBLE {
            if nibble & (1 << bit) == 0 {
                continue;
            }
            let freq = CARRIER_FREQS[carrier_offset + bit];

            for i in 0..sym_samples {
                if start + i >= samples.len() {
                    break;
                }
                let t = i as f32 / sr;
                let signal = (2.0 * PI * freq * t).sin();

                // Envelope: 3ms attack to 0.8, hold, 3ms release
                let env = if i < attack_samples {
                    TONE_AMPLITUDE * (i as f32 / attack_samples as f32)
                } else if i >= sym_samples - release_samples {
                    TONE_AMPLITUDE * ((sym_samples - 1 - i) as f32 / release_samples as f32)
                } else {
                    TONE_AMPLITUDE
                };

                samples[start + i] += signal * env * MASTER_GAIN;
            }
        }

        start + frame_samples
    }
}

impl Default for AcousticEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_produces_correct_duration() {
        let encoder = AcousticEncoder::new();
        let data = vec![0x42, 0x13];
        let audio = encoder.encode(&data).unwrap();

        let expected = SYNC_DURATION + (2.0 * 2.0 * FRAME_TIME) + END_DURATION;
        assert!((audio.duration - expected).abs() < 1e-6);
        assert_eq!(audio.sample_rate, DEFAULT_SAMPLE_RATE);
    }

    #[test]
    fn test_encode_empty_fails() {
        let encoder = AcousticEncoder::new();
        assert!(encoder.encode(&[]).is_err());
    }

    #[test]
    fn test_samples_within_range() {
        let encoder = AcousticEncoder::new();
        let audio = encoder.encode(&[0xFF]).unwrap();
        for &s in &audio.samples {
            assert!(
                s >= -1.0 && s <= 1.0,
                "Sample out of range: {}",
                s
            );
        }
    }

    #[test]
    fn test_silent_nibble() {
        // Nibble 0x00 should produce silence in its carrier band
        let encoder = AcousticEncoder::new();
        let audio = encoder.encode(&[0x00]).unwrap();
        // After sync chirp, the data region should be near-silent
        let sr = DEFAULT_SAMPLE_RATE as f32;
        let sync_end = (SYNC_DURATION * sr).round() as usize;
        let data_end = sync_end + (2.0 * FRAME_TIME * sr).round() as usize;
        let data_region = &audio.samples[sync_end..data_end.min(audio.samples.len())];
        let max_abs: f32 = data_region.iter().map(|s| s.abs()).fold(0.0, f32::max);
        assert!(max_abs < 0.01, "Expected near-silence for 0x00, got max={}", max_abs);
    }
}
