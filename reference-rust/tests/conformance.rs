/// AILL Conformance Test Suite (ACTS) - Rust Port
/// Tests the reference implementation against the specification.
///
/// Port of all 35 tests from Python test_conformance.py

use aill::*;
use aill::codebook::base::temporal;

// Helper to extract the body expression from an utterance
fn body_expr(node: &AstNode, idx: usize) -> &AstNode {
    match node {
        AstNode::Utterance { body, .. } => &body[idx],
        _ => panic!("Expected Utterance"),
    }
}

fn inner_expression(node: &AstNode) -> &AstNode {
    match node {
        AstNode::Pragmatic { expression, .. } => expression,
        AstNode::Modal { expression, .. } => expression,
        AstNode::Temporal { expression, .. } => expression,
        _ => panic!("Expected wrapping node"),
    }
}

fn get_meta(node: &AstNode) -> &MetaHeader {
    match node {
        AstNode::Utterance { meta, .. } => meta,
        _ => panic!("Expected Utterance"),
    }
}

fn literal_value(node: &AstNode) -> &LiteralValue {
    match node {
        AstNode::Literal { value, .. } => value,
        _ => panic!("Expected Literal, got {:?}", node),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TG-TYPES: Type System Tests (10 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_ty_001_int8_roundtrip() {
    for v in [-128i8, -1, 0, 1, 127] {
        let mut e = AILLEncoder::new();
        e.start_utterance().assert_().int8(v);
        let wire = e.end_utterance();
        let d = AILLDecoder::new();
        let utt = d.decode_utterance(&wire).unwrap();
        let lit = inner_expression(body_expr(&utt, 0));
        assert_eq!(*literal_value(lit), LiteralValue::Int8(v));
    }
}

#[test]
fn tg_ty_002_int32_roundtrip() {
    for v in [-2_147_483_648i32, 0, 2_147_483_647] {
        let mut e = AILLEncoder::new();
        e.start_utterance().assert_().int32(v);
        let wire = e.end_utterance();
        let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
        let lit = inner_expression(body_expr(&utt, 0));
        assert_eq!(*literal_value(lit), LiteralValue::Int32(v));
    }
}

#[test]
fn tg_ty_003_float32_special_values() {
    for v in [0.0f32, -0.0, 1.5, -1.5, f32::INFINITY, f32::NEG_INFINITY] {
        let mut e = AILLEncoder::new();
        e.start_utterance().assert_().float32(v);
        let wire = e.end_utterance();
        let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
        let result = match literal_value(inner_expression(body_expr(&utt, 0))) {
            LiteralValue::Float32(f) => *f,
            _ => panic!("Expected Float32"),
        };
        if v.is_infinite() {
            assert!(result.is_infinite());
            assert_eq!(v.is_sign_positive(), result.is_sign_positive());
        } else {
            assert_eq!(result, v);
        }
    }
    // NaN
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().float32(f32::NAN);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let result = match literal_value(inner_expression(body_expr(&utt, 0))) {
        LiteralValue::Float32(f) => *f,
        _ => panic!("Expected Float32"),
    };
    assert!(result.is_nan());
}

#[test]
fn tg_ty_004_float16_roundtrip() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().float16(0.5);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let result = match literal_value(inner_expression(body_expr(&utt, 0))) {
        LiteralValue::Float16(f) => *f,
        _ => panic!("Expected Float16"),
    };
    assert!((result - 0.5).abs() < 0.01);
}

#[test]
fn tg_ty_005_string_utf8() {
    let test_str = "Hello AILL! \u{1F916}";
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().string(test_str);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let result = match literal_value(inner_expression(body_expr(&utt, 0))) {
        LiteralValue::String(s) => s.clone(),
        _ => panic!("Expected String"),
    };
    assert_eq!(result, test_str);
}

#[test]
fn tg_ty_006_empty_string() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().string("");
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let result = match literal_value(inner_expression(body_expr(&utt, 0))) {
        LiteralValue::String(s) => s.clone(),
        _ => panic!("Expected String"),
    };
    assert_eq!(result, "");
}

#[test]
fn tg_ty_007_bool() {
    for v in [true, false] {
        let mut e = AILLEncoder::new();
        e.start_utterance().assert_().bool_(v);
        let wire = e.end_utterance();
        let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
        assert_eq!(*literal_value(inner_expression(body_expr(&utt, 0))), LiteralValue::Bool(v));
    }
}

#[test]
fn tg_ty_008_null() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().null();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    assert_eq!(*literal_value(inner_expression(body_expr(&utt, 0))), LiteralValue::Null);
}

