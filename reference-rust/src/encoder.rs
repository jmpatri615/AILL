use crate::codebook::base::{fc, ty, st, modal, pragma, meta, arith, rel, quant, esc};
use crate::wire::ByteWriter;
use crate::wire::crc8::crc8;

/// Maximum payload size per epoch.
pub const MAX_EPOCH_PAYLOAD: usize = 8192;

/// Fluent builder for encoding AILL utterances into wire format bytes.
pub struct AILLEncoder {
    stream: ByteWriter,
    _uuid: [u8; 16],
    in_utterance: bool,
}

impl AILLEncoder {
    pub fn new() -> Self {
        Self {
            stream: ByteWriter::new(),
            _uuid: [0u8; 16],
            in_utterance: false,
        }
    }

    pub fn with_uuid(uuid: [u8; 16]) -> Self {
        Self {
            stream: ByteWriter::new(),
            _uuid: uuid,
            in_utterance: false,
        }
    }

    fn code(&mut self, code: u8) -> &mut Self {
        self.stream.write_u8(code);
        self
    }

    // ── Utterance framing ──

    pub fn start_utterance(&mut self) -> &mut Self {
        self.start_utterance_with(1.0, 3, None, None, None)
    }

    pub fn start_utterance_with(
        &mut self,
        confidence: f32,
        priority: u8,
        timestamp_us: Option<i64>,
        dest_agent: Option<&[u8; 16]>,
        seqnum: Option<u32>,
    ) -> &mut Self {
        let ts = timestamp_us.unwrap_or(0);

        self.code(fc::START_UTTERANCE);

        // Mandatory meta header: CONFIDENCE, PRIORITY, TIMESTAMP
        self.code(meta::CONFIDENCE);
        self.stream.write_f16_be(confidence);
        self.code(meta::PRIORITY);
        self.stream.write_u8(priority);
        self.code(meta::TIMESTAMP_META);
        self.stream.write_i64_be(ts);

        // Optional meta fields
        if let Some(dest) = dest_agent {
            self.code(meta::DEST_AGENT);
            self.stream.write_uuid(dest);
        }
        if let Some(seq) = seqnum {
            self.code(meta::SEQNUM);
            self.stream.write_u32_be(seq);
        }

        self.in_utterance = true;
        self
    }

    pub fn end_utterance(&mut self) -> Vec<u8> {
        self.code(fc::END_UTTERANCE);
        self.in_utterance = false;
        self.stream.to_bytes()
    }

    // ── Pragmatic acts ──

    pub fn pragma(&mut self, act: u8) -> &mut Self {
        self.code(act)
    }

    pub fn query(&mut self) -> &mut Self { self.code(pragma::QUERY) }
    pub fn assert_(&mut self) -> &mut Self { self.code(pragma::ASSERT) }
    pub fn request(&mut self) -> &mut Self { self.code(pragma::REQUEST) }
    pub fn command(&mut self) -> &mut Self { self.code(pragma::COMMAND) }
    pub fn acknowledge(&mut self) -> &mut Self { self.code(pragma::ACKNOWLEDGE) }
    pub fn warn(&mut self) -> &mut Self { self.code(pragma::WARN) }

    // ── Modality ──

    pub fn modality(&mut self, m: u8) -> &mut Self {
        self.code(m)
    }

    pub fn observed(&mut self) -> &mut Self { self.code(modal::OBSERVED) }
    pub fn inferred(&mut self) -> &mut Self { self.code(modal::INFERRED) }

    pub fn predicted(&mut self, horizon_ms: f32) -> &mut Self {
        self.code(modal::PREDICTED);
        self.stream.write_f16_be(horizon_ms);
        self
    }

    // ── Temporal ──

    pub fn temporal(&mut self, t: u8) -> &mut Self {
        self.code(t)
    }

    // ── Structure ──

    pub fn begin_struct(&mut self) -> &mut Self { self.code(st::BEGIN_STRUCT) }
    pub fn end_struct(&mut self) -> &mut Self { self.code(st::END_STRUCT) }

    pub fn field(&mut self, field_code: u16) -> &mut Self {
        self.code(st::FIELD_ID);
        self.stream.write_u16_be(field_code);
        self
    }

