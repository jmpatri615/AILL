use crate::wire::float16::encode_float16;
use crate::wire::varint::encode_varint;

/// A buffer for building AILL wire-format byte sequences.
pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write_u8(&mut self, val: u8) -> &mut Self {
        self.buf.push(val);
        self
    }

    pub fn write_i8(&mut self, val: i8) -> &mut Self {
        self.buf.push(val as u8);
        self
    }

    pub fn write_u16_be(&mut self, val: u16) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_i16_be(&mut self, val: i16) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_u32_be(&mut self, val: u32) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_i32_be(&mut self, val: i32) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_i64_be(&mut self, val: i64) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_u64_be(&mut self, val: u64) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_f16_be(&mut self, val: f32) -> &mut Self {
        self.buf.extend_from_slice(&encode_float16(val));
        self
    }

    pub fn write_f32_be(&mut self, val: f32) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_f64_be(&mut self, val: f64) -> &mut Self {
        self.buf.extend_from_slice(&val.to_be_bytes());
        self
    }

    pub fn write_string(&mut self, val: &str) -> &mut Self {
        let bytes = val.as_bytes();
        self.write_u16_be(bytes.len() as u16);
        self.buf.extend_from_slice(bytes);
        self
    }

    pub fn write_bytes_val(&mut self, val: &[u8]) -> &mut Self {
        self.write_u16_be(val.len() as u16);
        self.buf.extend_from_slice(val);
        self
    }

    pub fn write_uuid(&mut self, val: &[u8; 16]) -> &mut Self {
        self.buf.extend_from_slice(val);
        self
    }

    pub fn write_varint(&mut self, val: u32) -> &mut Self {
        self.buf.extend_from_slice(&encode_varint(val));
        self
    }

    pub fn write_raw(&mut self, data: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(data);
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl Default for ByteWriter {
    fn default() -> Self {
        Self::new()
    }
}
