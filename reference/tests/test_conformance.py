#!/usr/bin/env python3
"""
AILL Conformance Test Suite (ACTS) - Core Tests
Tests the reference implementation against the specification.
"""

import sys, os, struct, math
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from aill import (
    AILLEncoder, AILLDecoder, pretty_print,
    FrameControl, TypeMarker, Structure, Pragmatic, Meta, Modality,
    Temporal, Arithmetic, Escape, Relational, Logic, Quantifier,
    BASE_CODEBOOK, NAV1, DIAG1, DOMAIN_REGISTRY, crc8,
)
from aill.encoder import encode_float16, decode_float16, encode_varint, decode_varint, ByteStream, EpochBuilder
from aill.decoder import AILLDecodeError, decode_epoch

passed = 0
failed = 0
errors = 0

def run_test(test_id, description, func):
    global passed, failed, errors
    try:
        result = func()
        if result:
            passed += 1
            status = "PASS"
        else:
            failed += 1
            status = "FAIL"
    except Exception as e:
        errors += 1
        status = f"ERROR: {e}"
    print(f"  {test_id:20s}  [{status:>6s}]  {description}")

# ═══════════════════════════════════════════════════════════════════════
# TG-TYPES: Type System Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_ty_001():
    """INT8 encode/decode round-trip"""
    for v in [-128, -1, 0, 1, 127]:
        e = AILLEncoder()
        e.start_utterance(confidence=1.0, priority=3).assert_().int8(v)
        wire = e.end_utterance()
        d = AILLDecoder()
        utt = d.decode_utterance(wire)
        lit = utt.body[0].expression
        if lit.value != v: return False
    return True

def tg_ty_002():
    """INT32 encode/decode"""
    for v in [-2147483648, 0, 2147483647]:
        e = AILLEncoder()
        e.start_utterance().assert_().int32(v)
        wire = e.end_utterance()
        utt = AILLDecoder().decode_utterance(wire)
        if utt.body[0].expression.value != v: return False
    return True

def tg_ty_003():
    """FLOAT32 encode/decode including special values"""
    for v in [0.0, -0.0, 1.5, -1.5, float('inf'), float('-inf')]:
        e = AILLEncoder()
        e.start_utterance().assert_().float32(v)
        wire = e.end_utterance()
        utt = AILLDecoder().decode_utterance(wire)
        result = utt.body[0].expression.value
        if math.isinf(v):
            if not math.isinf(result) or (v > 0) != (result > 0): return False
        elif result != v:
            return False
    # NaN
    e = AILLEncoder()
    e.start_utterance().assert_().float32(float('nan'))
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return math.isnan(utt.body[0].expression.value)

def tg_ty_004():
    """FLOAT16 encode/decode"""
    e = AILLEncoder()
    e.start_utterance().assert_().float16(0.5)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return abs(utt.body[0].expression.value - 0.5) < 0.01

def tg_ty_005():
    """STRING encode/decode with UTF-8"""
    test_str = "Hello AILL! 🤖"
    e = AILLEncoder()
    e.start_utterance().assert_().string(test_str)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.body[0].expression.value == test_str

def tg_ty_006():
    """Empty string"""
    e = AILLEncoder()
    e.start_utterance().assert_().string("")
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.body[0].expression.value == ""

def tg_ty_007():
    """BOOL encode/decode"""
    for v in [True, False]:
        e = AILLEncoder()
        e.start_utterance().assert_().bool_(v)
        wire = e.end_utterance()
        utt = AILLDecoder().decode_utterance(wire)
        if utt.body[0].expression.value != v: return False
    return True

def tg_ty_008():
    """NULL type"""
    e = AILLEncoder()
    e.start_utterance().assert_().null()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.body[0].expression.value is None

def tg_ty_009():
    """TIMESTAMP encode/decode"""
    ts = 1740000000000000
    e = AILLEncoder()
    e.start_utterance().assert_().timestamp(ts)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.body[0].expression.value == ts

