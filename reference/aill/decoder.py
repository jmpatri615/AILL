"""
AILL Reference Implementation - Wire Format Decoder
Acoustic Inter-agent Linguistic Link v1.1

Decodes AILL wire-format byte streams into structured expression trees.
"""

import struct
from dataclasses import dataclass, field
from typing import Any, Optional, Union
from enum import IntEnum

from .codebook import (
    FrameControl, TypeMarker, Structure, Quantifier, Logic, Relational,
    Temporal, Modality, Pragmatic, Meta, Arithmetic, Escape,
    BASE_CODEBOOK, BINARY_OPS, UNARY_OPS, TERNARY_OPS,
)
from .encoder import crc8, decode_float16, decode_float32, decode_float64, decode_varint


# ═══════════════════════════════════════════════════════════════════════
# AST Node Types
# ═══════════════════════════════════════════════════════════════════════

@dataclass
class ASTNode:
    """Base AST node for decoded AILL expressions."""
    node_type: str
    code: int = 0
    mnemonic: str = ""

@dataclass
class LiteralNode(ASTNode):
    """A typed literal value."""
    value_type: str = ""
    value: Any = None

@dataclass
class StructNode(ASTNode):
    """A struct with named fields."""
    fields: dict = field(default_factory=dict)

@dataclass
class ListNode(ASTNode):
    """A homogeneous list."""
    count: int = 0
    elements: list = field(default_factory=list)

@dataclass
class MapNode(ASTNode):
    """A key-value map."""
    count: int = 0
    pairs: list = field(default_factory=list)

@dataclass
class OperationNode(ASTNode):
    """An operation (operator + operands)."""
    operator: str = ""
    operands: list = field(default_factory=list)

@dataclass
class PragmaticNode(ASTNode):
    """A pragmatic act wrapping an expression."""
    act: str = ""
    expression: Any = None

@dataclass
class ModalNode(ASTNode):
    """A modality wrapper."""
    modality: str = ""
    expression: Any = None
    extra: Any = None  # e.g., horizon for PREDICTED

@dataclass
class TemporalNode(ASTNode):
    """A temporal modifier."""
    modifier: str = ""
    expression: Any = None

@dataclass
class MetaHeaderNode(ASTNode):
    """Decoded meta header."""
    confidence: float = 1.0
    priority: int = 3
    timestamp_us: int = 0
    source_agent: Optional[bytes] = None
    dest_agent: Optional[bytes] = None
    seqnum: Optional[int] = None
    annotations: dict = field(default_factory=dict)

@dataclass
class UtteranceNode(ASTNode):
    """A complete decoded utterance."""
    meta: MetaHeaderNode = None
    body: list = field(default_factory=list)

@dataclass 
class DomainRefNode(ASTNode):
    """Reference to a domain codebook entry."""
    level: int = 1
    domain_code: int = 0

@dataclass
class ContextRefNode(ASTNode):
    """Reference to a Session Context Table entry."""
    sct_index: int = 0


# ═══════════════════════════════════════════════════════════════════════
# Decoder Errors
# ═══════════════════════════════════════════════════════════════════════

class AILLDecodeError(Exception):
    """Raised when AILL data cannot be decoded."""
    def __init__(self, message: str, offset: int = -1):
        self.offset = offset
        super().__init__(f"[offset {offset}] {message}" if offset >= 0 else message)


# ═══════════════════════════════════════════════════════════════════════
# Epoch Decoder
# ═══════════════════════════════════════════════════════════════════════

@dataclass
class DecodedEpoch:
    """A decoded epoch with verified CRC."""
    seq_num: int
    payload: bytes
    crc_ok: bool


def decode_epoch(data: bytes, offset: int = 0) -> tuple[DecodedEpoch, int]:
    """
    Decode a single epoch from wire bytes.
    Returns (DecodedEpoch, bytes_consumed).
    """
    if len(data) - offset < 5:  # minimum: seq(2) + len(2) + crc(1)
        raise AILLDecodeError("Insufficient data for epoch header", offset)

    seq_num = struct.unpack('>H', data[offset:offset+2])[0]
    payload_len = struct.unpack('>H', data[offset+2:offset+4])[0]

    if len(data) - offset < 4 + payload_len + 1:
        raise AILLDecodeError(f"Incomplete epoch payload (expected {payload_len} bytes)", offset)

    payload = data[offset+4:offset+4+payload_len]
    received_crc = data[offset+4+payload_len]

    # Verify CRC over (seq + len + payload)
    computed_crc = crc8(data[offset:offset+4+payload_len])
    crc_ok = (received_crc == computed_crc)

    total_consumed = 4 + payload_len + 1
    return DecodedEpoch(seq_num, payload, crc_ok), total_consumed


