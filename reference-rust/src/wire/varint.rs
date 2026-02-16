use crate::error::AILLError;

/// Encode a non-negative integer as a variable-length integer.
///
/// Encoding scheme:
/// - 0-127: 1 byte (0x00-0x7F)
/// - 128-16383: 2 bytes (0x80 | high bits, low byte)
/// - 16384-2097151: 3 bytes (0xC0 | high bits, mid byte, low byte)
/// - 2097152-268435455: 4 bytes (0xE0 | high bits, ...)
/// - 268435456+: 5 bytes (0xF0 + 4-byte big-endian u32)
pub fn encode_varint(value: u32) -> Vec<u8> {
    if value < 128 {
        vec![value as u8]
    } else if value < 16384 {
        vec![0x80 | (value >> 8) as u8, (value & 0xFF) as u8]
    } else if value < 2_097_152 {
        vec![
            0xC0 | (value >> 16) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    } else if value < 268_435_456 {
        vec![
            0xE0 | (value >> 24) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    } else {
        let mut buf = vec![0xF0];
        buf.extend_from_slice(&value.to_be_bytes());
        buf
    }
}

/// Decode a variable-length integer from a byte slice at the given offset.
/// Returns (value, bytes_consumed).
pub fn decode_varint(data: &[u8], offset: usize) -> Result<(u32, usize), AILLError> {
    if offset >= data.len() {
        return Err(AILLError::UnexpectedEof { offset, needed: 1 });
    }
    let first = data[offset];

    if first < 0x80 {
        Ok((first as u32, 1))
    } else if first < 0xC0 {
        if offset + 1 >= data.len() {
            return Err(AILLError::UnexpectedEof { offset, needed: 2 });
        }
        let val = ((first as u32 & 0x3F) << 8) | data[offset + 1] as u32;
        Ok((val, 2))
    } else if first < 0xE0 {
        if offset + 2 >= data.len() {
            return Err(AILLError::UnexpectedEof { offset, needed: 3 });
        }
        let val = ((first as u32 & 0x1F) << 16)
            | ((data[offset + 1] as u32) << 8)
            | data[offset + 2] as u32;
        Ok((val, 3))
    } else if first < 0xF0 {
        if offset + 3 >= data.len() {
            return Err(AILLError::UnexpectedEof { offset, needed: 4 });
        }
        let val = ((first as u32 & 0x0F) << 24)
            | ((data[offset + 1] as u32) << 16)
            | ((data[offset + 2] as u32) << 8)
            | data[offset + 3] as u32;
        Ok((val, 4))
    } else {
        if offset + 4 >= data.len() {
            return Err(AILLError::UnexpectedEof { offset, needed: 5 });
        }
        let val = u32::from_be_bytes([
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
        ]);
        Ok((val, 5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_1byte() {
        for v in [0, 1, 63, 127] {
            let encoded = encode_varint(v);
            assert_eq!(encoded.len(), 1);
            let (decoded, consumed) = decode_varint(&encoded, 0).unwrap();
            assert_eq!(decoded, v);
            assert_eq!(consumed, 1);
        }
    }

    #[test]
    fn roundtrip_2byte() {
        for v in [128, 1000, 16383] {
            let encoded = encode_varint(v);
            assert_eq!(encoded.len(), 2);
            let (decoded, consumed) = decode_varint(&encoded, 0).unwrap();
            assert_eq!(decoded, v);
            assert_eq!(consumed, 2);
        }
    }

    #[test]
    fn roundtrip_large() {
        for v in [16384, 100_000, 2_097_151, 268_435_455] {
            let encoded = encode_varint(v);
            let (decoded, _consumed) = decode_varint(&encoded, 0).unwrap();
            assert_eq!(decoded, v);
        }
    }

    #[test]
    fn roundtrip_5byte() {
        let v = 268_435_456;
        let encoded = encode_varint(v);
        assert_eq!(encoded.len(), 5);
        let (decoded, consumed) = decode_varint(&encoded, 0).unwrap();
        assert_eq!(decoded, v);
        assert_eq!(consumed, 5);
    }
}