    pub fn begin_list(&mut self, count: u16) -> &mut Self {
        self.code(st::BEGIN_LIST);
        self.stream.write_u16_be(count);
        self
    }

    pub fn end_list(&mut self) -> &mut Self { self.code(st::END_LIST) }

    pub fn begin_map(&mut self, count: u16) -> &mut Self {
        self.code(st::BEGIN_MAP);
        self.stream.write_u16_be(count);
        self
    }

    pub fn end_map(&mut self) -> &mut Self { self.code(st::END_MAP) }

    // ── Typed values ──

    pub fn int8(&mut self, val: i8) -> &mut Self {
        self.code(ty::TYPE_INT8);
        self.stream.write_i8(val);
        self
    }

    pub fn int16(&mut self, val: i16) -> &mut Self {
        self.code(ty::TYPE_INT16);
        self.stream.write_i16_be(val);
        self
    }

    pub fn int32(&mut self, val: i32) -> &mut Self {
        self.code(ty::TYPE_INT32);
        self.stream.write_i32_be(val);
        self
    }

    pub fn int64(&mut self, val: i64) -> &mut Self {
        self.code(ty::TYPE_INT64);
        self.stream.write_i64_be(val);
        self
    }

    pub fn uint8(&mut self, val: u8) -> &mut Self {
        self.code(ty::TYPE_UINT8);
        self.stream.write_u8(val);
        self
    }

    pub fn uint16(&mut self, val: u16) -> &mut Self {
        self.code(ty::TYPE_UINT16);
        self.stream.write_u16_be(val);
        self
    }

    pub fn uint32(&mut self, val: u32) -> &mut Self {
        self.code(ty::TYPE_UINT32);
        self.stream.write_u32_be(val);
        self
    }

    pub fn float16(&mut self, val: f32) -> &mut Self {
        self.code(ty::TYPE_FLOAT16);
        self.stream.write_f16_be(val);
        self
    }

    pub fn float32(&mut self, val: f32) -> &mut Self {
        self.code(ty::TYPE_FLOAT32);
        self.stream.write_f32_be(val);
        self
    }

    pub fn float64(&mut self, val: f64) -> &mut Self {
        self.code(ty::TYPE_FLOAT64);
        self.stream.write_f64_be(val);
        self
    }

    pub fn bool_(&mut self, val: bool) -> &mut Self {
        self.code(ty::TYPE_BOOL);
        self.stream.write_u8(if val { 0x01 } else { 0x00 });
        self
    }

    pub fn string(&mut self, val: &str) -> &mut Self {
        self.code(ty::TYPE_STRING);
        self.stream.write_string(val);
        self
    }

    pub fn null(&mut self) -> &mut Self {
        self.code(ty::TYPE_NULL)
    }

    pub fn timestamp(&mut self, val: i64) -> &mut Self {
        self.code(ty::TYPE_TIMESTAMP);
        self.stream.write_i64_be(val);
        self
    }

    // ── Convenience: typed lists ──

    pub fn list_of_float32(&mut self, values: &[f32]) -> &mut Self {
        self.begin_list(values.len() as u16);
        for &v in values {
            self.float32(v);
        }
        self.end_list()
    }

    pub fn list_of_int32(&mut self, values: &[i32]) -> &mut Self {
        self.begin_list(values.len() as u16);
        for &v in values {
            self.int32(v);
        }
        self.end_list()
    }

    // ── Domain codebook references ──

    pub fn l1_ref(&mut self, code: u16) -> &mut Self {
        self.code(esc::ESCAPE_L1);
        self.stream.write_u16_be(code);
        self
    }

    pub fn l2_ref(&mut self, code: u16) -> &mut Self {
        self.code(esc::ESCAPE_L2);
        self.stream.write_u16_be(code);
        self
    }

    pub fn l3_ref(&mut self, code: u16) -> &mut Self {
        self.code(esc::ESCAPE_L3);
        self.stream.write_u16_be(code);
        self
    }

    // ── Operators ──