# ═══════════════════════════════════════════════════════════════════════
# Main Decoder
# ═══════════════════════════════════════════════════════════════════════

class AILLDecoder:
    """
    Decodes AILL wire-format bytes into an AST.
    
    Usage:
        decoder = AILLDecoder()
        utterance = decoder.decode_utterance(wire_bytes)
    """

    def __init__(self):
        self._data: bytes = b''
        self._pos: int = 0

    def _remaining(self) -> int:
        return len(self._data) - self._pos

    def _peek(self) -> int:
        if self._pos >= len(self._data):
            raise AILLDecodeError("Unexpected end of data", self._pos)
        return self._data[self._pos]

    def _read_byte(self) -> int:
        if self._pos >= len(self._data):
            raise AILLDecodeError("Unexpected end of data", self._pos)
        val = self._data[self._pos]
        self._pos += 1
        return val

    def _read_bytes(self, n: int) -> bytes:
        if self._pos + n > len(self._data):
            raise AILLDecodeError(f"Need {n} bytes, only {self._remaining()} available", self._pos)
        val = self._data[self._pos:self._pos + n]
        self._pos += n
        return val

    def _read_uint8(self) -> int:
        return self._read_byte()

    def _read_int8(self) -> int:
        return struct.unpack('>b', self._read_bytes(1))[0]

    def _read_uint16(self) -> int:
        return struct.unpack('>H', self._read_bytes(2))[0]

    def _read_int16(self) -> int:
        return struct.unpack('>h', self._read_bytes(2))[0]

    def _read_uint32(self) -> int:
        return struct.unpack('>I', self._read_bytes(4))[0]

    def _read_int32(self) -> int:
        return struct.unpack('>i', self._read_bytes(4))[0]

    def _read_int64(self) -> int:
        return struct.unpack('>q', self._read_bytes(8))[0]

    def _read_uint64(self) -> int:
        return struct.unpack('>Q', self._read_bytes(8))[0]

    def _read_float16(self) -> float:
        return decode_float16(self._read_bytes(2))

    def _read_float32(self) -> float:
        return decode_float32(self._read_bytes(4))

    def _read_float64(self) -> float:
        return decode_float64(self._read_bytes(8))

    def _read_string(self) -> str:
        length = self._read_uint16()
        return self._read_bytes(length).decode('utf-8')

    def _read_uuid(self) -> bytes:
        return self._read_bytes(16)

    def _read_varint(self) -> int:
        val, consumed = decode_varint(self._data, self._pos)
        self._pos += consumed
        return val

    # ── Decode entry points ──

    def decode_utterance(self, data: bytes) -> UtteranceNode:
        """Decode a complete AILL utterance from wire bytes."""
        self._data = data
        self._pos = 0

        # Expect START_UTTERANCE
        code = self._read_byte()
        if code != FrameControl.START_UTTERANCE:
            raise AILLDecodeError(f"Expected START_UTTERANCE (0x00), got 0x{code:02X}", 0)

        # Decode meta header
        meta = self._decode_meta_header()

        # Decode body expressions until END_UTTERANCE
        body = []
        while self._pos < len(self._data):
            if self._peek() == FrameControl.END_UTTERANCE:
                self._read_byte()  # consume END_UTTERANCE
                break
            expr = self._decode_expression()
            if expr is not None:
                body.append(expr)

        return UtteranceNode(
            node_type="utterance",
            meta=meta,
            body=body,
        )

    def _decode_meta_header(self) -> MetaHeaderNode:
        """Decode the mandatory meta header."""
        meta = MetaHeaderNode(node_type="meta_header")

        # CONFIDENCE (mandatory)
        code = self._read_byte()
        if code != Meta.CONFIDENCE:
            raise AILLDecodeError(f"Expected CONFIDENCE (0x90), got 0x{code:02X}", self._pos - 1)
        meta.confidence = self._read_float16()

        # PRIORITY (mandatory)
        code = self._read_byte()
        if code != Meta.PRIORITY:
            raise AILLDecodeError(f"Expected PRIORITY (0x91), got 0x{code:02X}", self._pos - 1)
        meta.priority = self._read_uint8()

        # TIMESTAMP (mandatory)
        code = self._read_byte()
        if code != Meta.TIMESTAMP_META:
            raise AILLDecodeError(f"Expected TIMESTAMP_META (0x94), got 0x{code:02X}", self._pos - 1)
        meta.timestamp_us = self._read_int64()

        # Optional meta annotations
        while self._pos < len(self._data) and self._peek() in range(0x92, 0xA0):
            ann_code = self._read_byte()
            if ann_code == Meta.SOURCE_AGENT:
                meta.source_agent = self._read_uuid()
            elif ann_code == Meta.DEST_AGENT:
                meta.dest_agent = self._read_uuid()
            elif ann_code == Meta.SEQNUM:
                meta.seqnum = self._read_uint32()
            elif ann_code == Meta.TRACE_ID:
                meta.annotations['trace_id'] = self._read_uint64()
            elif ann_code == Meta.TTL:
                meta.annotations['ttl'] = self._read_uint16()
            elif ann_code == Meta.TOPIC:
                meta.annotations['topic'] = self._read_uint16()
            elif ann_code == Meta.VERSION_TAG:
                meta.annotations['version'] = (self._read_uint16(), self._read_uint16())
            else:
                break  # Unknown meta code, stop meta parsing

        return meta

    def _decode_expression(self) -> Optional[ASTNode]:
        """Decode a single expression from the current position."""
        if self._pos >= len(self._data):
            return None

        code = self._peek()

        # Pragmatic acts (0x80-0x8F)
        if 0x80 <= code <= 0x8F:
            return self._decode_pragmatic()

        # Modality (0x70-0x7F)
        if 0x70 <= code <= 0x7F:
            return self._decode_modal()

        # Temporal (0x60-0x6F)
        if 0x60 <= code <= 0x6F:
            return self._decode_temporal()

        # Meta annotations inline
        if code == Meta.CONFIDENCE or code == Meta.LABEL:
            return self._decode_annotation()

        # Type markers (literals)
        if 0x10 <= code <= 0x1F:
            return self._decode_literal()

        # Structure codes
        if code == Structure.BEGIN_STRUCT:
            return self._decode_struct()
        if code == Structure.BEGIN_LIST:
            return self._decode_list()
        if code == Structure.BEGIN_MAP:
            return self._decode_map()

        # Escape/domain refs
        if code in (Escape.ESCAPE_L1, Escape.ESCAPE_L2, Escape.ESCAPE_L3):
            return self._decode_domain_ref()

        # Context ref
        if code == Meta.CONTEXT_REF:
            self._read_byte()
            idx = self._read_varint()
            return ContextRefNode(node_type="context_ref", sct_index=idx)

        # NOP / COMMENT (skip)
        if code == Escape.NOP:
            self._read_byte()
            return None
        if code == Escape.COMMENT:
            self._read_byte()
            _comment = self._read_string()
            return None

        # Operators and other codes - just emit as-is
        self._read_byte()
        entry = BASE_CODEBOOK.get(code)
        mnemonic = entry.mnemonic if entry else f"UNKNOWN_0x{code:02X}"
        return ASTNode(node_type="code", code=code, mnemonic=mnemonic)

    def _decode_literal(self) -> LiteralNode:
        """Decode a typed literal value."""
        code = self._read_byte()

        if code == TypeMarker.TYPE_INT8:
            return LiteralNode(node_type="literal", code=code, value_type="int8", value=self._read_int8())
        elif code == TypeMarker.TYPE_INT16:
            return LiteralNode(node_type="literal", code=code, value_type="int16", value=self._read_int16())
        elif code == TypeMarker.TYPE_INT32:
            return LiteralNode(node_type="literal", code=code, value_type="int32", value=self._read_int32())
        elif code == TypeMarker.TYPE_INT64:
            return LiteralNode(node_type="literal", code=code, value_type="int64", value=self._read_int64())
        elif code == TypeMarker.TYPE_UINT8:
            return LiteralNode(node_type="literal", code=code, value_type="uint8", value=self._read_uint8())
        elif code == TypeMarker.TYPE_UINT16:
            return LiteralNode(node_type="literal", code=code, value_type="uint16", value=self._read_uint16())
        elif code == TypeMarker.TYPE_UINT32:
            return LiteralNode(node_type="literal", code=code, value_type="uint32", value=self._read_uint32())
        elif code == TypeMarker.TYPE_UINT64:
            return LiteralNode(node_type="literal", code=code, value_type="uint64", value=self._read_uint64())
        elif code == TypeMarker.TYPE_FLOAT16:
            return LiteralNode(node_type="literal", code=code, value_type="float16", value=self._read_float16())
        elif code == TypeMarker.TYPE_FLOAT32:
            return LiteralNode(node_type="literal", code=code, value_type="float32", value=self._read_float32())
        elif code == TypeMarker.TYPE_FLOAT64:
            return LiteralNode(node_type="literal", code=code, value_type="float64", value=self._read_float64())
        elif code == TypeMarker.TYPE_BOOL:
            return LiteralNode(node_type="literal", code=code, value_type="bool", value=self._read_uint8() != 0)
        elif code == TypeMarker.TYPE_STRING:
            return LiteralNode(node_type="literal", code=code, value_type="string", value=self._read_string())
        elif code == TypeMarker.TYPE_BYTES:
            length = self._read_uint16()
            return LiteralNode(node_type="literal", code=code, value_type="bytes", value=self._read_bytes(length))
        elif code == TypeMarker.TYPE_TIMESTAMP:
            return LiteralNode(node_type="literal", code=code, value_type="timestamp", value=self._read_int64())
        elif code == TypeMarker.TYPE_NULL:
            return LiteralNode(node_type="literal", code=code, value_type="null", value=None)
        else:
            raise AILLDecodeError(f"Unknown type marker 0x{code:02X}", self._pos - 1)

    def _decode_struct(self) -> StructNode:
        """Decode a BEGIN_STRUCT ... END_STRUCT."""
        self._read_byte()  # consume BEGIN_STRUCT
        fields = {}
        while self._pos < len(self._data) and self._peek() != Structure.END_STRUCT:
            if self._peek() == Structure.FIELD_SEP:
                self._read_byte()
                continue
            if self._peek() == Structure.FIELD_ID:
                self._read_byte()
                field_code = self._read_uint16()
                value = self._decode_expression()
                fields[field_code] = value
            else:
                # Unnamed field (positional)
                expr = self._decode_expression()
                fields[len(fields)] = expr
        if self._pos < len(self._data):
            self._read_byte()  # consume END_STRUCT
        return StructNode(node_type="struct", fields=fields)

    def _decode_list(self) -> ListNode:
        """Decode a BEGIN_LIST count elements... END_LIST."""
        self._read_byte()  # consume BEGIN_LIST
        count = self._read_uint16()
        elements = []
        for _ in range(count):
            if self._pos >= len(self._data) or self._peek() == Structure.END_LIST:
                break
            elements.append(self._decode_expression())
        if self._pos < len(self._data) and self._peek() == Structure.END_LIST:
            self._read_byte()  # consume END_LIST
        return ListNode(node_type="list", count=count, elements=elements)

    def _decode_map(self) -> MapNode:
        """Decode a BEGIN_MAP count (key value)... END_MAP."""
        self._read_byte()  # consume BEGIN_MAP
        count = self._read_uint16()
        pairs = []
        for _ in range(count):
            if self._pos >= len(self._data) or self._peek() == Structure.END_MAP:
                break
            key = self._decode_expression()
            val = self._decode_expression()
            pairs.append((key, val))
        if self._pos < len(self._data) and self._peek() == Structure.END_MAP:
            self._read_byte()
        return MapNode(node_type="map", count=count, pairs=pairs)

    def _decode_pragmatic(self) -> PragmaticNode:
        """Decode a pragmatic act + expression."""
        code = self._read_byte()
        entry = BASE_CODEBOOK.get(code)
        act_name = entry.mnemonic if entry else f"PRAGMA_0x{code:02X}"
        expr = self._decode_expression()
        return PragmaticNode(node_type="pragmatic", act=act_name, expression=expr)

    def _decode_modal(self) -> ModalNode:
        """Decode a modality + expression."""
        code = self._read_byte()
        entry = BASE_CODEBOOK.get(code)
        mod_name = entry.mnemonic if entry else f"MODAL_0x{code:02X}"
        extra = None
        if code == Modality.PREDICTED:
            extra = self._read_float16()
        elif code == Modality.REPORTED:
            extra = self._read_uuid()
        expr = self._decode_expression()
        return ModalNode(node_type="modal", modality=mod_name, expression=expr, extra=extra)

    def _decode_temporal(self) -> TemporalNode:
        """Decode a temporal modifier + expression."""
        code = self._read_byte()
        entry = BASE_CODEBOOK.get(code)
        mod_name = entry.mnemonic if entry else f"TEMPORAL_0x{code:02X}"
        expr = self._decode_expression()
        return TemporalNode(node_type="temporal", modifier=mod_name, expression=expr)

    def _decode_annotation(self) -> ASTNode:
        """Decode an inline meta annotation."""
        code = self._read_byte()
        if code == Meta.CONFIDENCE:
            conf = self._read_float16()
            expr = self._decode_expression()
            return ASTNode(node_type="annotated", code=code, mnemonic=f"CONFIDENCE({conf:.2f})")
        elif code == Meta.LABEL:
            label = self._read_string()
            expr = self._decode_expression()
            return ASTNode(node_type="annotated", code=code, mnemonic=f"LABEL({label})")
        return ASTNode(node_type="annotation", code=code)

    def _decode_domain_ref(self) -> DomainRefNode:
        """Decode an ESCAPE_Ln domain reference."""
        code = self._read_byte()
        level = {Escape.ESCAPE_L1: 1, Escape.ESCAPE_L2: 2, Escape.ESCAPE_L3: 3}[code]
        domain_code = self._read_uint16()
        return DomainRefNode(node_type="domain_ref", level=level, domain_code=domain_code)


