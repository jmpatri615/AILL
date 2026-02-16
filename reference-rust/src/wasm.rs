use wasm_bindgen::prelude::*;
use crate::codebook::base::{self, fc, ty, st, pragma, BASE_CODEBOOK};
use crate::encoder::AILLEncoder;
use crate::decoder::AILLDecoder;
use crate::pretty_print as pp;
use crate::wire::crc8::crc8 as compute_crc8;

// ═══════════════════════════════════════════════════════════════════════
// Encoding functions
// ═══════════════════════════════════════════════════════════════════════

/// Encode a string message as an AILL ASSERT utterance.
/// Equivalent to JS `AILL.encodeString(msg)`.
#[wasm_bindgen]
pub fn encode_string(msg: &str) -> Vec<u8> {
    let mut enc = AILLEncoder::new();
    enc.start_utterance();
    enc.assert_();
    enc.string(msg);
    enc.end_utterance()
}

/// Encode a URL as an AILL ASSERT utterance with struct { type: "url", content: url }.
/// Equivalent to JS `AILL.encodeURL(url)`.
#[wasm_bindgen]
pub fn encode_url(url: &str) -> Vec<u8> {
    let mut enc = AILLEncoder::new();
    enc.start_utterance();
    enc.assert_();
    enc.begin_struct();
    enc.field(0x0001); // type
    enc.string("url");
    enc.field(0x0002); // content
    enc.string(url);
    enc.end_struct();
    enc.end_utterance()
}

/// Encode arbitrary content as an AILL ASSERT utterance with struct { type, content }.
/// Equivalent to JS `AILL.encodeContent(type, content)`.
#[wasm_bindgen]
pub fn encode_content(content_type: &str, content: &str) -> Vec<u8> {
    let mut enc = AILLEncoder::new();
    enc.start_utterance();
    enc.assert_();
    enc.begin_struct();
    enc.field(0x0001); // type
    enc.string(content_type);
    enc.field(0x0002); // content
    enc.string(content);
    enc.end_struct();
    enc.end_utterance()
}

// ═══════════════════════════════════════════════════════════════════════
// Negotiation encoding functions
// ═══════════════════════════════════════════════════════════════════════

/// Encode a pragmatic act message for song negotiation protocol.
/// act: 0x88=PROPOSE, 0x89=ACCEPT, 0x85=REJECT, 0x81=ASSERT, etc.
/// topic_id: e.g. 0x0100=song_proposal, 0x0101=role_claim, 0x0102=heartbeat
/// content: string payload (e.g. song key, role name, heartbeat data)
/// agent_id: 16-byte UUID of the source agent
#[wasm_bindgen]
pub fn encode_pragmatic(act: u8, topic_id: u16, content: &str, agent_id: &[u8]) -> Vec<u8> {
    let mut enc = AILLEncoder::new();
    enc.start_utterance();
    enc.source_agent(agent_id);
    enc.topic(topic_id);
    enc.pragma(act);
    enc.string(content);
    enc.end_utterance()
}

/// Encode a task allocation message (PROPOSE + PLAN-1 ALLOCATE_TASK struct).
/// task_id: numeric task identifier
/// role: role string (e.g. "lead", "harmony", "bass", "descant")
/// agent_id: 16-byte UUID of the source agent
#[wasm_bindgen]
pub fn encode_task_allocation(task_id: u32, role: &str, agent_id: &[u8]) -> Vec<u8> {
    let mut enc = AILLEncoder::new();
    enc.start_utterance();
    enc.source_agent(agent_id);
    enc.topic(0x0101); // role_claim topic
    enc.propose();
    // PLAN-1 ALLOCATE_TASK struct: ESCAPE_L1 + 0x000D + struct{task_id, role}
    enc.l1_ref(0x000D); // ALLOCATE_TASK
    enc.begin_struct();
    enc.field(0x0001); // task_id field
    enc.uint32(task_id);
    enc.field(0x0002); // role field
    enc.string(role);
    enc.end_struct();
    enc.end_utterance()
}

/// Decode a pragmatic message, returning { act, topic, content, agent } or null.
#[wasm_bindgen]
pub fn decode_pragmatic_simple(data: &[u8]) -> JsValue {
    match decode_pragmatic_inner(data) {
        Some((act, topic, content, agent_hex)) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"act".into(), &act.into()).ok();
            js_sys::Reflect::set(&obj, &"topic".into(), &JsValue::from(topic)).ok();
            js_sys::Reflect::set(&obj, &"content".into(), &content.into()).ok();
            js_sys::Reflect::set(&obj, &"agent".into(), &agent_hex.into()).ok();
            obj.into()
        }
        None => JsValue::NULL,
    }
}

