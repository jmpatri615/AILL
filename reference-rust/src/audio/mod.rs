pub mod constants;
pub mod decode;
pub mod encode;

#[cfg(feature = "audio")]
pub mod wav;

#[cfg(all(feature = "audio-live", not(target_family = "wasm")))]
pub mod live;

pub use constants::*;
pub use decode::AcousticDecoder;
pub use encode::{AcousticEncoder, EncodedAudio};

#[cfg(feature = "audio")]
pub use wav::{read_wav, write_wav};

#[cfg(all(feature = "audio-live", not(target_family = "wasm")))]
pub use live::{play_audio, record_audio};