def tg_ty_010():
    """UINT16 and UINT32"""
    e = AILLEncoder()
    e.start_utterance().assert_().uint16(65535).uint32(4294967295)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return True  # Basic parse success

# ═══════════════════════════════════════════════════════════════════════
# TG-STRUCT: Structure Tests  
# ═══════════════════════════════════════════════════════════════════════

def tg_st_001():
    """Simple struct with fields"""
    e = AILLEncoder()
    e.start_utterance().assert_()
    e.begin_struct()
    e.field(0x0000).float32(3.5)
    e.field(0x0001).float32(7.2)
    e.end_struct()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    s = utt.body[0].expression
    return s.node_type == "struct" and len(s.fields) == 2

def tg_st_002():
    """List of float32"""
    e = AILLEncoder()
    e.start_utterance().assert_()
    e.list_of_float32([1.0, 2.0, 3.0])
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    lst = utt.body[0].expression
    return (lst.node_type == "list" and lst.count == 3 and
            len(lst.elements) == 3)

def tg_st_003():
    """Nested struct inside list"""
    e = AILLEncoder()
    e.start_utterance().assert_()
    e.begin_list(2)
    e.begin_struct().field(0x0000).int32(1).end_struct()
    e.begin_struct().field(0x0000).int32(2).end_struct()
    e.end_list()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    lst = utt.body[0].expression
    return (lst.node_type == "list" and lst.count == 2 and
            lst.elements[0].node_type == "struct")

def tg_st_004():
    """Map with key-value pairs"""
    e = AILLEncoder()
    e.start_utterance().assert_()
    e.begin_map(2)
    e.string("x").float32(1.0)
    e.string("y").float32(2.0)
    e.end_map()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    m = utt.body[0].expression
    return m.node_type == "map" and m.count == 2 and len(m.pairs) == 2

# ═══════════════════════════════════════════════════════════════════════
# TG-EXPR: Expression Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_ex_001():
    """Pragmatic: ASSERT wraps expression"""
    e = AILLEncoder()
    e.start_utterance().assert_().int32(42)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return (utt.body[0].node_type == "pragmatic" and
            utt.body[0].act == "ASSERT")

def tg_ex_002():
    """Pragmatic: QUERY"""
    e = AILLEncoder()
    e.start_utterance().query().l1_ref(0x0000)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.body[0].act == "QUERY"

def tg_ex_003():
    """Modality: OBSERVED wraps expression"""
    e = AILLEncoder()
    e.start_utterance().assert_().observed().float32(1.5)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    inner = utt.body[0].expression
    return inner.node_type == "modal" and inner.modality == "OBSERVED"

def tg_ex_004():
    """Modality: PREDICTED with horizon"""
    e = AILLEncoder()
    e.start_utterance().assert_().predicted(500.0).float32(2.0)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    inner = utt.body[0].expression
    return (inner.modality == "PREDICTED" and
            abs(inner.extra - 500.0) < 1.0)

def tg_ex_005():
    """Temporal: PAST modifier"""
    e = AILLEncoder()
    e.start_utterance().assert_().temporal(Temporal.PAST).float32(5.0)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    inner = utt.body[0].expression
    return inner.node_type == "temporal" and inner.modifier == "PAST"

def tg_ex_006():
    """Domain reference: L1"""
    e = AILLEncoder()
    e.start_utterance().assert_().l1_ref(0x0090)
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    ref = utt.body[0].expression
    return ref.node_type == "domain_ref" and ref.level == 1 and ref.domain_code == 0x0090

# ═══════════════════════════════════════════════════════════════════════
# TG-META: Meta Header Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_mt_001():
    """Meta header: confidence, priority, timestamp parsed"""
    e = AILLEncoder()
    e.start_utterance(confidence=0.85, priority=6, timestamp_us=12345678)
    e.assert_().null()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    m = utt.meta
    return (abs(m.confidence - 0.85) < 0.02 and
            m.priority == 6 and
            m.timestamp_us == 12345678)