    pub fn op(&mut self, opcode: u8) -> &mut Self { self.code(opcode) }
    pub fn add(&mut self) -> &mut Self { self.code(arith::ADD) }
    pub fn sub(&mut self) -> &mut Self { self.code(arith::SUB) }
    pub fn mul(&mut self) -> &mut Self { self.code(arith::MUL) }
    pub fn div(&mut self) -> &mut Self { self.code(arith::DIV) }
    pub fn distance(&mut self) -> &mut Self { self.code(arith::DISTANCE) }
    pub fn norm(&mut self) -> &mut Self { self.code(arith::NORM) }
    pub fn eq(&mut self) -> &mut Self { self.code(rel::EQ) }
    pub fn lt(&mut self) -> &mut Self { self.code(rel::LT) }
    pub fn gt(&mut self) -> &mut Self { self.code(rel::GT) }

    // ── Quantifiers ──

    pub fn forall(&mut self) -> &mut Self { self.code(quant::FORALL) }
    pub fn exists(&mut self) -> &mut Self { self.code(quant::EXISTS) }

    // ── Annotations ──

    pub fn confidence(&mut self, val: f32) -> &mut Self {
        self.code(meta::CONFIDENCE);
        self.stream.write_f16_be(val);
        self
    }

    pub fn label(&mut self, text: &str) -> &mut Self {
        self.code(meta::LABEL);
        self.stream.write_string(text);
        self
    }

    pub fn context_ref(&mut self, sct_index: u32) -> &mut Self {
        self.code(meta::CONTEXT_REF);
        self.stream.write_varint(sct_index);
        self
    }

    // ── Meta field helpers ──

    /// Emit SOURCE_AGENT(0x92) + 16 UUID bytes
    pub fn source_agent(&mut self, uuid: &[u8]) -> &mut Self {
        self.code(meta::SOURCE_AGENT);
        // Write exactly 16 bytes (pad or truncate)
        let mut buf = [0u8; 16];
        let len = uuid.len().min(16);
        buf[..len].copy_from_slice(&uuid[..len]);
        self.stream.write_uuid(&buf);
        self
    }

    /// Emit TOPIC(0x97) + u16
    pub fn topic(&mut self, topic_id: u16) -> &mut Self {
        self.code(meta::TOPIC);
        self.stream.write_u16_be(topic_id);
        self
    }

    // ── Negotiation pragmatic acts ──

    pub fn propose(&mut self) -> &mut Self { self.code(pragma::PROPOSE) }
    pub fn accept_pragma(&mut self) -> &mut Self { self.code(pragma::ACCEPT) }
    pub fn reject(&mut self) -> &mut Self { self.code(pragma::REJECT) }

    // ── Raw byte access ──

    pub fn raw(&mut self, data: &[u8]) -> &mut Self {
        self.stream.write_raw(data);
        self
    }

    pub fn current_size(&self) -> usize {
        self.stream.len()
    }
}

impl Default for AILLEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds epochs with sequence numbers and CRC-8 checksums.
pub struct EpochBuilder {
    seq: u16,
    epochs: Vec<Vec<u8>>,
    current_payload: ByteWriter,
}

impl EpochBuilder {
    pub fn new() -> Self {
        Self {
            seq: 0,
            epochs: Vec::new(),
            current_payload: ByteWriter::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        if self.current_payload.len() + data.len() > MAX_EPOCH_PAYLOAD {
            self.flush();
        }
        self.current_payload.write_raw(data);
    }

    pub fn flush(&mut self) {
        if self.current_payload.is_empty() {
            return;
        }
        let payload = self.current_payload.to_bytes();
        let mut epoch = ByteWriter::new();
        epoch.write_u16_be(self.seq);
        epoch.write_u16_be(payload.len() as u16);
        epoch.write_raw(&payload);
        // CRC-8 over (seq + length + payload)
        let epoch_bytes = epoch.to_bytes();
        let checksum = crc8(&epoch_bytes);
        epoch.write_u8(checksum);
        self.epochs.push(epoch.into_bytes());
        self.seq += 1;
        self.current_payload = ByteWriter::new();
    }

    pub fn get_epochs(&mut self) -> Vec<Vec<u8>> {
        self.flush();
        self.epochs.clone()
    }
}

impl Default for EpochBuilder {
    fn default() -> Self {
        Self::new()
    }
}