# ═══════════════════════════════════════════════════════════════════════
# Pretty Printer
# ═══════════════════════════════════════════════════════════════════════

def pretty_print(node: ASTNode, indent: int = 0, domain_codebooks=None) -> str:
    """Produce a human-readable representation of a decoded AILL AST."""
    prefix = "  " * indent
    lines = []

    if isinstance(node, UtteranceNode):
        lines.append(f"{prefix}UTTERANCE:")
        lines.append(pretty_print(node.meta, indent + 1, domain_codebooks))
        lines.append(f"{prefix}  BODY:")
        for expr in node.body:
            lines.append(pretty_print(expr, indent + 2, domain_codebooks))

    elif isinstance(node, MetaHeaderNode):
        lines.append(f"{prefix}META: confidence={node.confidence:.2f} priority={node.priority} "
                     f"timestamp={node.timestamp_us}")
        if node.dest_agent:
            lines.append(f"{prefix}  dest_agent={node.dest_agent.hex()}")
        if node.seqnum is not None:
            lines.append(f"{prefix}  seqnum={node.seqnum}")

    elif isinstance(node, PragmaticNode):
        lines.append(f"{prefix}{node.act}:")
        if node.expression:
            lines.append(pretty_print(node.expression, indent + 1, domain_codebooks))

    elif isinstance(node, ModalNode):
        extra_str = f" (horizon={node.extra}ms)" if node.extra else ""
        lines.append(f"{prefix}[{node.modality}{extra_str}]:")
        if node.expression:
            lines.append(pretty_print(node.expression, indent + 1, domain_codebooks))

    elif isinstance(node, TemporalNode):
        lines.append(f"{prefix}<{node.modifier}>:")
        if node.expression:
            lines.append(pretty_print(node.expression, indent + 1, domain_codebooks))

    elif isinstance(node, LiteralNode):
        lines.append(f"{prefix}{node.value_type}: {node.value}")

    elif isinstance(node, StructNode):
        lines.append(f"{prefix}STRUCT:")
        for fid, val in node.fields.items():
            field_label = f"field_0x{fid:04X}" if isinstance(fid, int) else str(fid)
            # Try to resolve field name from domain codebooks
            if domain_codebooks and isinstance(fid, int):
                for cb in domain_codebooks.values():
                    entry = cb.lookup(fid)
                    if entry:
                        field_label = entry.mnemonic
                        break
            lines.append(f"{prefix}  {field_label}:")
            lines.append(pretty_print(val, indent + 2, domain_codebooks))

    elif isinstance(node, ListNode):
        lines.append(f"{prefix}LIST[{node.count}]:")
        for elem in node.elements:
            lines.append(pretty_print(elem, indent + 1, domain_codebooks))

    elif isinstance(node, MapNode):
        lines.append(f"{prefix}MAP[{node.count}]:")
        for k, v in node.pairs:
            lines.append(f"{prefix}  key: {pretty_print(k, 0, domain_codebooks).strip()}")
            lines.append(f"{prefix}  val: {pretty_print(v, 0, domain_codebooks).strip()}")

    elif isinstance(node, DomainRefNode):
        level_names = {1: "L1", 2: "L2", 3: "L3"}
        label = f"DOMAIN_0x{node.domain_code:04X}"
        if domain_codebooks:
            for cb in domain_codebooks.values():
                entry = cb.lookup(node.domain_code)
                if entry:
                    label = f"{cb.name}:{entry.mnemonic}"
                    break
        lines.append(f"{prefix}REF({level_names.get(node.level, '?')}: {label})")

    elif isinstance(node, ContextRefNode):
        lines.append(f"{prefix}SCT_REF[{node.sct_index}]")

    elif isinstance(node, ASTNode):
        lines.append(f"{prefix}{node.mnemonic or f'CODE_0x{node.code:02X}'}")

    return "\n".join(lines)
