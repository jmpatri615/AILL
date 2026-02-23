use std::env;
use std::process;

use aill::audio::{AcousticDecoder, AcousticEncoder};
use aill::audio::constants::DEFAULT_SAMPLE_RATE;
use aill::audio::live;

/// Maximum recording duration the CLI will accept (seconds).
const MAX_RECORD_DURATION_SECS: f32 = 60.0;

/// Extra recording time beyond the encoded audio duration for roundtrip (seconds).
const ROUNDTRIP_LATENCY_MARGIN_SECS: f32 = 1.0;

/// Delay before playback starts in roundtrip mode, letting the recording
/// stream fully initialize (milliseconds).
const RECORDING_INIT_DELAY_MS: u64 = 200;

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  aill-live tx <hex-bytes>       Encode hex data and play through speaker");
    eprintln!("  aill-live rx <seconds>         Record from mic, decode, and print hex");
    eprintln!("  aill-live roundtrip <hex>      Transmit then receive, verify match");
    process::exit(1);
}

fn parse_hex(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    if s.len() % 2 != 0 {
        return Err(format!("Hex string must have even length, got {}", s.len()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex at position {}: {}", i, e))
        })
        .collect()
}

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

fn cmd_tx(hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wire_bytes = parse_hex(hex)?;
    println!("Encoding {} bytes: {}", wire_bytes.len(), hex_string(&wire_bytes));

    let encoder = AcousticEncoder::new();
    let encoded = encoder.encode(&wire_bytes)?;
    println!(
        "Audio: {} samples, {:.2}s at {} Hz",
        encoded.samples.len(),
        encoded.duration,
        encoded.sample_rate
    );

    println!("Playing...");
    live::play_audio(&encoded.samples, encoded.sample_rate)?;
    println!("Done.");
    Ok(())
}

fn cmd_rx(seconds_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let seconds: f32 = seconds_str
        .parse()
        .map_err(|e| format!("Invalid duration '{}': {}", seconds_str, e))?;
    if seconds <= 0.0 || seconds > MAX_RECORD_DURATION_SECS {
        return Err(format!(
            "Duration must be greater than 0 and at most {} seconds",
            MAX_RECORD_DURATION_SECS
        )
        .into());
    }

    println!("Recording {:.1}s at {} Hz...", seconds, DEFAULT_SAMPLE_RATE);
    let samples = live::record_audio(seconds, DEFAULT_SAMPLE_RATE)?;
    println!("Captured {} samples.", samples.len());

    println!("Decoding...");
    let decoder = AcousticDecoder::new();
    let bytes = decoder.decode(&samples)?;
    println!("Decoded {} bytes: {}", bytes.len(), hex_string(&bytes));
    Ok(())
}

fn cmd_roundtrip(hex: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wire_bytes = parse_hex(hex)?;
    println!("Roundtrip test: {} bytes: {}", wire_bytes.len(), hex_string(&wire_bytes));

    let encoder = AcousticEncoder::new();
    let encoded = encoder.encode(&wire_bytes)?;

    // Calculate recording duration: audio duration + margin for latency
    let rx_duration = encoded.duration + ROUNDTRIP_LATENCY_MARGIN_SECS;

    // Start recording in a background thread before playing
    let rx_handle = std::thread::spawn(move || {
        live::record_audio(rx_duration, DEFAULT_SAMPLE_RATE)
    });

    // Small delay to let the recording stream initialize
    std::thread::sleep(std::time::Duration::from_millis(RECORDING_INIT_DELAY_MS));

    println!("Playing...");
    live::play_audio(&encoded.samples, encoded.sample_rate)?;
    println!("Playback done, waiting for recording...");

    let samples = rx_handle
        .join()
        .map_err(|_| "Recording thread panicked")?
        .map_err(|e| format!("Recording failed: {}", e))?;
    println!("Captured {} samples.", samples.len());

    println!("Decoding...");
    let decoder = AcousticDecoder::new();
    let decoded = decoder.decode(&samples)?;
    println!("Decoded {} bytes: {}", decoded.len(), hex_string(&decoded));
    if decoded == wire_bytes {
        println!("PASS: roundtrip matched!");
    } else {
        return Err(format!(
            "FAIL: expected {}, got {}",
            hex_string(&wire_bytes),
            hex_string(&decoded)
        )
        .into());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
    }

    let result = match args[1].as_str() {
        "tx" => cmd_tx(&args[2]),
        "rx" => cmd_rx(&args[2]),
        "roundtrip" => cmd_roundtrip(&args[2]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            usage();
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