def tg_mt_002():
    """Meta header: dest_agent and seqnum"""
    dest = bytes(range(16))
    e = AILLEncoder()
    e.start_utterance(dest_agent=dest, seqnum=42).assert_().null()
    wire = e.end_utterance()
    utt = AILLDecoder().decode_utterance(wire)
    return utt.meta.dest_agent == dest and utt.meta.seqnum == 42

# ═══════════════════════════════════════════════════════════════════════
# TG-CRC: CRC and Epoch Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_crc_001():
    """CRC-8 known vector"""
    # CRC-8/CCITT of empty = 0x00
    return crc8(b'') == 0x00

def tg_crc_002():
    """CRC-8 known vector: "123456789" """
    # Standard test vector for CRC-8/CCITT
    return crc8(b'123456789') == 0xF4

def tg_crc_003():
    """Epoch encode/decode with CRC verification"""
    eb = EpochBuilder()
    eb.write(b'Hello AILL')
    epochs = eb.get_epochs()
    assert len(epochs) == 1
    decoded, consumed = decode_epoch(epochs[0])
    return decoded.crc_ok and decoded.payload == b'Hello AILL'

def tg_crc_004():
    """Epoch CRC failure detection"""
    eb = EpochBuilder()
    eb.write(b'test data')
    epochs = eb.get_epochs()
    # Corrupt the payload
    corrupted = bytearray(epochs[0])
    corrupted[5] ^= 0xFF
    decoded, _ = decode_epoch(bytes(corrupted))
    return not decoded.crc_ok

# ═══════════════════════════════════════════════════════════════════════
# TG-VARINT: Variable-Length Integer Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_vi_001():
    """VarInt: 1-byte values (0-127)"""
    for v in [0, 1, 63, 127]:
        encoded = encode_varint(v)
        decoded, consumed = decode_varint(encoded)
        if decoded != v or consumed != 1: return False
    return True

def tg_vi_002():
    """VarInt: 2-byte values (128-16383)"""
    for v in [128, 1000, 16383]:
        encoded = encode_varint(v)
        decoded, consumed = decode_varint(encoded)
        if decoded != v or consumed != 2: return False
    return True

def tg_vi_003():
    """VarInt: large values"""
    for v in [16384, 100000, 2097151, 268435455]:
        encoded = encode_varint(v)
        decoded, consumed = decode_varint(encoded)
        if decoded != v: return False
    return True

# ═══════════════════════════════════════════════════════════════════════
# TG-CODEC: Codebook Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_cd_001():
    """All 256 base codebook entries exist"""
    for code in range(256):
        if code not in BASE_CODEBOOK: return False
    return True

def tg_cd_002():
    """NAV-1 codebook has expected entries"""
    return (NAV1.lookup(0x0000) is not None and
            NAV1.lookup(0x0090) is not None and
            NAV1.lookup(0x0000).mnemonic == "POSITION_3D")

def tg_cd_003():
    """DIAG-1 codebook has expected entries"""
    return (DIAG1.lookup(0x0000) is not None and
            DIAG1.lookup(0x0000).mnemonic == "BATTERY_LEVEL")

# ═══════════════════════════════════════════════════════════════════════
# TG-ERR: Error Handling Tests
# ═══════════════════════════════════════════════════════════════════════

def tg_er_001():
    """Missing START_UTTERANCE raises error"""
    try:
        AILLDecoder().decode_utterance(bytes([0x81, 0x01]))
        return False
    except AILLDecodeError:
        return True

def tg_er_002():
    """Truncated data raises error"""
    try:
        AILLDecoder().decode_utterance(bytes([0x00, 0x90]))
        return False
    except (AILLDecodeError, Exception):
        return True

def tg_er_003():
    """Epoch with insufficient data"""
    try:
        decode_epoch(bytes([0x00]))
        return False
    except AILLDecodeError:
        return True

