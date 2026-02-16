use std::path::Path;

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

use crate::error::AILLError;

/// Write mono f32 PCM samples to a WAV file.
pub fn write_wav<P: AsRef<Path>>(
    path: P,
    samples: &[f32],
    sample_rate: u32,
) -> Result<(), AILLError> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)
        .map_err(|e| AILLError::EncoderError(format!("WAV write error: {}", e)))?;

    for &s in samples {
        writer
            .write_sample(s)
            .map_err(|e| AILLError::EncoderError(format!("WAV sample write error: {}", e)))?;
    }

    writer
        .finalize()
        .map_err(|e| AILLError::EncoderError(format!("WAV finalize error: {}", e)))?;

    Ok(())
}

/// Read mono f32 PCM samples from a WAV file.
/// Returns (samples, sample_rate).
pub fn read_wav<P: AsRef<Path>>(path: P) -> Result<(Vec<f32>, u32), AILLError> {
    let reader = WavReader::open(path)
        .map_err(|e| AILLError::InvalidStructure(format!("WAV read error: {}", e)))?;

    let spec = reader.spec();
    let sample_rate = spec.sample_rate;

    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Float => reader
            .into_samples::<f32>()
            .map(|s| s.map_err(|e| AILLError::InvalidStructure(format!("WAV sample error: {}", e))))
            .collect::<Result<Vec<f32>, _>>()?,
        SampleFormat::Int => {
            let max_val = (1u32 << (spec.bits_per_sample - 1)) as f32;
            reader
                .into_samples::<i32>()
                .map(|s| {
                    s.map(|v| v as f32 / max_val)
                        .map_err(|e| AILLError::InvalidStructure(format!("WAV sample error: {}", e)))
                })
                .collect::<Result<Vec<f32>, _>>()?
        }
    };

    Ok((samples, sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_wav_roundtrip() {
        let path = "/tmp/aill_test_wav_roundtrip.wav";
        let samples: Vec<f32> = (0..1000).map(|i| (i as f32 * 0.01).sin()).collect();
        let sr = 48000;

        write_wav(path, &samples, sr).unwrap();
        let (read_samples, read_sr) = read_wav(path).unwrap();

        assert_eq!(read_sr, sr);
        assert_eq!(read_samples.len(), samples.len());
        for (a, b) in samples.iter().zip(read_samples.iter()) {
            assert!((a - b).abs() < 1e-6, "Sample mismatch: {} vs {}", a, b);
        }

        fs::remove_file(path).ok();
    }
}
