use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::error::AILLError;

/// Polling interval (ms) while waiting for playback to finish.
const POLL_INTERVAL_MS: u64 = 10;

/// Delay (ms) after playback finishes to let the audio device flush its buffer.
const DRAIN_DELAY_MS: u64 = 50;

/// Maximum recording duration (seconds) to prevent runaway allocations.
const MAX_RECORD_DURATION_SECS: f32 = 300.0;

/// Build a mono f32 stream config at the given sample rate.
fn stream_config(sample_rate: u32) -> cpal::StreamConfig {
    cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(sample_rate),
        buffer_size: cpal::BufferSize::Default,
    }
}

/// Lock a mutex, recovering from poisoning rather than panicking.
fn lock_or_recover<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

/// Play mono f32 PCM samples through the default output device.
///
/// Blocks until all samples have been played, then drops the stream.
/// Returns an error if no output device is available or the stream fails.
pub fn play_audio(samples: &[f32], sample_rate: u32) -> Result<(), AILLError> {
    if samples.is_empty() {
        return Err(AILLError::EncoderError("No audio samples to play".into()));
    }
    if sample_rate == 0 {
        return Err(AILLError::EncoderError("Sample rate must be > 0".into()));
    }

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| AILLError::EncoderError("No output audio device available".into()))?;

    let config = stream_config(sample_rate);

    let data = Arc::new(samples.to_vec());
    let cursor = Arc::new(AtomicUsize::new(0));
    let finished = Arc::new(AtomicBool::new(false));
    let error_flag: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let data_cb = Arc::clone(&data);
    let cursor_cb = Arc::clone(&cursor);
    let finished_cb = Arc::clone(&finished);
    let error_cb = Arc::clone(&error_flag);

    let stream = device
        .build_output_stream(
            &config,
            move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let len = data_cb.len();
                for sample in output.iter_mut() {
                    let pos = cursor_cb.fetch_add(1, Ordering::Relaxed);
                    if pos < len {
                        *sample = data_cb[pos];
                    } else {
                        *sample = 0.0;
                        finished_cb.store(true, Ordering::Release);
                    }
                }
            },
            move |err| {
                let mut guard = lock_or_recover(&error_cb);
                *guard = Some(format!("Output stream error: {}", err));
            },
            None,
        )
        .map_err(|e| AILLError::EncoderError(format!("Failed to build output stream: {}", e)))?;

    stream
        .play()
        .map_err(|e| AILLError::EncoderError(format!("Failed to play stream: {}", e)))?;

    // Poll until all samples have been consumed
    while !finished.load(Ordering::Acquire) {
        std::thread::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS));

        // Check for stream errors
        if let Some(err) = lock_or_recover(&error_flag).take() {
            return Err(AILLError::EncoderError(err));
        }
    }

    // Brief drain to let the audio device flush its buffer
    std::thread::sleep(std::time::Duration::from_millis(DRAIN_DELAY_MS));

    drop(stream);
    Ok(())
}

/// Record mono f32 PCM samples from the default input device.
///
/// Records for `duration_secs` seconds at the given sample rate,
/// then returns the captured buffer. `duration_secs` must be positive
/// and at most 300 seconds. Returns an error if no input device is
/// available or the stream fails.
pub fn record_audio(duration_secs: f32, sample_rate: u32) -> Result<Vec<f32>, AILLError> {
    if sample_rate == 0 {
        return Err(AILLError::EncoderError("Sample rate must be > 0".into()));
    }
    if duration_secs <= 0.0 || duration_secs > MAX_RECORD_DURATION_SECS {
        return Err(AILLError::EncoderError(format!(
            "Recording duration must be between 0 and {} seconds",
            MAX_RECORD_DURATION_SECS
        )));
    }

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| AILLError::EncoderError("No input audio device available".into()))?;

    let config = stream_config(sample_rate);

    let capacity = (duration_secs * sample_rate as f32).ceil() as usize;
    let buffer = Arc::new(Mutex::new(Vec::<f32>::with_capacity(capacity)));
    let error_flag: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let buffer_cb = Arc::clone(&buffer);
    let error_cb = Arc::clone(&error_flag);

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = lock_or_recover(&buffer_cb);
                buf.extend_from_slice(data);
            },
            move |err| {
                let mut guard = lock_or_recover(&error_cb);
                *guard = Some(format!("Input stream error: {}", err));
            },
            None,
        )
        .map_err(|e| AILLError::EncoderError(format!("Failed to build input stream: {}", e)))?;

    stream
        .play()
        .map_err(|e| AILLError::EncoderError(format!("Failed to start recording: {}", e)))?;

    let total_ms = (duration_secs * 1000.0) as u64;
    std::thread::sleep(std::time::Duration::from_millis(total_ms));

    // Check for stream errors
    if let Some(err) = lock_or_recover(&error_flag).take() {
        return Err(AILLError::EncoderError(err));
    }

    drop(stream);

    let samples = std::mem::take(&mut *lock_or_recover(&buffer));
    Ok(samples)
}
