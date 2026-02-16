/// CRC-8/CCITT lookup table (polynomial 0x07).
const CRC8_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u8;
        let mut bit = 0;
        while bit < 8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// Compute CRC-8/CCITT over a byte slice.
pub fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &b in data {
        crc = CRC8_TABLE[(crc ^ b) as usize];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc8_empty() {
        assert_eq!(crc8(b""), 0x00);
    }

    #[test]
    fn crc8_standard_vector() {
        assert_eq!(crc8(b"123456789"), 0xF4);
    }

    #[test]
    fn crc8_single_byte() {
        let result = crc8(&[0x00]);
        assert_eq!(result, CRC8_TABLE[0]);
    }
}
