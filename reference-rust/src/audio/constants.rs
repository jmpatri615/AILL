/// Acoustic protocol constants matching the JS web demo exactly.

// ── Carrier frequencies ──

/// Base frequency for the lowest carrier (Hz).
pub const BASE_FREQ: f32 = 600.0;

/// Spacing between adjacent carriers (Hz).
pub const TONE_SPACING: f32 = 100.0;

/// Number of active carriers (4 lo + 4 hi).
pub const NUM_CARRIERS: usize = 8;

/// Carrier frequencies: [600, 700, 800, 900, 1000, 1100, 1200, 1300] Hz.
pub const CARRIER_FREQS: [f32; NUM_CARRIERS] = [
    600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0, 1300.0,
];

/// Lo-nibble carriers (indices 0..3): 600-900 Hz.
pub const LO_CARRIER_OFFSET: usize = 0;

/// Hi-nibble carriers (indices 4..7): 1000-1300 Hz.
pub const HI_CARRIER_OFFSET: usize = 4;

/// Bits per nibble.
pub const BITS_PER_NIBBLE: usize = 4;

// ── Timing ──

/// Duration of each data tone (seconds).
pub const SYMBOL_DURATION: f32 = 0.05;

/// Silence between symbols (seconds).
pub const GUARD_TIME: f32 = 0.01;

/// Total frame time per symbol: SYMBOL_DURATION + GUARD_TIME (seconds).
pub const FRAME_TIME: f32 = 0.06;

// ── Sync chirp (rising: 300→1800 Hz) ──

pub const SYNC_FREQ_START: f32 = 300.0;
pub const SYNC_FREQ_END: f32 = 1800.0;
pub const SYNC_DURATION: f32 = 0.15;

// ── End chirp (falling: 1800→300 Hz) ──

pub const END_FREQ_START: f32 = 1800.0;
pub const END_FREQ_END: f32 = 300.0;
pub const END_DURATION: f32 = 0.10;

// ── Chirp envelope ──

pub const CHIRP_ATTACK: f32 = 0.01;
pub const CHIRP_RELEASE: f32 = 0.01;

// ── Data tone envelope ──

pub const TONE_ATTACK: f32 = 0.003;
pub const TONE_AMPLITUDE: f32 = 0.8;
pub const TONE_RELEASE: f32 = 0.003;

/// Master gain applied to data tones.
pub const MASTER_GAIN: f32 = 0.15;

// ── FFT / decoder ──

pub const FFT_SIZE: usize = 4096;

/// Absolute minimum threshold for raw linear FFT magnitudes.
/// JS uses 70 on a 0-255 dB scale; for linear magnitudes with our
/// signal levels (peak ~0.12), 0.005 cleanly separates signal from silence.
pub const ABS_THRESHOLD: f32 = 0.005;

/// Multiplier applied to noise floor to derive dynamic threshold.
pub const TONE_THRESHOLD_RATIO: f32 = 2.0;

/// Default sample rate.
pub const DEFAULT_SAMPLE_RATE: u32 = 48000;

/// Minimum supported sample rate. Below this, carrier frequencies cannot
/// be represented (Nyquist must exceed the highest carrier + margin).
pub const MIN_SAMPLE_RATE: u32 = 4000;

/// Maximum number of symbol frames the decoder will scan before stopping.
/// Each byte produces 2 frames (hi + lo nibble), so this allows up to
/// MAX_DECODE_FRAMES / 2 = 500 bytes.
pub const MAX_DECODE_FRAMES: usize = 1000;

// ── Decoder sync detection bands ──

pub const SYNC_LO_BAND: (f32, f32) = (250.0, 550.0);
pub const SYNC_HI_BAND: (f32, f32) = (1400.0, 1900.0);

/// Noise floor estimation band (outside signal band).
pub const NOISE_BAND: (f32, f32) = (2500.0, 4000.0);

/// IIR smoothing for noise floor update.
pub const NOISE_SMOOTH: f32 = 0.93;

/// Minimum elapsed ms before accepting sync rising→high transition.
pub const SYNC_MIN_ELAPSED_MS: f32 = 60.0;

/// Maximum elapsed ms before timing out sync detection.
pub const SYNC_MAX_ELAPSED_MS: f32 = 400.0;

/// Wait after sync chirp high-band detection before starting data decode.
pub const SYNC_WAIT_MS: f32 = 40.0;

/// Fraction of FRAME_TIME to wait before sampling a symbol.
pub const SYMBOL_SAMPLE_FRACTION: f32 = 0.75;

/// Maximum silence in RECEIVING before auto-finish (ms).
pub const MAX_SILENCE_MS: f32 = 250.0;

/// Minimum symbols for a valid reception.
pub const MIN_SYMBOLS: usize = 4;
