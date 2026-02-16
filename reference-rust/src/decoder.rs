use std::collections::BTreeMap;

use crate::ast::{AstNode, MetaHeader, LiteralValue, AnnotationValue, DecodedEpoch};
use crate::codebook::base::{fc, ty, st, meta, modal, esc, BASE_CODEBOOK};
use crate::error::AILLError;
use crate::wire::ByteReader;
use crate::wire::crc8::crc8;

/// Decodes AILL wire-format bytes into an AST.
pub struct AILLDecoder;

impl AILLDecoder {
    pub fn new() -> Self {
        Self
    }

    /// Decode a complete AILL utterance from wire bytes.
    pub fn decode_utterance(&self, data: &[u8]) -> Result<AstNode, AILLError> {
        let mut reader = ByteReader::new(data);

        // Expect START_UTTERANCE
        let code = reader.read_u8()?;
        if code != fc::START_UTTERANCE {
            return Err(AILLError::InvalidStructure(format!(
                "Expected START_UTTERANCE (0x00), got 0x{:02X}",
                code
            )));
        }

        // Decode meta header
        let meta_header = decode_meta_header(&mut reader)?;

        // Decode body expressions until END_UTTERANCE
        let mut body = Vec::new();
        while !reader.is_empty() {
            if reader.peek()? == fc::END_UTTERANCE {
                reader.read_u8()?; // consume
                break;
            }
            if let Some(expr) = decode_expression(&mut reader)? {
                body.push(expr);
            }
        }

        Ok(AstNode::Utterance {
            meta: meta_header,
            body,
        })
    }
}

impl Default for AILLDecoder {
    fn default() -> Self {
        Self::new()
    }
}

fn decode_meta_header(reader: &mut ByteReader) -> Result<MetaHeader, AILLError> {
    let mut hdr = MetaHeader::default();

    // CONFIDENCE (mandatory)
    let code = reader.read_u8()?;
    if code != meta::CONFIDENCE {
        return Err(AILLError::InvalidStructure(format!(
            "Expected CONFIDENCE (0x90), got 0x{:02X}", code
        )));
    }
    hdr.confidence = reader.read_f16_be()?;

    // PRIORITY (mandatory)
    let code = reader.read_u8()?;
    if code != meta::PRIORITY {
        return Err(AILLError::InvalidStructure(format!(
            "Expected PRIORITY (0x91), got 0x{:02X}", code
        )));
    }
    hdr.priority = reader.read_u8()?;

    // TIMESTAMP (mandatory)
    let code = reader.read_u8()?;
    if code != meta::TIMESTAMP_META {
        return Err(AILLError::InvalidStructure(format!(
            "Expected TIMESTAMP_META (0x94), got 0x{:02X}", code
        )));
    }
    hdr.timestamp_us = reader.read_i64_be()?;

    // Optional meta annotations (0x92-0x9F range)
    while !reader.is_empty() {
        let peek = reader.peek()?;
        if !(0x92..=0x9F).contains(&peek) {
            break;
        }
        let ann_code = reader.read_u8()?;
        match ann_code {
            meta::SOURCE_AGENT => {
                hdr.source_agent = Some(reader.read_uuid()?.to_vec());
            }
            meta::DEST_AGENT => {
                hdr.dest_agent = Some(reader.read_uuid()?.to_vec());
            }
            meta::SEQNUM => {
                hdr.seqnum = Some(reader.read_u32_be()?);
            }
            meta::TRACE_ID => {
                hdr.annotations.insert("trace_id".into(), AnnotationValue::U64(reader.read_u64_be()?));
            }
            meta::TTL => {
                hdr.annotations.insert("ttl".into(), AnnotationValue::U16(reader.read_u16_be()?));
            }
            meta::TOPIC => {
                hdr.annotations.insert("topic".into(), AnnotationValue::U16(reader.read_u16_be()?));
            }
            meta::VERSION_TAG => {
                let major = reader.read_u16_be()?;
                let minor = reader.read_u16_be()?;
                hdr.annotations.insert("version".into(), AnnotationValue::Pair(major, minor));
            }
            _ => break,
        }
    }

    Ok(hdr)
}

