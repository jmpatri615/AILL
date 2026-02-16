pub mod error;
pub mod wire;
pub mod codebook;
pub mod ast;
pub mod encoder;
pub mod decoder;

#[cfg(feature = "wasm")]
pub mod wasm;

// Re-exports for convenience
pub use error::AILLError;
pub use ast::{AstNode, MetaHeader, LiteralValue, DecodedEpoch};
pub use encoder::{AILLEncoder, EpochBuilder};
pub use decoder::{AILLDecoder, decode_epoch, pretty_print};
pub use wire::{crc8, encode_varint, decode_varint, encode_float16, decode_float16};
pub use codebook::{
    base::{self, BASE_CODEBOOK, CodeEntry},
    DomainCodebook, DomainEntry,
    NAV1, PERCEPT1, DIAG1, PLAN1,
    DOMAIN_REGISTRY, get_domain_codebook,
};