#[test]
fn tg_ty_009_timestamp() {
    let ts: i64 = 1_740_000_000_000_000;
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().timestamp(ts);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    assert_eq!(
        *literal_value(inner_expression(body_expr(&utt, 0))),
        LiteralValue::Timestamp(ts)
    );
}

#[test]
fn tg_ty_010_uint16_uint32() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().uint16(65535).uint32(4_294_967_295);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    // Basic parse success
    assert!(matches!(&utt, AstNode::Utterance { .. }));
}

// ═══════════════════════════════════════════════════════════════════════
// TG-STRUCT: Structure Tests (4 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_st_001_simple_struct() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_();
    e.begin_struct();
    e.field(0x0000).float32(3.5);
    e.field(0x0001).float32(7.2);
    e.end_struct();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let s = inner_expression(body_expr(&utt, 0));
    match s {
        AstNode::Struct { fields } => assert_eq!(fields.len(), 2),
        _ => panic!("Expected Struct"),
    }
}

#[test]
fn tg_st_002_list_float32() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_();
    e.list_of_float32(&[1.0, 2.0, 3.0]);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let lst = inner_expression(body_expr(&utt, 0));
    match lst {
        AstNode::List { count, elements } => {
            assert_eq!(*count, 3);
            assert_eq!(elements.len(), 3);
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn tg_st_003_nested_struct_in_list() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_();
    e.begin_list(2);
    e.begin_struct().field(0x0000).int32(1).end_struct();
    e.begin_struct().field(0x0000).int32(2).end_struct();
    e.end_list();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let lst = inner_expression(body_expr(&utt, 0));
    match lst {
        AstNode::List { count, elements } => {
            assert_eq!(*count, 2);
            assert!(matches!(&elements[0], AstNode::Struct { .. }));
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn tg_st_004_map() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_();
    e.begin_map(2);
    e.string("x").float32(1.0);
    e.string("y").float32(2.0);
    e.end_map();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let m = inner_expression(body_expr(&utt, 0));
    match m {
        AstNode::Map { count, pairs } => {
            assert_eq!(*count, 2);
            assert_eq!(pairs.len(), 2);
        }
        _ => panic!("Expected Map"),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TG-EXPR: Expression Tests (6 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_ex_001_assert_wraps() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().int32(42);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    match body_expr(&utt, 0) {
        AstNode::Pragmatic { act, .. } => assert_eq!(act, "ASSERT"),
        _ => panic!("Expected Pragmatic"),
    }
}

#[test]
fn tg_ex_002_query() {
    let mut e = AILLEncoder::new();
    e.start_utterance().query().l1_ref(0x0000);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    match body_expr(&utt, 0) {
        AstNode::Pragmatic { act, .. } => assert_eq!(act, "QUERY"),
        _ => panic!("Expected Pragmatic"),
    }
}

#[test]
fn tg_ex_003_observed_modality() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().observed().float32(1.5);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let inner = inner_expression(body_expr(&utt, 0));
    match inner {
        AstNode::Modal { modality, .. } => assert_eq!(modality, "OBSERVED"),
        _ => panic!("Expected Modal"),
    }
}

#[test]
fn tg_ex_004_predicted_with_horizon() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().predicted(500.0).float32(2.0);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let inner = inner_expression(body_expr(&utt, 0));
    match inner {
        AstNode::Modal { modality, extra, .. } => {
            assert_eq!(modality, "PREDICTED");
            assert!((extra.unwrap() - 500.0).abs() < 1.0);
        }
        _ => panic!("Expected Modal"),
    }
}

#[test]
fn tg_ex_005_past_temporal() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().temporal(temporal::PAST).float32(5.0);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let inner = inner_expression(body_expr(&utt, 0));
    match inner {
        AstNode::Temporal { modifier, .. } => assert_eq!(modifier, "PAST"),
        _ => panic!("Expected Temporal"),
    }
}

#[test]
fn tg_ex_006_l1_domain_ref() {
    let mut e = AILLEncoder::new();
    e.start_utterance().assert_().l1_ref(0x0090);
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let ref_node = inner_expression(body_expr(&utt, 0));
    match ref_node {
        AstNode::DomainRef { level, domain_code } => {
            assert_eq!(*level, 1);
            assert_eq!(*domain_code, 0x0090);
        }
        _ => panic!("Expected DomainRef"),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TG-META: Meta Header Tests (2 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_mt_001_confidence_priority_timestamp() {
    let mut e = AILLEncoder::new();
    e.start_utterance_with(0.85, 6, Some(12345678), None, None);
    e.assert_().null();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let m = get_meta(&utt);
    assert!((m.confidence - 0.85).abs() < 0.02);
    assert_eq!(m.priority, 6);
    assert_eq!(m.timestamp_us, 12345678);
}

#[test]
fn tg_mt_002_dest_agent_seqnum() {
    let dest: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut e = AILLEncoder::new();
    e.start_utterance_with(1.0, 3, Some(0), Some(&dest), Some(42));
    e.assert_().null();
    let wire = e.end_utterance();
    let utt = AILLDecoder::new().decode_utterance(&wire).unwrap();
    let m = get_meta(&utt);
    assert_eq!(m.dest_agent.as_deref(), Some(dest.as_slice()));
    assert_eq!(m.seqnum, Some(42));
}

// ═══════════════════════════════════════════════════════════════════════
// TG-CRC: CRC and Epoch Tests (4 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_crc_001_empty_vector() {
    assert_eq!(crc8(b""), 0x00);
}

#[test]
fn tg_crc_002_standard_vector() {
    assert_eq!(crc8(b"123456789"), 0xF4);
}

#[test]
fn tg_crc_003_epoch_roundtrip() {
    let mut eb = EpochBuilder::new();
    eb.write(b"Hello AILL");
    let epochs = eb.get_epochs();
    assert_eq!(epochs.len(), 1);
    let (decoded, _consumed) = decode_epoch(&epochs[0], 0).unwrap();
    assert!(decoded.crc_ok);
    assert_eq!(decoded.payload, b"Hello AILL");
}

#[test]
fn tg_crc_004_epoch_crc_failure() {
    let mut eb = EpochBuilder::new();
    eb.write(b"test data");
    let epochs = eb.get_epochs();
    let mut corrupted = epochs[0].clone();
    corrupted[5] ^= 0xFF; // corrupt a payload byte
    let (decoded, _) = decode_epoch(&corrupted, 0).unwrap();
    assert!(!decoded.crc_ok);
}

// ═══════════════════════════════════════════════════════════════════════
// TG-VARINT: Variable-Length Integer Tests (3 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_vi_001_1byte_values() {
    for v in [0u32, 1, 63, 127] {
        let encoded = encode_varint(v);
        assert_eq!(encoded.len(), 1);
        let (decoded, consumed) = decode_varint(&encoded, 0).unwrap();
        assert_eq!(decoded, v);
        assert_eq!(consumed, 1);
    }
}

#[test]
fn tg_vi_002_2byte_values() {
    for v in [128u32, 1000, 16383] {
        let encoded = encode_varint(v);
        assert_eq!(encoded.len(), 2);
        let (decoded, consumed) = decode_varint(&encoded, 0).unwrap();
        assert_eq!(decoded, v);
        assert_eq!(consumed, 2);
    }
}

#[test]
fn tg_vi_003_large_values() {
    for v in [16384u32, 100_000, 2_097_151, 268_435_455] {
        let encoded = encode_varint(v);
        let (decoded, _consumed) = decode_varint(&encoded, 0).unwrap();
        assert_eq!(decoded, v);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// TG-CODEC: Codebook Tests (3 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_cd_001_all_256_entries() {
    // All 256 base codebook entries should exist (non-"UNKNOWN" mnemonic)
    for code in 0..=255u8 {
        let entry = &BASE_CODEBOOK[code as usize];
        assert_ne!(entry.mnemonic, "UNKNOWN", "Missing entry for 0x{:02X}", code);
    }
}

#[test]
fn tg_cd_002_nav1_codebook() {
    assert!(NAV1.lookup(0x0000).is_some());
    assert!(NAV1.lookup(0x0090).is_some());
    assert_eq!(NAV1.lookup(0x0000).unwrap().mnemonic, "POSITION_3D");
}

#[test]
fn tg_cd_003_diag1_codebook() {
    assert!(DIAG1.lookup(0x0000).is_some());
    assert_eq!(DIAG1.lookup(0x0000).unwrap().mnemonic, "BATTERY_LEVEL");
}

// ═══════════════════════════════════════════════════════════════════════
// TG-ERR: Error Handling Tests (3 tests)
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn tg_er_001_missing_start_utterance() {
    let result = AILLDecoder::new().decode_utterance(&[0x81, 0x01]);
    assert!(result.is_err());
}

#[test]
fn tg_er_002_truncated_data() {
    let result = AILLDecoder::new().decode_utterance(&[0x00, 0x90]);
    assert!(result.is_err());
}

#[test]
fn tg_er_003_insufficient_epoch_data() {
    let result = decode_epoch(&[0x00], 0);
    assert!(result.is_err());
}
