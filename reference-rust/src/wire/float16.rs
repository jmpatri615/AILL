use half::f16;

/// Encode an f32 value as IEEE 754 binary16 (2 bytes, big-endian).
pub fn encode_float16(value: f32) -> [u8; 2] {
    f16::from_f32(value).to_be_bytes()
}

/// Decode 2 big-endian bytes as IEEE 754 binary16, returning f32.
pub fn decode_float16(bytes: [u8; 2]) -> f32 {
    f16::from_be_bytes(bytes).to_f32()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_half() {
        let val = 0.5f32;
        let encoded = encode_float16(val);
        let decoded = decode_float16(encoded);
        assert!((decoded - val).abs() < 0.001);
    }

    #[test]
    fn roundtrip_one() {
        let encoded = encode_float16(1.0);
        let decoded = decode_float16(encoded);
        assert!((decoded - 1.0).abs() < 0.001);
    }
}
