use std::fmt;

/// Errors that can occur during AILL encoding/decoding.
#[derive(Debug, Clone, PartialEq)]
pub enum AILLError {
    InvalidOpCode(u8),
    CrcMismatch { expected: u8, actual: u8 },
    UnexpectedEof { offset: usize, needed: usize },
    InvalidStructure(String),
    InvalidVarInt,
    Utf8Error(String),
    EncoderError(String),
}

impl fmt::Display for AILLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AILLError::InvalidOpCode(code) => write!(f, "Invalid opcode: 0x{:02X}", code),
            AILLError::CrcMismatch { expected, actual } => {
                write!(f, "CRC mismatch: expected 0x{:02X}, got 0x{:02X}", expected, actual)
            }
            AILLError::UnexpectedEof { offset, needed } => {
                write!(f, "[offset {}] Unexpected end of data, need {} more bytes", offset, needed)
            }
            AILLError::InvalidStructure(msg) => write!(f, "Invalid structure: {}", msg),
            AILLError::InvalidVarInt => write!(f, "Invalid variable-length integer"),
            AILLError::Utf8Error(msg) => write!(f, "UTF-8 error: {}", msg),
            AILLError::EncoderError(msg) => write!(f, "Encoder error: {}", msg),
        }
    }
}

impl std::error::Error for AILLError {}