/// Internal: parse pragmatic negotiation messages.
fn decode_pragmatic_inner(bytes: &[u8]) -> Option<(String, u16, String, String)> {
    if bytes.is_empty() || bytes[0] != fc::START_UTTERANCE {
        return None;
    }
    let mut pos: usize = 1;
    let mut agent_bytes: Option<[u8; 16]> = None;
    let mut topic_id: u16 = 0;

    // Parse meta header, extracting SOURCE_AGENT and TOPIC
    while pos < bytes.len() {
        let code = bytes[pos];
        match code {
            0x90 => { pos += 3; } // CONFIDENCE + f16
            0x91 => { pos += 2; } // PRIORITY + u8
            0x94 => { pos += 9; } // TIMESTAMP + i64
            0x92 => { // SOURCE_AGENT + 16 bytes
                pos += 1;
                if pos + 16 > bytes.len() { return None; }
                let mut uuid = [0u8; 16];
                uuid.copy_from_slice(&bytes[pos..pos + 16]);
                agent_bytes = Some(uuid);
                pos += 16;
            }
            0x93 => { // DEST_AGENT + 16 bytes
                pos += 17;
            }
            0x95 => { // SEQNUM + u32
                pos += 5;
            }
            0x97 => { // TOPIC + u16
                pos += 1;
                if pos + 2 > bytes.len() { return None; }
                topic_id = ((bytes[pos] as u16) << 8) | (bytes[pos + 1] as u16);
                pos += 2;
            }
            0x96 | 0x98..=0x9F => { pos += 1; } // other meta (skip opcode only)
            _ => break,
        }
    }

    if pos >= bytes.len() {
        return None;
    }

    // Read pragmatic act
    let act_code = bytes[pos];
    pos += 1;

    let act_name = match act_code {
        0x80 => "QUERY",
        0x81 => "ASSERT",
        0x82 => "REQUEST",
        0x83 => "COMMAND",
        0x84 => "ACKNOWLEDGE",
        0x85 => "REJECT",
        0x88 => "PROPOSE",
        0x89 => "ACCEPT",
        0x8A => "WARN",
        0x8E => "GREET",
        0x8F => "FAREWELL",
        _ => return None, // Not a pragmatic act
    };

    // Try to read content string
    let content = if pos < bytes.len() && bytes[pos] == ty::TYPE_STRING {
        pos += 1;
        if pos + 2 > bytes.len() { return None; }
        let len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
        pos += 2;
        if pos + len > bytes.len() { return None; }
        std::str::from_utf8(&bytes[pos..pos + len]).unwrap_or("").to_string()
    } else {
        String::new()
    };

    let agent_hex = match agent_bytes {
        Some(uuid) => uuid.iter().map(|b| format!("{:02x}", b)).collect::<String>(),
        None => String::new(),
    };

    Some((act_name.to_string(), topic_id, content, agent_hex))
}

// ═══════════════════════════════════════════════════════════════════════
// Decoding functions
// ═══════════════════════════════════════════════════════════════════════

/// Simple decoder that returns {type, content} or null.
/// Matches the demo's `AILL.decode(wire)` behavior.
#[wasm_bindgen]
pub fn decode_simple(data: &[u8]) -> JsValue {
    // Quick parse: walk the wire bytes looking for the same patterns
    // the JS decoder does. We need to return a simple {type, content} object.
    let result = simple_decode(data);
    match result {
        Some((typ, content)) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"type".into(), &typ.into()).ok();
            js_sys::Reflect::set(&obj, &"content".into(), &content.into()).ok();
            obj.into()
        }
        None => JsValue::NULL,
    }
}

