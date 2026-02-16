#![cfg(feature = "audio")]

use aill::audio::{
    AcousticDecoder, AcousticEncoder,
    constants::*,
    read_wav, write_wav,
};
use aill::{AILLEncoder, EpochBuilder};

/// Helper: encode wire bytes → PCM → decode back to wire bytes.
fn roundtrip(wire_bytes: &[u8]) -> Vec<u8> {
    let encoder = AcousticEncoder::new();
    let audio = encoder.encode(wire_bytes).unwrap();
    let decoder = AcousticDecoder::new();
    decoder.decode(&audio.samples).unwrap()
}

/// Helper: encode wire bytes → PCM → decode back, at a given sample rate.
fn roundtrip_at(wire_bytes: &[u8], sample_rate: u32) -> Vec<u8> {
    let encoder = AcousticEncoder::with_sample_rate(sample_rate);
    let audio = encoder.encode(wire_bytes).unwrap();
    let decoder = AcousticDecoder::with_sample_rate(sample_rate);
    decoder.decode(&audio.samples).unwrap()
}

#[test]
fn test_simple_message_roundtrip() {
    let original = vec![0x42, 0x13, 0xAB, 0xFF, 0x01];
    let recovered = roundtrip(&original);
    assert_eq!(
        recovered, original,
        "Simple round-trip failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        original, recovered
    );
}

#[test]
fn test_all_nibbles_stress() {
    let original: Vec<u8> = (0x10..=0x1F).collect();
    let recovered = roundtrip(&original);
    assert_eq!(
        recovered, original,
        "All-nibbles stress test failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        original, recovered
    );
}

#[test]
fn test_high_nibble_variety() {
    let original: Vec<u8> = vec![
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ];
    let recovered = roundtrip(&original);
    assert_eq!(
        recovered, original,
        "High nibble variety failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        original, recovered
    );
}

#[test]
fn test_44100hz_sample_rate() {
    let original = vec![0x42, 0x13, 0xAB];
    let recovered = roundtrip_at(&original, 44100);
    assert_eq!(
        recovered, original,
        "44100 Hz round-trip failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        original, recovered
    );
}

#[test]
fn test_wav_file_roundtrip() {
    let path = "/tmp/aill_audio_roundtrip_test.wav";
    let original = vec![0x42, 0x13, 0xAB, 0xFF, 0x01];

    let encoder = AcousticEncoder::new();
    let audio = encoder.encode(&original).unwrap();

    write_wav(path, &audio.samples, audio.sample_rate).unwrap();
    let (samples, sr) = read_wav(path).unwrap();
    assert_eq!(sr, audio.sample_rate);

    let decoder = AcousticDecoder::new();
    let recovered = decoder.decode(&samples).unwrap();
    assert_eq!(
        recovered, original,
        "WAV round-trip failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        original, recovered
    );

    std::fs::remove_file(path).ok();
}

#[test]
fn test_duration_formula() {
    let encoder = AcousticEncoder::new();
    for len in [1, 5, 10, 50] {
        let data: Vec<u8> = (0..len).map(|i| (i & 0xFF) as u8).collect();
        let audio = encoder.encode(&data).unwrap();
        let expected = SYNC_DURATION + (len as f32 * 2.0 * FRAME_TIME) + END_DURATION;
        assert!(
            (audio.duration - expected).abs() < 1e-5,
            "Duration mismatch for len={}: got {} expected {}",
            len, audio.duration, expected
        );
    }
}

#[test]
fn test_epoch_wrapped_roundtrip() {
    let mut enc = AILLEncoder::new();
    enc.start_utterance()
        .assert_()
        .string("hello")
        .end_utterance();

    let wire = enc.end_utterance();
    let mut eb = EpochBuilder::new();
    eb.write(&wire);
    let epochs = eb.get_epochs();
    let epoch_bytes = &epochs[0];

    let recovered = roundtrip(epoch_bytes);
    assert_eq!(
        recovered, *epoch_bytes,
        "Epoch round-trip failed:\n  original:  {:02X?}\n  recovered: {:02X?}",
        epoch_bytes, recovered
    );
}