# ═══════════════════════════════════════════════════════════════════════
# RUN ALL TESTS
# ═══════════════════════════════════════════════════════════════════════

def main():
    print("""
    ╔═══════════════════════════════════════════════════════════╗
    ║   AILL Conformance Test Suite (ACTS)                     ║
    ║   Reference Implementation v1.1                          ║
    ╚═══════════════════════════════════════════════════════════╝
    """)

    print("  TG-TYPES: Type System")
    print("  " + "─" * 56)
    run_test("TG-TY-001", "INT8 round-trip (boundary values)", tg_ty_001)
    run_test("TG-TY-002", "INT32 round-trip (boundary values)", tg_ty_002)
    run_test("TG-TY-003", "FLOAT32 including special values", tg_ty_003)
    run_test("TG-TY-004", "FLOAT16 encode/decode", tg_ty_004)
    run_test("TG-TY-005", "STRING with UTF-8 (incl emoji)", tg_ty_005)
    run_test("TG-TY-006", "Empty string", tg_ty_006)
    run_test("TG-TY-007", "BOOL true/false", tg_ty_007)
    run_test("TG-TY-008", "NULL type", tg_ty_008)
    run_test("TG-TY-009", "TIMESTAMP int64", tg_ty_009)
    run_test("TG-TY-010", "UINT16 and UINT32", tg_ty_010)

    print("\n  TG-STRUCT: Structure Types")
    print("  " + "─" * 56)
    run_test("TG-ST-001", "Simple struct with fields", tg_st_001)
    run_test("TG-ST-002", "List of float32", tg_st_002)
    run_test("TG-ST-003", "Nested struct inside list", tg_st_003)
    run_test("TG-ST-004", "Map with key-value pairs", tg_st_004)

    print("\n  TG-EXPR: Expression Parsing")
    print("  " + "─" * 56)
    run_test("TG-EX-001", "ASSERT wraps expression", tg_ex_001)
    run_test("TG-EX-002", "QUERY pragmatic act", tg_ex_002)
    run_test("TG-EX-003", "OBSERVED modality", tg_ex_003)
    run_test("TG-EX-004", "PREDICTED with horizon", tg_ex_004)
    run_test("TG-EX-005", "PAST temporal modifier", tg_ex_005)
    run_test("TG-EX-006", "L1 domain reference", tg_ex_006)

    print("\n  TG-META: Meta Header")
    print("  " + "─" * 56)
    run_test("TG-MT-001", "Confidence/priority/timestamp", tg_mt_001)
    run_test("TG-MT-002", "Dest agent and seqnum", tg_mt_002)

    print("\n  TG-CRC: CRC and Epoch Integrity")
    print("  " + "─" * 56)
    run_test("TG-CRC-001", "CRC-8 empty vector", tg_crc_001)
    run_test("TG-CRC-002", "CRC-8 standard test vector", tg_crc_002)
    run_test("TG-CRC-003", "Epoch encode/decode with CRC", tg_crc_003)
    run_test("TG-CRC-004", "Epoch CRC failure detection", tg_crc_004)

    print("\n  TG-VARINT: Variable-Length Integers")
    print("  " + "─" * 56)
    run_test("TG-VI-001", "1-byte values (0-127)", tg_vi_001)
    run_test("TG-VI-002", "2-byte values (128-16383)", tg_vi_002)
    run_test("TG-VI-003", "Large values (16K+)", tg_vi_003)

    print("\n  TG-CODEC: Codebook")
    print("  " + "─" * 56)
    run_test("TG-CD-001", "All 256 base codebook entries", tg_cd_001)
    run_test("TG-CD-002", "NAV-1 domain codebook", tg_cd_002)
    run_test("TG-CD-003", "DIAG-1 domain codebook", tg_cd_003)

    print("\n  TG-ERR: Error Handling")
    print("  " + "─" * 56)
    run_test("TG-ER-001", "Missing START_UTTERANCE", tg_er_001)
    run_test("TG-ER-002", "Truncated data", tg_er_002)
    run_test("TG-ER-003", "Insufficient epoch data", tg_er_003)

    # Summary
    total = passed + failed + errors
    print(f"\n  {'=' * 56}")
    print(f"  Results: {passed} PASSED / {failed} FAILED / {errors} ERRORS  ({total} total)")
    print(f"  {'=' * 56}")
    
    if failed == 0 and errors == 0:
        print(f"\n  ✓ ALL TESTS PASSED - Implementation conforms to AILL v1.1")
    else:
        print(f"\n  ✗ CONFORMANCE FAILURE - {failed + errors} test(s) did not pass")
    
    return 0 if (failed == 0 and errors == 0) else 1

