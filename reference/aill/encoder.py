"""
AILL Reference Implementation - Wire Format Encoder
Acoustic Inter-agent Linguistic Link v1.1

Encodes AILL semantic expressions into the wire byte format
specified in Part IV of the supplemental specification.
"""

import struct
import time
import math
from typing import Union, Optional
from dataclasses import dataclass, field

from .codebook import (
    FrameControl, TypeMarker, Structure, Quantifier, Logic, Relational,
    Temporal, Modality, Pragmatic, Meta, Arithmetic, Escape,
    BINARY_OPS, UNARY_OPS, TERNARY_OPS,
)


# ═══════════════════════════════════════════════════════════════════════
# CRC-8 Implementation (CRC-8/CCITT)
# ═══════════════════════════════════════════════════════════════════════

_CRC8_TABLE = []
for i in range(256):
    crc = i
    for _ in range(8):
        if crc & 0x80:
            crc = (crc << 1) ^ 0x07
        else:
            crc <<= 1
        crc &= 0xFF
    _CRC8_TABLE.append(crc)

def crc8(data: bytes) -> int:
    """Compute CRC-8/CCITT over a byte sequence."""
    crc = 0x00
    for b in data:
        crc = _CRC8_TABLE[crc ^ b]
    return crc


# ═══════════════════════════════════════════════════════════════════════
# Float16 helpers (IEEE 754 binary16)
# ═══════════════════════════════════════════════════════════════════════

def encode_float16(value: float) -> bytes:
    """Encode a Python float as IEEE 754 binary16 (2 bytes, big-endian)."""
    return struct.pack('>e', value)

def decode_float16(data: bytes) -> float:
    """Decode 2 bytes (big-endian) as IEEE 754 binary16."""
    return struct.unpack('>e', data)[0]

def encode_float32(value: float) -> bytes:
    return struct.pack('>f', value)

def decode_float32(data: bytes) -> float:
    return struct.unpack('>f', data)[0]

def encode_float64(value: float) -> bytes:
    return struct.pack('>d', value)

def decode_float64(data: bytes) -> float:
    return struct.unpack('>d', data)[0]


# ═══════════════════════════════════════════════════════════════════════
# VarInt encoding
# ═══════════════════════════════════════════════════════════════════════

def encode_varint(value: int) -> bytes:
    """Encode a non-negative integer as a variable-length integer."""
    if value < 0:
        raise ValueError("VarInt does not support negative values")
    if value < 128:
        return bytes([value])
    elif value < 16384:
        return bytes([0x80 | (value >> 8), value & 0xFF])
    elif value < 2097152:
        return bytes([0xC0 | (value >> 16), (value >> 8) & 0xFF, value & 0xFF])
    elif value < 268435456:
        return bytes([0xE0 | (value >> 24), (value >> 16) & 0xFF,
                      (value >> 8) & 0xFF, value & 0xFF])
    else:
        return bytes([0xF0]) + struct.pack('>I', value)

def decode_varint(data: bytes, offset: int = 0) -> tuple[int, int]:
    """Decode a VarInt, returning (value, bytes_consumed)."""
    first = data[offset]
    if first < 0x80:
        return first, 1
    elif first < 0xC0:
        val = ((first & 0x3F) << 8) | data[offset + 1]
        return val, 2
    elif first < 0xE0:
        val = ((first & 0x1F) << 16) | (data[offset + 1] << 8) | data[offset + 2]
        return val, 3
    elif first < 0xF0:
        val = ((first & 0x0F) << 24) | (data[offset + 1] << 16) | \
              (data[offset + 2] << 8) | data[offset + 3]
        return val, 4
    else:
        val = struct.unpack('>I', data[offset + 1:offset + 5])[0]
        return val, 5


# ═══════════════════════════════════════════════════════════════════════
# Byte Stream Builder
# ═══════════════════════════════════════════════════════════════════════

