use crate::error::AILLError;
use crate::wire::float16::decode_float16;
use crate::wire::varint::decode_varint;

/// A cursor for reading AILL wire-format bytes.
pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn peek(&self) -> Result<u8, AILLError> {
        if self.pos >= self.data.len() {
            return Err(AILLError::UnexpectedEof {
                offset: self.pos,
                needed: 1,
            });
        }
        Ok(self.data[self.pos])
    }

    pub fn read_u8(&mut self) -> Result<u8, AILLError> {
        if self.pos >= self.data.len() {
            return Err(AILLError::UnexpectedEof {
                offset: self.pos,
                needed: 1,
            });
        }
        let val = self.data[self.pos];
        self.pos += 1;
        Ok(val)
    }

    pub fn read_i8(&mut self) -> Result<i8, AILLError> {
        Ok(self.read_u8()? as i8)
    }

    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], AILLError> {
        if self.pos + n > self.data.len() {
            return Err(AILLError::UnexpectedEof {
                offset: self.pos,
                needed: n,
            });
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    pub fn read_u16_be(&mut self) -> Result<u16, AILLError> {
        let bytes = self.read_bytes(2)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_i16_be(&mut self) -> Result<i16, AILLError> {
        let bytes = self.read_bytes(2)?;
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_u32_be(&mut self) -> Result<u32, AILLError> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_i32_be(&mut self) -> Result<i32, AILLError> {
        let bytes = self.read_bytes(4)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_i64_be(&mut self) -> Result<i64, AILLError> {
        let bytes = self.read_bytes(8)?;
        Ok(i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn read_u64_be(&mut self) -> Result<u64, AILLError> {
        let bytes = self.read_bytes(8)?;
        Ok(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn read_f16_be(&mut self) -> Result<f32, AILLError> {
        let bytes = self.read_bytes(2)?;
        Ok(decode_float16([bytes[0], bytes[1]]))
    }

    pub fn read_f32_be(&mut self) -> Result<f32, AILLError> {
        let bytes = self.read_bytes(4)?;
        Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_f64_be(&mut self) -> Result<f64, AILLError> {
        let bytes = self.read_bytes(8)?;
        Ok(f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn read_string(&mut self) -> Result<String, AILLError> {
        let length = self.read_u16_be()? as usize;
        let bytes = self.read_bytes(length)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|e| AILLError::Utf8Error(e.to_string()))
    }

    pub fn read_uuid(&mut self) -> Result<[u8; 16], AILLError> {
        let bytes = self.read_bytes(16)?;
        let mut uuid = [0u8; 16];
        uuid.copy_from_slice(bytes);
        Ok(uuid)
    }

    pub fn read_varint(&mut self) -> Result<u32, AILLError> {
        let (val, consumed) = decode_varint(self.data, self.pos)?;
        self.pos += consumed;
        Ok(val)
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, AILLError> {
        let bytes = self.read_bytes(n)?;
        Ok(bytes.to_vec())
    }
}