if __name__ == "__main__":
    sys.exit(main())

# ═══════════════════════════════════════════════════════════════════════
# Pytest-compatible wrappers
# ═══════════════════════════════════════════════════════════════════════

import pytest

_ALL_TESTS = [
    ("TG-TY-001", "INT8 round-trip", tg_ty_001),
    ("TG-TY-002", "INT32 round-trip", tg_ty_002),
    ("TG-TY-003", "FLOAT32 special values", tg_ty_003),
    ("TG-TY-004", "FLOAT16 encode/decode", tg_ty_004),
    ("TG-TY-005", "STRING UTF-8", tg_ty_005),
    ("TG-TY-006", "Empty string", tg_ty_006),
    ("TG-TY-007", "BOOL", tg_ty_007),
    ("TG-TY-008", "NULL", tg_ty_008),
    ("TG-TY-009", "TIMESTAMP", tg_ty_009),
    ("TG-TY-010", "UINT16/UINT32", tg_ty_010),
    ("TG-ST-001", "Simple struct", tg_st_001),
    ("TG-ST-002", "List of float32", tg_st_002),
    ("TG-ST-003", "Nested struct in list", tg_st_003),
    ("TG-ST-004", "Map", tg_st_004),
    ("TG-EX-001", "ASSERT wraps expression", tg_ex_001),
    ("TG-EX-002", "QUERY", tg_ex_002),
    ("TG-EX-003", "OBSERVED modality", tg_ex_003),
    ("TG-EX-004", "PREDICTED with horizon", tg_ex_004),
    ("TG-EX-005", "PAST temporal", tg_ex_005),
    ("TG-EX-006", "L1 domain reference", tg_ex_006),
    ("TG-MT-001", "Confidence/priority/timestamp", tg_mt_001),
    ("TG-MT-002", "Dest agent and seqnum", tg_mt_002),
    ("TG-CRC-001", "CRC-8 empty vector", tg_crc_001),
    ("TG-CRC-002", "CRC-8 standard vector", tg_crc_002),
    ("TG-CRC-003", "Epoch roundtrip", tg_crc_003),
    ("TG-CRC-004", "Epoch CRC failure", tg_crc_004),
    ("TG-VI-001", "VarInt 1-byte", tg_vi_001),
    ("TG-VI-002", "VarInt 2-byte", tg_vi_002),
    ("TG-VI-003", "VarInt large", tg_vi_003),
    ("TG-CD-001", "All 256 base entries", tg_cd_001),
    ("TG-CD-002", "NAV-1 codebook", tg_cd_002),
    ("TG-CD-003", "DIAG-1 codebook", tg_cd_003),
    ("TG-ER-001", "Missing START_UTTERANCE", tg_er_001),
    ("TG-ER-002", "Truncated data", tg_er_002),
    ("TG-ER-003", "Insufficient epoch data", tg_er_003),
]

@pytest.mark.parametrize("test_id,description,func", _ALL_TESTS, ids=[t[0] for t in _ALL_TESTS])
def test_conformance(test_id, description, func):
    assert func(), f"{test_id}: {description}"