class ByteStream:
    """Builds a byte sequence by appending typed values."""

    def __init__(self):
        self._buf = bytearray()

    def write_byte(self, val: int) -> 'ByteStream':
        self._buf.append(val & 0xFF)
        return self

    def write_uint8(self, val: int) -> 'ByteStream':
        self._buf.append(val & 0xFF)
        return self

    def write_int8(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>b', val))
        return self

    def write_uint16(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>H', val))
        return self

    def write_int16(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>h', val))
        return self

    def write_uint32(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>I', val))
        return self

    def write_int32(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>i', val))
        return self

    def write_int64(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>q', val))
        return self

    def write_uint64(self, val: int) -> 'ByteStream':
        self._buf.extend(struct.pack('>Q', val))
        return self

    def write_float16(self, val: float) -> 'ByteStream':
        self._buf.extend(encode_float16(val))
        return self

    def write_float32(self, val: float) -> 'ByteStream':
        self._buf.extend(encode_float32(val))
        return self

    def write_float64(self, val: float) -> 'ByteStream':
        self._buf.extend(encode_float64(val))
        return self

    def write_string(self, val: str) -> 'ByteStream':
        encoded = val.encode('utf-8')
        self.write_uint16(len(encoded))
        self._buf.extend(encoded)
        return self

    def write_bytes_val(self, val: bytes) -> 'ByteStream':
        self.write_uint16(len(val))
        self._buf.extend(val)
        return self

    def write_uuid(self, val: bytes) -> 'ByteStream':
        """Write a 128-bit UUID (16 bytes)."""
        assert len(val) == 16, "UUID must be 16 bytes"
        self._buf.extend(val)
        return self

    def write_varint(self, val: int) -> 'ByteStream':
        self._buf.extend(encode_varint(val))
        return self

    def write_raw(self, data: bytes) -> 'ByteStream':
        self._buf.extend(data)
        return self

    def to_bytes(self) -> bytes:
        return bytes(self._buf)

    def __len__(self):
        return len(self._buf)


# ═══════════════════════════════════════════════════════════════════════
# Epoch Builder
# ═══════════════════════════════════════════════════════════════════════

MAX_EPOCH_PAYLOAD = 8192

class EpochBuilder:
    """Builds epochs with sequence numbers and CRC-8 checksums."""

    def __init__(self):
        self._seq = 0
        self._epochs: list[bytes] = []
        self._current_payload = ByteStream()

    def write(self, data: bytes):
        """Add data to the current epoch. Flushes if size exceeded."""
        if len(self._current_payload) + len(data) > MAX_EPOCH_PAYLOAD:
            self.flush()
        self._current_payload.write_raw(data)

    def flush(self):
        """Finalize the current epoch and start a new one."""
        if len(self._current_payload) == 0:
            return
        payload = self._current_payload.to_bytes()
        epoch = ByteStream()
        epoch.write_uint16(self._seq)
        epoch.write_uint16(len(payload))
        epoch.write_raw(payload)
        # CRC-8 over (seq + length + payload)
        epoch_bytes = epoch.to_bytes()
        checksum = crc8(epoch_bytes)
        epoch.write_uint8(checksum)
        self._epochs.append(epoch.to_bytes())
        self._seq += 1
        self._current_payload = ByteStream()

    def get_epochs(self) -> list[bytes]:
        self.flush()
        return self._epochs

    @property
    def epoch_count(self):
        return len(self._epochs) + (1 if len(self._current_payload) > 0 else 0)


# ═══════════════════════════════════════════════════════════════════════
# High-Level Encoder (Expression → Bytes)
# ═══════════════════════════════════════════════════════════════════════

@dataclass
class AILLValue:
    """Represents a typed AILL value for encoding."""
    type_code: int       # TypeMarker enum value
    value: any           # Python value
    semantic_tag: Optional[int] = None  # Optional domain codebook code


class AILLEncoder:
    """
    Encodes AILL utterances into wire format bytes.
    
    Usage:
        encoder = AILLEncoder(agent_uuid=my_uuid)
        data = (encoder
            .start_utterance(confidence=0.95, priority=5)
            .pragma(Pragmatic.ASSERT)
            .begin_struct()
            .field(0x0000)  # NAV-1: POSITION_3D
            .list_of_float32([3.5, 7.2, 0.0])
            .end_struct()
            .end_utterance())
    """

    def __init__(self, agent_uuid: bytes = b'\x00' * 16):
        self._uuid = agent_uuid
        self._stream = ByteStream()
        self._in_utterance = False

    def _code(self, code: int) -> 'AILLEncoder':
        self._stream.write_byte(code)
        return self

    def start_utterance(self, confidence: float = 1.0, priority: int = 3,
                        timestamp_us: Optional[int] = None,
                        dest_agent: Optional[bytes] = None,
                        seqnum: Optional[int] = None) -> 'AILLEncoder':
        """Begin a new utterance with mandatory meta header."""
        if timestamp_us is None:
            timestamp_us = int(time.time() * 1_000_000)

        self._code(FrameControl.START_UTTERANCE)

        # Mandatory meta header: CONFIDENCE, PRIORITY, TIMESTAMP
        self._code(Meta.CONFIDENCE)
        self._stream.write_float16(confidence)
        self._code(Meta.PRIORITY)
        self._stream.write_uint8(priority)
        self._code(Meta.TIMESTAMP_META)
        self._stream.write_int64(timestamp_us)

        # Optional meta fields
        if dest_agent is not None:
            self._code(Meta.DEST_AGENT)
            self._stream.write_uuid(dest_agent)
        if seqnum is not None:
            self._code(Meta.SEQNUM)
            self._stream.write_uint32(seqnum)

        self._in_utterance = True
        return self

    def end_utterance(self) -> bytes:
        """End the utterance and return the complete wire bytes."""
        self._code(FrameControl.END_UTTERANCE)
        self._in_utterance = False
        return self._stream.to_bytes()

    # ── Pragmatic acts ──
    def pragma(self, act: Pragmatic) -> 'AILLEncoder':
        return self._code(act)

    def query(self) -> 'AILLEncoder':
        return self._code(Pragmatic.QUERY)

    def assert_(self) -> 'AILLEncoder':
        return self._code(Pragmatic.ASSERT)

    def request(self) -> 'AILLEncoder':
        return self._code(Pragmatic.REQUEST)

    def command(self) -> 'AILLEncoder':
        return self._code(Pragmatic.COMMAND)

    def acknowledge(self) -> 'AILLEncoder':
        return self._code(Pragmatic.ACKNOWLEDGE)

    def warn(self) -> 'AILLEncoder':
        return self._code(Pragmatic.WARN)

    # ── Modality ──
    def modality(self, mod: Modality) -> 'AILLEncoder':
        return self._code(mod)

    def observed(self) -> 'AILLEncoder':
        return self._code(Modality.OBSERVED)

    def inferred(self) -> 'AILLEncoder':
        return self._code(Modality.INFERRED)

    def predicted(self, horizon_ms: float) -> 'AILLEncoder':
        self._code(Modality.PREDICTED)
        self._stream.write_float16(horizon_ms)
        return self

    # ── Temporal ──
    def temporal(self, mod: Temporal) -> 'AILLEncoder':
        return self._code(mod)

    # ── Structure ──
    def begin_struct(self) -> 'AILLEncoder':
        return self._code(Structure.BEGIN_STRUCT)

    def end_struct(self) -> 'AILLEncoder':
        return self._code(Structure.END_STRUCT)

    def field(self, field_code: int) -> 'AILLEncoder':
        self._code(Structure.FIELD_ID)
        self._stream.write_uint16(field_code)
        return self

    def begin_list(self, count: int) -> 'AILLEncoder':
        self._code(Structure.BEGIN_LIST)
        self._stream.write_uint16(count)
        return self

    def end_list(self) -> 'AILLEncoder':
        return self._code(Structure.END_LIST)

    def begin_map(self, count: int) -> 'AILLEncoder':
        self._code(Structure.BEGIN_MAP)
        self._stream.write_uint16(count)
        return self

    def end_map(self) -> 'AILLEncoder':
        return self._code(Structure.END_MAP)

    # ── Typed values ──
    def int8(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_INT8)
        self._stream.write_int8(val)
        return self

    def int16(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_INT16)
        self._stream.write_int16(val)
        return self

    def int32(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_INT32)
        self._stream.write_int32(val)
        return self

    def int64(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_INT64)
        self._stream.write_int64(val)
        return self

    def uint8(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_UINT8)
        self._stream.write_uint8(val)
        return self

    def uint16(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_UINT16)
        self._stream.write_uint16(val)
        return self

    def uint32(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_UINT32)
        self._stream.write_uint32(val)
        return self

    def float16(self, val: float) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_FLOAT16)
        self._stream.write_float16(val)
        return self

    def float32(self, val: float) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_FLOAT32)
        self._stream.write_float32(val)
        return self

    def float64(self, val: float) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_FLOAT64)
        self._stream.write_float64(val)
        return self

    def bool_(self, val: bool) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_BOOL)
        self._stream.write_uint8(0x01 if val else 0x00)
        return self

    def string(self, val: str) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_STRING)
        self._stream.write_string(val)
        return self

    def null(self) -> 'AILLEncoder':
        return self._code(TypeMarker.TYPE_NULL)

    def timestamp(self, val: int) -> 'AILLEncoder':
        self._code(TypeMarker.TYPE_TIMESTAMP)
        self._stream.write_int64(val)
        return self

    # ── Convenience: typed lists ──
    def list_of_float32(self, values: list[float]) -> 'AILLEncoder':
        """Encode a list of float32 values."""
        self.begin_list(len(values))
        for v in values:
            self.float32(v)
        self.end_list()
        return self

    def list_of_int32(self, values: list[int]) -> 'AILLEncoder':
        self.begin_list(len(values))
        for v in values:
            self.int32(v)
        self.end_list()
        return self

    # ── Domain codebook references ──
    def l1_ref(self, code: int) -> 'AILLEncoder':
        """Reference a Level 1 (standard domain) codebook entry."""
        self._code(Escape.ESCAPE_L1)
        self._stream.write_uint16(code)
        return self

    def l2_ref(self, code: int) -> 'AILLEncoder':
        """Reference a Level 2 (session-negotiated) codebook entry."""
        self._code(Escape.ESCAPE_L2)
        self._stream.write_uint16(code)
        return self

    def l3_ref(self, code: int) -> 'AILLEncoder':
        """Reference a Level 3 (ephemeral) codebook entry."""
        self._code(Escape.ESCAPE_L3)
        self._stream.write_uint16(code)
        return self

    # ── Operators ──
    def op(self, opcode: int) -> 'AILLEncoder':
        """Emit any operator code."""
        return self._code(opcode)

    def add(self) -> 'AILLEncoder': return self._code(Arithmetic.ADD)
    def sub(self) -> 'AILLEncoder': return self._code(Arithmetic.SUB)
    def mul(self) -> 'AILLEncoder': return self._code(Arithmetic.MUL)
    def div(self) -> 'AILLEncoder': return self._code(Arithmetic.DIV)
    def distance(self) -> 'AILLEncoder': return self._code(Arithmetic.DISTANCE)
    def norm(self) -> 'AILLEncoder': return self._code(Arithmetic.NORM)
    def eq(self) -> 'AILLEncoder': return self._code(Relational.EQ)
    def lt(self) -> 'AILLEncoder': return self._code(Relational.LT)
    def gt(self) -> 'AILLEncoder': return self._code(Relational.GT)

    # ── Quantifiers ──
    def forall(self) -> 'AILLEncoder': return self._code(Quantifier.FORALL)
    def exists(self) -> 'AILLEncoder': return self._code(Quantifier.EXISTS)

    # ── Annotation ──
    def confidence(self, val: float) -> 'AILLEncoder':
        self._code(Meta.CONFIDENCE)
        self._stream.write_float16(val)
        return self

    def label(self, text: str) -> 'AILLEncoder':
        self._code(Meta.LABEL)
        self._stream.write_string(text)
        return self

    def context_ref(self, sct_index: int) -> 'AILLEncoder':
        self._code(Meta.CONTEXT_REF)
        self._stream.write_varint(sct_index)
        return self

    # ── Raw byte access ──
    def raw(self, data: bytes) -> 'AILLEncoder':
        self._stream.write_raw(data)
        return self

    @property
    def current_size(self) -> int:
        return len(self._stream)