/// Internal simple decoder matching JS demo behavior.
fn simple_decode(bytes: &[u8]) -> Option<(String, String)> {
    if bytes.is_empty() || bytes[0] != fc::START_UTTERANCE {
        return None;
    }
    let mut pos: usize = 1;

    // Skip meta header
    while pos < bytes.len() {
        let code = bytes[pos];
        match code {
            0x90 => { pos += 3; } // CONFIDENCE + f16
            0x91 => { pos += 2; } // PRIORITY + u8
            0x94 => { pos += 9; } // TIMESTAMP + i64
            0x92..=0x9F => { pos += 1; } // other meta
            _ => break,
        }
    }

    if pos >= bytes.len() {
        return None;
    }

    let code = bytes[pos];
    pos += 1;

    if code == pragma::ASSERT && pos < bytes.len() {
        let type_code = bytes[pos];
        pos += 1;

        if type_code == ty::TYPE_STRING {
            // Simple string
            if pos + 2 > bytes.len() { return None; }
            let len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
            pos += 2;
            if pos + len > bytes.len() { return None; }
            let s = std::str::from_utf8(&bytes[pos..pos + len]).ok()?;
            return Some(("string".to_string(), s.to_string()));
        }

        if type_code == st::BEGIN_STRUCT {
            // Struct with type/content fields
            let mut typ = String::new();
            let mut content = String::new();
            while pos < bytes.len() && bytes[pos] != st::END_STRUCT {
                if bytes[pos] == st::FIELD_ID {
                    pos += 1;
                    if pos + 2 > bytes.len() { break; }
                    let fid = ((bytes[pos] as u16) << 8) | (bytes[pos + 1] as u16);
                    pos += 2;
                    if pos < bytes.len() && bytes[pos] == ty::TYPE_STRING {
                        pos += 1;
                        if pos + 2 > bytes.len() { break; }
                        let len = ((bytes[pos] as usize) << 8) | (bytes[pos + 1] as usize);
                        pos += 2;
                        if pos + len > bytes.len() { break; }
                        let val = std::str::from_utf8(&bytes[pos..pos + len]).unwrap_or("");
                        pos += len;
                        match fid {
                            1 => typ = val.to_string(),
                            2 => content = val.to_string(),
                            _ => {}
                        }
                    } else {
                        pos += 1; // skip unknown value type
                    }
                } else {
                    pos += 1;
                }
            }
            if !typ.is_empty() || !content.is_empty() {
                return Some((typ, content));
            }
        }
    }

    None
}

/// Full AST decode — returns the AST as a JS value (serde-serialized).
#[wasm_bindgen]
pub fn decode_ast(data: &[u8]) -> Result<JsValue, JsError> {
    let decoder = AILLDecoder::new();
    let node = decoder.decode_utterance(data)
        .map_err(|e| JsError::new(&format!("Decode error: {}", e)))?;
    serde_wasm_bindgen::to_value(&node)
        .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
}

/// Pretty-print AILL wire-format bytes as a human-readable tree.
#[wasm_bindgen]
pub fn pretty_print_bytes(data: &[u8]) -> Result<String, JsError> {
    let decoder = AILLDecoder::new();
    let node = decoder.decode_utterance(data)
        .map_err(|e| JsError::new(&format!("Decode error: {}", e)))?;
    Ok(pp(&node, 0))
}

// ═══════════════════════════════════════════════════════════════════════
// Utility functions
// ═══════════════════════════════════════════════════════════════════════

/// Compute CRC-8 of data. Equivalent to JS `AILL.crc8(data)`.
#[wasm_bindgen]
pub fn crc8_compute(data: &[u8]) -> u8 {
    compute_crc8(data)
}

/// Generate a hex dump of data with HTML formatting.
/// Equivalent to JS `AILL.hexDump(data, maxBytes)`.
#[wasm_bindgen]
pub fn hex_dump(data: &[u8], max_bytes: usize) -> String {
    let len = data.len().min(max_bytes);
    let mut lines = Vec::new();

    let mut i = 0;
    while i < len {
        let end = (i + 16).min(len);
        let slice = &data[i..end];

        let hex: String = slice.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        let ascii: String = slice.iter()
            .map(|&b| if b >= 32 && b < 127 { b as char } else { '.' })
            .collect();

        lines.push(format!(
            "<span class=\"hex-offset\">{:04x}</span>  <span class=\"hex-data\">{:<48}</span>  <span class=\"hex-ascii\">{}</span>",
            i, hex, ascii
        ));

        i += 16;
    }

    if data.len() > max_bytes {
        lines.push(format!(
            "<span class=\"hex-offset\">...</span>  <span class=\"hex-data\">({} more bytes)</span>",
            data.len() - max_bytes
        ));
    }

    lines.join("\n")
}

/// Get the full mnemonic table as a JS object { code: name, ... }.
/// Equivalent to JS `AILL.MNEMONICS`.
#[wasm_bindgen]
pub fn get_mnemonics() -> JsValue {
    let obj = js_sys::Object::new();
    for entry in BASE_CODEBOOK.iter() {
        if !entry.mnemonic.is_empty() && entry.mnemonic != "RESERVED" {
            js_sys::Reflect::set(
                &obj,
                &JsValue::from(entry.code as u32),
                &entry.mnemonic.into(),
            ).ok();
        }
    }
    obj.into()
}

/// Look up the mnemonic name for a single opcode byte.
#[wasm_bindgen]
pub fn mnemonic_for(code: u8) -> String {
    base::mnemonic_for(code).to_string()
}

/// Validate CRC of wire-format bytes (epoch format).
#[wasm_bindgen]
pub fn validate_epoch(data: &[u8]) -> bool {
    if data.len() < 5 {
        return false;
    }
    match crate::decode_epoch(data, 0) {
        Ok((epoch, _)) => epoch.crc_ok,
        Err(_) => false,
    }
}