fn decode_expression(reader: &mut ByteReader) -> Result<Option<AstNode>, AILLError> {
    if reader.is_empty() {
        return Ok(None);
    }

    let code = reader.peek()?;

    // Pragmatic acts (0x80-0x8F)
    if (0x80..=0x8F).contains(&code) {
        return Ok(Some(decode_pragmatic(reader)?));
    }

    // Modality (0x70-0x7F)
    if (0x70..=0x7F).contains(&code) {
        return Ok(Some(decode_modal(reader)?));
    }

    // Temporal (0x60-0x6F)
    if (0x60..=0x6F).contains(&code) {
        return Ok(Some(decode_temporal(reader)?));
    }

    // Meta annotations inline
    if code == meta::CONFIDENCE || code == meta::LABEL {
        return Ok(Some(decode_annotation(reader)?));
    }

    // Type markers (literals)
    if (0x10..=0x1F).contains(&code) {
        return Ok(Some(decode_literal(reader)?));
    }

    // Structure codes
    if code == st::BEGIN_STRUCT {
        return Ok(Some(decode_struct(reader)?));
    }
    if code == st::BEGIN_LIST {
        return Ok(Some(decode_list(reader)?));
    }
    if code == st::BEGIN_MAP {
        return Ok(Some(decode_map(reader)?));
    }

    // Escape/domain refs
    if code == esc::ESCAPE_L1 || code == esc::ESCAPE_L2 || code == esc::ESCAPE_L3 {
        return Ok(Some(decode_domain_ref(reader)?));
    }

    // Context ref
    if code == meta::CONTEXT_REF {
        reader.read_u8()?;
        let idx = reader.read_varint()?;
        return Ok(Some(AstNode::ContextRef { sct_index: idx }));
    }

    // NOP
    if code == esc::NOP {
        reader.read_u8()?;
        return Ok(None);
    }

    // COMMENT
    if code == esc::COMMENT {
        reader.read_u8()?;
        let _comment = reader.read_string()?;
        return Ok(None);
    }

    // Operators and other codes - emit as-is
    reader.read_u8()?;
    let mnemonic = BASE_CODEBOOK[code as usize].mnemonic.to_string();
    Ok(Some(AstNode::Code { code, mnemonic }))
}

fn decode_literal(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;

    let (value_type, value) = match code {
        ty::TYPE_INT8 => ("int8", LiteralValue::Int8(reader.read_i8()?)),
        ty::TYPE_INT16 => ("int16", LiteralValue::Int16(reader.read_i16_be()?)),
        ty::TYPE_INT32 => ("int32", LiteralValue::Int32(reader.read_i32_be()?)),
        ty::TYPE_INT64 => ("int64", LiteralValue::Int64(reader.read_i64_be()?)),
        ty::TYPE_UINT8 => ("uint8", LiteralValue::Uint8(reader.read_u8()?)),
        ty::TYPE_UINT16 => ("uint16", LiteralValue::Uint16(reader.read_u16_be()?)),
        ty::TYPE_UINT32 => ("uint32", LiteralValue::Uint32(reader.read_u32_be()?)),
        ty::TYPE_UINT64 => ("uint64", LiteralValue::Uint64(reader.read_u64_be()?)),
        ty::TYPE_FLOAT16 => ("float16", LiteralValue::Float16(reader.read_f16_be()?)),
        ty::TYPE_FLOAT32 => ("float32", LiteralValue::Float32(reader.read_f32_be()?)),
        ty::TYPE_FLOAT64 => ("float64", LiteralValue::Float64(reader.read_f64_be()?)),
        ty::TYPE_BOOL => ("bool", LiteralValue::Bool(reader.read_u8()? != 0)),
        ty::TYPE_STRING => ("string", LiteralValue::String(reader.read_string()?)),
        ty::TYPE_BYTES => {
            let length = reader.read_u16_be()? as usize;
            ("bytes", LiteralValue::Bytes(reader.read_n_bytes(length)?))
        }
        ty::TYPE_TIMESTAMP => ("timestamp", LiteralValue::Timestamp(reader.read_i64_be()?)),
        ty::TYPE_NULL => ("null", LiteralValue::Null),
        _ => return Err(AILLError::InvalidOpCode(code)),
    };

    Ok(AstNode::Literal {
        value_type: value_type.to_string(),
        value,
    })
}

fn decode_struct(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    reader.read_u8()?; // consume BEGIN_STRUCT
    let mut fields = BTreeMap::new();
    let mut positional_idx: u16 = 0;

    while !reader.is_empty() && reader.peek()? != st::END_STRUCT {
        if reader.peek()? == st::FIELD_SEP {
            reader.read_u8()?;
            continue;
        }
        if reader.peek()? == st::FIELD_ID {
            reader.read_u8()?;
            let field_code = reader.read_u16_be()?;
            if let Some(value) = decode_expression(reader)? {
                fields.insert(field_code, value);
            }
        } else {
            // Unnamed (positional) field
            if let Some(expr) = decode_expression(reader)? {
                fields.insert(positional_idx, expr);
                positional_idx += 1;
            }
        }
    }
    if !reader.is_empty() {
        reader.read_u8()?; // consume END_STRUCT
    }

    Ok(AstNode::Struct { fields })
}

fn decode_list(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    reader.read_u8()?; // consume BEGIN_LIST
    let count = reader.read_u16_be()?;
    let mut elements = Vec::new();

    for _ in 0..count {
        if reader.is_empty() || reader.peek()? == st::END_LIST {
            break;
        }
        if let Some(elem) = decode_expression(reader)? {
            elements.push(elem);
        }
    }
    if !reader.is_empty() && reader.peek()? == st::END_LIST {
        reader.read_u8()?; // consume END_LIST
    }

    Ok(AstNode::List { count, elements })
}

fn decode_map(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    reader.read_u8()?; // consume BEGIN_MAP
    let count = reader.read_u16_be()?;
    let mut pairs = Vec::new();

    for _ in 0..count {
        if reader.is_empty() || reader.peek()? == st::END_MAP {
            break;
        }
        let key = decode_expression(reader)?.unwrap_or(AstNode::Literal {
            value_type: "null".into(),
            value: LiteralValue::Null,
        });
        let val = decode_expression(reader)?.unwrap_or(AstNode::Literal {
            value_type: "null".into(),
            value: LiteralValue::Null,
        });
        pairs.push((key, val));
    }
    if !reader.is_empty() && reader.peek()? == st::END_MAP {
        reader.read_u8()?;
    }

    Ok(AstNode::Map { count, pairs })
}

fn decode_pragmatic(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;
    let act_name = BASE_CODEBOOK[code as usize].mnemonic.to_string();
    let expr = decode_expression(reader)?.unwrap_or(AstNode::Literal {
        value_type: "null".into(),
        value: LiteralValue::Null,
    });
    Ok(AstNode::Pragmatic {
        act: act_name,
        expression: Box::new(expr),
    })
}

fn decode_modal(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;
    let mod_name = BASE_CODEBOOK[code as usize].mnemonic.to_string();
    let extra = match code {
        modal::PREDICTED => Some(reader.read_f16_be()? as f64),
        modal::REPORTED => {
            let _uuid = reader.read_uuid()?;
            None // UUID handled separately; matching Python which stores it as extra
        }
        _ => None,
    };
    let expr = decode_expression(reader)?.unwrap_or(AstNode::Literal {
        value_type: "null".into(),
        value: LiteralValue::Null,
    });
    Ok(AstNode::Modal {
        modality: mod_name,
        expression: Box::new(expr),
        extra,
    })
}

fn decode_temporal(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;
    let mod_name = BASE_CODEBOOK[code as usize].mnemonic.to_string();
    let expr = decode_expression(reader)?.unwrap_or(AstNode::Literal {
        value_type: "null".into(),
        value: LiteralValue::Null,
    });
    Ok(AstNode::Temporal {
        modifier: mod_name,
        expression: Box::new(expr),
    })
}

fn decode_annotation(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;
    let mnemonic = if code == meta::CONFIDENCE {
        let conf = reader.read_f16_be()?;
        let _expr = decode_expression(reader)?;
        format!("CONFIDENCE({:.2})", conf)
    } else if code == meta::LABEL {
        let label = reader.read_string()?;
        let _expr = decode_expression(reader)?;
        format!("LABEL({})", label)
    } else {
        format!("ANNOTATION_0x{:02X}", code)
    };

    Ok(AstNode::Annotated { code, mnemonic })
}

fn decode_domain_ref(reader: &mut ByteReader) -> Result<AstNode, AILLError> {
    let code = reader.read_u8()?;
    let level = match code {
        esc::ESCAPE_L1 => 1,
        esc::ESCAPE_L2 => 2,
        esc::ESCAPE_L3 => 3,
        _ => return Err(AILLError::InvalidOpCode(code)),
    };
    let domain_code = reader.read_u16_be()?;
    Ok(AstNode::DomainRef { level, domain_code })
}

/// Decode a single epoch from wire bytes.
/// Returns (DecodedEpoch, bytes_consumed).
pub fn decode_epoch(data: &[u8], offset: usize) -> Result<(DecodedEpoch, usize), AILLError> {
    if data.len() - offset < 5 {
        return Err(AILLError::InvalidStructure(
            "Insufficient data for epoch header".into(),
        ));
    }

    let seq_num = u16::from_be_bytes([data[offset], data[offset + 1]]);
    let payload_len = u16::from_be_bytes([data[offset + 2], data[offset + 3]]) as usize;

    if data.len() - offset < 4 + payload_len + 1 {
        return Err(AILLError::InvalidStructure(format!(
            "Incomplete epoch payload (expected {} bytes)",
            payload_len
        )));
    }

    let payload = data[offset + 4..offset + 4 + payload_len].to_vec();
    let received_crc = data[offset + 4 + payload_len];

    // Verify CRC over (seq + len + payload)
    let computed_crc = crc8(&data[offset..offset + 4 + payload_len]);
    let crc_ok = received_crc == computed_crc;

    let total_consumed = 4 + payload_len + 1;
    Ok((
        DecodedEpoch {
            seq_num,
            payload,
            crc_ok,
        },
        total_consumed,
    ))
}

// ═══════════════════════════════════════════════════════════════════════
// Pretty Printer
// ═══════════════════════════════════════════════════════════════════════

/// Produce a human-readable representation of a decoded AILL AST.
pub fn pretty_print(node: &AstNode, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    let mut lines = Vec::new();

    match node {
        AstNode::Utterance { meta, body } => {
            lines.push(format!("{}UTTERANCE:", prefix));
            lines.push(pretty_print_meta(meta, indent + 1));
            lines.push(format!("{}  BODY:", prefix));
            for expr in body {
                lines.push(pretty_print(expr, indent + 2));
            }
        }
        AstNode::Literal { value_type, value } => {
            let val_str = match value {
                LiteralValue::Int8(v) => v.to_string(),
                LiteralValue::Int16(v) => v.to_string(),
                LiteralValue::Int32(v) => v.to_string(),
                LiteralValue::Int64(v) => v.to_string(),
                LiteralValue::Uint8(v) => v.to_string(),
                LiteralValue::Uint16(v) => v.to_string(),
                LiteralValue::Uint32(v) => v.to_string(),
                LiteralValue::Uint64(v) => v.to_string(),
                LiteralValue::Float16(v) => format!("{}", v),
                LiteralValue::Float32(v) => format!("{}", v),
                LiteralValue::Float64(v) => format!("{}", v),
                LiteralValue::Bool(v) => v.to_string(),
                LiteralValue::String(v) => v.clone(),
                LiteralValue::Bytes(v) => format!("{:?}", v),
                LiteralValue::Timestamp(v) => v.to_string(),
                LiteralValue::Null => "None".to_string(),
            };
            lines.push(format!("{}{}: {}", prefix, value_type, val_str));
        }
        AstNode::Struct { fields } => {
            lines.push(format!("{}STRUCT:", prefix));
            for (fid, val) in fields {
                lines.push(format!("{}  field_0x{:04X}:", prefix, fid));
                lines.push(pretty_print(val, indent + 2));
            }
        }
        AstNode::List { count, elements } => {
            lines.push(format!("{}LIST[{}]:", prefix, count));
            for elem in elements {
                lines.push(pretty_print(elem, indent + 1));
            }
        }
        AstNode::Map { count, pairs } => {
            lines.push(format!("{}MAP[{}]:", prefix, count));
            for (k, v) in pairs {
                lines.push(format!("{}  key: {}", prefix, pretty_print(k, 0).trim()));
                lines.push(format!("{}  val: {}", prefix, pretty_print(v, 0).trim()));
            }
        }
        AstNode::Pragmatic { act, expression } => {
            lines.push(format!("{}{}:", prefix, act));
            lines.push(pretty_print(expression, indent + 1));
        }
        AstNode::Modal { modality, expression, extra } => {
            let extra_str = match extra {
                Some(v) => format!(" (horizon={}ms)", v),
                None => String::new(),
            };
            lines.push(format!("{}[{}{}]:", prefix, modality, extra_str));
            lines.push(pretty_print(expression, indent + 1));
        }
        AstNode::Temporal { modifier, expression } => {
            lines.push(format!("{}<{}>:", prefix, modifier));
            lines.push(pretty_print(expression, indent + 1));
        }
        AstNode::DomainRef { level, domain_code } => {
            let level_name = match level {
                1 => "L1",
                2 => "L2",
                3 => "L3",
                _ => "?",
            };
            lines.push(format!("{}REF({}: DOMAIN_0x{:04X})", prefix, level_name, domain_code));
        }
        AstNode::ContextRef { sct_index } => {
            lines.push(format!("{}SCT_REF[{}]", prefix, sct_index));
        }
        AstNode::Code { mnemonic, .. } => {
            lines.push(format!("{}{}", prefix, mnemonic));
        }
        AstNode::Annotated { mnemonic, .. } => {
            lines.push(format!("{}{}", prefix, mnemonic));
        }
    }

    lines.join("\n")
}

fn pretty_print_meta(meta: &MetaHeader, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    let mut lines = Vec::new();
    lines.push(format!(
        "{}META: confidence={:.2} priority={} timestamp={}",
        prefix, meta.confidence, meta.priority, meta.timestamp_us
    ));
    if let Some(ref dest) = meta.dest_agent {
        let hex: String = dest.iter().map(|b| format!("{:02x}", b)).collect();
        lines.push(format!("{}  dest_agent={}", prefix, hex));
    }
    if let Some(seq) = meta.seqnum {
        lines.push(format!("{}  seqnum={}", prefix, seq));
    }
    lines.join("\n")
}
