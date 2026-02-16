/// Base codebook entry metadata.
#[derive(Debug, Clone, Copy)]
pub struct CodeEntry {
    pub code: u8,
    pub mnemonic: &'static str,
    pub category: &'static str,
}

// ═══════════════════════════════════════════════════════════════════════
// Frame Control (0x00-0x0F)
// ═══════════════════════════════════════════════════════════════════════

pub mod fc {
    pub const START_UTTERANCE: u8 = 0x00;
    pub const END_UTTERANCE: u8 = 0x01;
    pub const ABORT: u8 = 0x02;
    pub const PAUSE: u8 = 0x03;
    pub const RESUME: u8 = 0x04;
    pub const RETRANSMIT: u8 = 0x05;
    pub const ACK_EPOCH: u8 = 0x06;
    pub const NACK_EPOCH: u8 = 0x07;
    pub const SYNC_MARK: u8 = 0x08;
    pub const FRAGMENT_START: u8 = 0x09;
    pub const FRAGMENT_CONT: u8 = 0x0A;
    pub const FRAGMENT_END: u8 = 0x0B;
    pub const ECHO_REQUEST: u8 = 0x0C;
    pub const ECHO_REPLY: u8 = 0x0D;
    pub const RESERVED_0E: u8 = 0x0E;
    pub const RESERVED_0F: u8 = 0x0F;
}

// ═══════════════════════════════════════════════════════════════════════
// Type Markers (0x10-0x1F)
// ═══════════════════════════════════════════════════════════════════════

pub mod ty {
    pub const TYPE_INT8: u8 = 0x10;
    pub const TYPE_INT16: u8 = 0x11;
    pub const TYPE_INT32: u8 = 0x12;
    pub const TYPE_INT64: u8 = 0x13;
    pub const TYPE_UINT8: u8 = 0x14;
    pub const TYPE_UINT16: u8 = 0x15;
    pub const TYPE_UINT32: u8 = 0x16;
    pub const TYPE_UINT64: u8 = 0x17;
    pub const TYPE_FLOAT16: u8 = 0x18;
    pub const TYPE_FLOAT32: u8 = 0x19;
    pub const TYPE_FLOAT64: u8 = 0x1A;
    pub const TYPE_BOOL: u8 = 0x1B;
    pub const TYPE_STRING: u8 = 0x1C;
    pub const TYPE_BYTES: u8 = 0x1D;
    pub const TYPE_TIMESTAMP: u8 = 0x1E;
    pub const TYPE_NULL: u8 = 0x1F;
}

// ═══════════════════════════════════════════════════════════════════════
// Structure (0x20-0x2F)
// ═══════════════════════════════════════════════════════════════════════

pub mod st {
    pub const BEGIN_STRUCT: u8 = 0x20;
    pub const END_STRUCT: u8 = 0x21;
    pub const FIELD_SEP: u8 = 0x22;
    pub const BEGIN_LIST: u8 = 0x23;
    pub const END_LIST: u8 = 0x24;
    pub const BEGIN_MAP: u8 = 0x25;
    pub const END_MAP: u8 = 0x26;
    pub const BEGIN_TUPLE: u8 = 0x27;
    pub const END_TUPLE: u8 = 0x28;
    pub const FIELD_ID: u8 = 0x29;
    pub const BEGIN_UNION: u8 = 0x2A;
    pub const END_UNION: u8 = 0x2B;
    pub const BEGIN_OPTION: u8 = 0x2C;
    pub const END_OPTION: u8 = 0x2D;
    pub const SCHEMA_REF: u8 = 0x2E;
    pub const RESERVED_2F: u8 = 0x2F;
}

// ═══════════════════════════════════════════════════════════════════════
// Quantifiers (0x30-0x3F)
// ═══════════════════════════════════════════════════════════════════════

pub mod quant {
    pub const FORALL: u8 = 0x30;
    pub const EXISTS: u8 = 0x31;
    pub const EXISTS_UNIQUE: u8 = 0x32;
    pub const EXACTLY_N: u8 = 0x33;
    pub const AT_LEAST_N: u8 = 0x34;
    pub const AT_MOST_N: u8 = 0x35;
    pub const COUNT: u8 = 0x36;
    pub const ZERO: u8 = 0x37;
    pub const ONE: u8 = 0x38;
    pub const FEW: u8 = 0x39;
    pub const MANY: u8 = 0x3A;
    pub const ALL: u8 = 0x3B;
    pub const NONE_Q: u8 = 0x3C;
    pub const MOST: u8 = 0x3D;
    pub const PROPORTION: u8 = 0x3E;
    pub const RESERVED_3F: u8 = 0x3F;
}

// ═══════════════════════════════════════════════════════════════════════
// Logic (0x40-0x4F)
// ═══════════════════════════════════════════════════════════════════════

pub mod logic {
    pub const AND: u8 = 0x40;
    pub const OR: u8 = 0x41;
    pub const NOT: u8 = 0x42;
    pub const XOR: u8 = 0x43;
    pub const IMPLIES: u8 = 0x44;
    pub const IFF: u8 = 0x45;
    pub const NAND: u8 = 0x46;
    pub const NOR: u8 = 0x47;
    pub const IF_THEN_ELSE: u8 = 0x48;
    pub const COALESCE: u8 = 0x49;
    pub const IS_NULL: u8 = 0x4A;
    pub const IS_TYPE: u8 = 0x4B;
    pub const RESERVED_4C: u8 = 0x4C;
    pub const RESERVED_4D: u8 = 0x4D;
    pub const RESERVED_4E: u8 = 0x4E;
    pub const RESERVED_4F: u8 = 0x4F;
}

// ═══════════════════════════════════════════════════════════════════════
// Relational (0x50-0x5F)
// ═══════════════════════════════════════════════════════════════════════

pub mod rel {
    pub const EQ: u8 = 0x50;
    pub const NEQ: u8 = 0x51;
    pub const LT: u8 = 0x52;
    pub const GT: u8 = 0x53;
    pub const LTE: u8 = 0x54;
    pub const GTE: u8 = 0x55;
    pub const APPROX: u8 = 0x56;
    pub const CONTAINS: u8 = 0x57;
    pub const SUBSET: u8 = 0x58;
    pub const SUPERSET: u8 = 0x59;
    pub const IN_RANGE: u8 = 0x5A;
    pub const MATCHES: u8 = 0x5B;
    pub const STARTS_WITH: u8 = 0x5C;
    pub const ENDS_WITH: u8 = 0x5D;
    pub const BETWEEN: u8 = 0x5E;
    pub const RESERVED_5F: u8 = 0x5F;
}

// ═══════════════════════════════════════════════════════════════════════
// Temporal (0x60-0x6F)
// ═══════════════════════════════════════════════════════════════════════

pub mod temporal {
    pub const PAST: u8 = 0x60;
    pub const PRESENT: u8 = 0x61;
    pub const FUTURE: u8 = 0x62;
    pub const DURATION: u8 = 0x63;
    pub const T_BEFORE: u8 = 0x64;
    pub const T_AFTER: u8 = 0x65;
    pub const T_DURING: u8 = 0x66;
    pub const T_SIMULTANEOUS: u8 = 0x67;
    pub const T_STARTS: u8 = 0x68;
    pub const T_FINISHES: u8 = 0x69;
    pub const T_OVERLAPS: u8 = 0x6A;
    pub const T_MEETS: u8 = 0x6B;
    pub const T_ELAPSED: u8 = 0x6C;
    pub const T_NOW: u8 = 0x6D;
    pub const T_DEADLINE: u8 = 0x6E;
    pub const RESERVED_6F: u8 = 0x6F;
}

// ═══════════════════════════════════════════════════════════════════════
// Modality (0x70-0x7F)
// ═══════════════════════════════════════════════════════════════════════

pub mod modal {
    pub const CERTAIN: u8 = 0x70;
    pub const PROBABLE: u8 = 0x71;
    pub const POSSIBLE: u8 = 0x72;
    pub const UNLIKELY: u8 = 0x73;
    pub const UNCERTAIN: u8 = 0x74;
    pub const HYPOTHETICAL: u8 = 0x75;
    pub const COUNTERFACTUAL: u8 = 0x76;
    pub const OBLIGATORY: u8 = 0x77;
    pub const PERMITTED: u8 = 0x78;
    pub const FORBIDDEN: u8 = 0x79;
    pub const INFERRED: u8 = 0x7A;
    pub const OBSERVED: u8 = 0x7B;
    pub const REPORTED: u8 = 0x7C;
    pub const PREDICTED: u8 = 0x7D;
    pub const DESIRED: u8 = 0x7E;
    pub const UNDESIRED: u8 = 0x7F;
}

// ═══════════════════════════════════════════════════════════════════════
// Pragmatic Acts (0x80-0x8F)
// ═══════════════════════════════════════════════════════════════════════

pub mod pragma {
    pub const QUERY: u8 = 0x80;
    pub const ASSERT: u8 = 0x81;
    pub const REQUEST: u8 = 0x82;
    pub const COMMAND: u8 = 0x83;
    pub const ACKNOWLEDGE: u8 = 0x84;
    pub const REJECT: u8 = 0x85;
    pub const CLARIFY: u8 = 0x86;
    pub const CORRECT: u8 = 0x87;
    pub const PROPOSE: u8 = 0x88;
    pub const ACCEPT: u8 = 0x89;
    pub const WARN: u8 = 0x8A;
    pub const PROMISE: u8 = 0x8B;
    pub const INFORM: u8 = 0x8C;
    pub const SUGGEST: u8 = 0x8D;
    pub const GREET: u8 = 0x8E;
    pub const FAREWELL: u8 = 0x8F;
}

// ═══════════════════════════════════════════════════════════════════════
// Meta & Annotations (0x90-0x9F)
// ═══════════════════════════════════════════════════════════════════════

pub mod meta {
    pub const CONFIDENCE: u8 = 0x90;
    pub const PRIORITY: u8 = 0x91;
    pub const SOURCE_AGENT: u8 = 0x92;
    pub const DEST_AGENT: u8 = 0x93;
    pub const TIMESTAMP_META: u8 = 0x94;
    pub const SEQNUM: u8 = 0x95;
    pub const HASH_REF: u8 = 0x96;
    pub const TOPIC: u8 = 0x97;
    pub const CONTEXT_REF: u8 = 0x98;
    pub const EPOCH_BOUNDARY: u8 = 0x99;
    pub const LABEL: u8 = 0x9A;
    pub const VERSION_TAG: u8 = 0x9B;
    pub const TRACE_ID: u8 = 0x9C;
    pub const COST: u8 = 0x9D;
    pub const TTL: u8 = 0x9E;
    pub const RESERVED_9F: u8 = 0x9F;
}

// ═══════════════════════════════════════════════════════════════════════
// Arithmetic (0xA0-0xBF)
// ═══════════════════════════════════════════════════════════════════════

pub mod arith {
    pub const ADD: u8 = 0xA0;
    pub const SUB: u8 = 0xA1;
    pub const MUL: u8 = 0xA2;
    pub const DIV: u8 = 0xA3;
    pub const MOD: u8 = 0xA4;
    pub const POW: u8 = 0xA5;
    pub const SQRT: u8 = 0xA6;
    pub const LOG: u8 = 0xA7;
    pub const LOG10: u8 = 0xA8;
    pub const LOG2: u8 = 0xA9;
    pub const ABS: u8 = 0xAA;
    pub const NEG: u8 = 0xAB;
    pub const ROUND: u8 = 0xAC;
    pub const FLOOR: u8 = 0xAD;
    pub const CEIL: u8 = 0xAE;
    pub const TRUNC: u8 = 0xAF;
    pub const MIN: u8 = 0xB0;
    pub const MAX: u8 = 0xB1;
    pub const SUM: u8 = 0xB2;
    pub const MEAN: u8 = 0xB3;
    pub const MEDIAN: u8 = 0xB4;
    pub const STDDEV: u8 = 0xB5;
    pub const VARIANCE: u8 = 0xB6;
    pub const DOT_PRODUCT: u8 = 0xB7;
    pub const CROSS_PRODUCT: u8 = 0xB8;
    pub const NORM: u8 = 0xB9;
    pub const CLAMP: u8 = 0xBA;
    pub const LERP: u8 = 0xBB;
    pub const SIN: u8 = 0xBC;
    pub const COS: u8 = 0xBD;
    pub const ATAN2: u8 = 0xBE;
    pub const DISTANCE: u8 = 0xBF;
}

// ═══════════════════════════════════════════════════════════════════════
// Escape Codes (0xF0-0xFF)
// ═══════════════════════════════════════════════════════════════════════

pub mod esc {
    pub const ESCAPE_L1: u8 = 0xF0;
    pub const ESCAPE_L2: u8 = 0xF1;
    pub const ESCAPE_L3: u8 = 0xF2;
    pub const LITERAL_BYTES: u8 = 0xF3;
    pub const CODEBOOK_REF: u8 = 0xF4;
    pub const EXTENSION: u8 = 0xF5;
    pub const EXT_ACK: u8 = 0xF6;
    pub const EXT_NACK: u8 = 0xF7;
    pub const CODEBOOK_DEF: u8 = 0xF8;
    pub const CODEBOOK_ACK: u8 = 0xF9;
    pub const CODEBOOK_NACK: u8 = 0xFA;
    pub const STREAM_ID: u8 = 0xFB;
    pub const XREF: u8 = 0xFC;
    pub const COMMENT: u8 = 0xFD;
    pub const NOP: u8 = 0xFE;
    pub const RESERVED_FF: u8 = 0xFF;
}

/// Look up the mnemonic name for a base codebook byte.
pub fn mnemonic_for(code: u8) -> &'static str {
    BASE_CODEBOOK[code as usize].mnemonic
}

/// The complete 256-entry base codebook.
pub static BASE_CODEBOOK: [CodeEntry; 256] = {
    // We initialize with a macro-like approach using const
    let mut table = [CodeEntry {
        code: 0,
        mnemonic: "UNKNOWN",
        category: "unknown",
    }; 256];

    // Frame Control 0x00-0x0F
    table[0x00] = CodeEntry { code: 0x00, mnemonic: "START_UTTERANCE", category: "frame_control" };
    table[0x01] = CodeEntry { code: 0x01, mnemonic: "END_UTTERANCE", category: "frame_control" };
    table[0x02] = CodeEntry { code: 0x02, mnemonic: "ABORT", category: "frame_control" };
    table[0x03] = CodeEntry { code: 0x03, mnemonic: "PAUSE", category: "frame_control" };
    table[0x04] = CodeEntry { code: 0x04, mnemonic: "RESUME", category: "frame_control" };
    table[0x05] = CodeEntry { code: 0x05, mnemonic: "RETRANSMIT", category: "frame_control" };
    table[0x06] = CodeEntry { code: 0x06, mnemonic: "ACK_EPOCH", category: "frame_control" };
    table[0x07] = CodeEntry { code: 0x07, mnemonic: "NACK_EPOCH", category: "frame_control" };
    table[0x08] = CodeEntry { code: 0x08, mnemonic: "SYNC_MARK", category: "frame_control" };
    table[0x09] = CodeEntry { code: 0x09, mnemonic: "FRAGMENT_START", category: "frame_control" };
    table[0x0A] = CodeEntry { code: 0x0A, mnemonic: "FRAGMENT_CONT", category: "frame_control" };
    table[0x0B] = CodeEntry { code: 0x0B, mnemonic: "FRAGMENT_END", category: "frame_control" };
    table[0x0C] = CodeEntry { code: 0x0C, mnemonic: "ECHO_REQUEST", category: "frame_control" };
    table[0x0D] = CodeEntry { code: 0x0D, mnemonic: "ECHO_REPLY", category: "frame_control" };
    table[0x0E] = CodeEntry { code: 0x0E, mnemonic: "RESERVED_0E", category: "frame_control" };
    table[0x0F] = CodeEntry { code: 0x0F, mnemonic: "RESERVED_0F", category: "frame_control" };

    // Type Markers 0x10-0x1F
    table[0x10] = CodeEntry { code: 0x10, mnemonic: "TYPE_INT8", category: "type_marker" };
    table[0x11] = CodeEntry { code: 0x11, mnemonic: "TYPE_INT16", category: "type_marker" };
    table[0x12] = CodeEntry { code: 0x12, mnemonic: "TYPE_INT32", category: "type_marker" };
    table[0x13] = CodeEntry { code: 0x13, mnemonic: "TYPE_INT64", category: "type_marker" };
    table[0x14] = CodeEntry { code: 0x14, mnemonic: "TYPE_UINT8", category: "type_marker" };
    table[0x15] = CodeEntry { code: 0x15, mnemonic: "TYPE_UINT16", category: "type_marker" };
    table[0x16] = CodeEntry { code: 0x16, mnemonic: "TYPE_UINT32", category: "type_marker" };
    table[0x17] = CodeEntry { code: 0x17, mnemonic: "TYPE_UINT64", category: "type_marker" };
    table[0x18] = CodeEntry { code: 0x18, mnemonic: "TYPE_FLOAT16", category: "type_marker" };
    table[0x19] = CodeEntry { code: 0x19, mnemonic: "TYPE_FLOAT32", category: "type_marker" };
    table[0x1A] = CodeEntry { code: 0x1A, mnemonic: "TYPE_FLOAT64", category: "type_marker" };
    table[0x1B] = CodeEntry { code: 0x1B, mnemonic: "TYPE_BOOL", category: "type_marker" };
    table[0x1C] = CodeEntry { code: 0x1C, mnemonic: "TYPE_STRING", category: "type_marker" };
    table[0x1D] = CodeEntry { code: 0x1D, mnemonic: "TYPE_BYTES", category: "type_marker" };
    table[0x1E] = CodeEntry { code: 0x1E, mnemonic: "TYPE_TIMESTAMP", category: "type_marker" };
    table[0x1F] = CodeEntry { code: 0x1F, mnemonic: "TYPE_NULL", category: "type_marker" };

    // Structure 0x20-0x2F
    table[0x20] = CodeEntry { code: 0x20, mnemonic: "BEGIN_STRUCT", category: "structure" };
    table[0x21] = CodeEntry { code: 0x21, mnemonic: "END_STRUCT", category: "structure" };
    table[0x22] = CodeEntry { code: 0x22, mnemonic: "FIELD_SEP", category: "structure" };
    table[0x23] = CodeEntry { code: 0x23, mnemonic: "BEGIN_LIST", category: "structure" };
    table[0x24] = CodeEntry { code: 0x24, mnemonic: "END_LIST", category: "structure" };
    table[0x25] = CodeEntry { code: 0x25, mnemonic: "BEGIN_MAP", category: "structure" };
    table[0x26] = CodeEntry { code: 0x26, mnemonic: "END_MAP", category: "structure" };
    table[0x27] = CodeEntry { code: 0x27, mnemonic: "BEGIN_TUPLE", category: "structure" };
    table[0x28] = CodeEntry { code: 0x28, mnemonic: "END_TUPLE", category: "structure" };
    table[0x29] = CodeEntry { code: 0x29, mnemonic: "FIELD_ID", category: "structure" };
    table[0x2A] = CodeEntry { code: 0x2A, mnemonic: "BEGIN_UNION", category: "structure" };
    table[0x2B] = CodeEntry { code: 0x2B, mnemonic: "END_UNION", category: "structure" };
    table[0x2C] = CodeEntry { code: 0x2C, mnemonic: "BEGIN_OPTION", category: "structure" };
    table[0x2D] = CodeEntry { code: 0x2D, mnemonic: "END_OPTION", category: "structure" };
    table[0x2E] = CodeEntry { code: 0x2E, mnemonic: "SCHEMA_REF", category: "structure" };
    table[0x2F] = CodeEntry { code: 0x2F, mnemonic: "RESERVED_2F", category: "structure" };

    // Quantifiers 0x30-0x3F
    table[0x30] = CodeEntry { code: 0x30, mnemonic: "FORALL", category: "quantifier" };
    table[0x31] = CodeEntry { code: 0x31, mnemonic: "EXISTS", category: "quantifier" };
    table[0x32] = CodeEntry { code: 0x32, mnemonic: "EXISTS_UNIQUE", category: "quantifier" };
    table[0x33] = CodeEntry { code: 0x33, mnemonic: "EXACTLY_N", category: "quantifier" };
    table[0x34] = CodeEntry { code: 0x34, mnemonic: "AT_LEAST_N", category: "quantifier" };
    table[0x35] = CodeEntry { code: 0x35, mnemonic: "AT_MOST_N", category: "quantifier" };
    table[0x36] = CodeEntry { code: 0x36, mnemonic: "COUNT", category: "quantifier" };
    table[0x37] = CodeEntry { code: 0x37, mnemonic: "ZERO", category: "quantifier" };
    table[0x38] = CodeEntry { code: 0x38, mnemonic: "ONE", category: "quantifier" };
    table[0x39] = CodeEntry { code: 0x39, mnemonic: "FEW", category: "quantifier" };
    table[0x3A] = CodeEntry { code: 0x3A, mnemonic: "MANY", category: "quantifier" };
    table[0x3B] = CodeEntry { code: 0x3B, mnemonic: "ALL", category: "quantifier" };
    table[0x3C] = CodeEntry { code: 0x3C, mnemonic: "NONE_Q", category: "quantifier" };
    table[0x3D] = CodeEntry { code: 0x3D, mnemonic: "MOST", category: "quantifier" };
    table[0x3E] = CodeEntry { code: 0x3E, mnemonic: "PROPORTION", category: "quantifier" };
    table[0x3F] = CodeEntry { code: 0x3F, mnemonic: "RESERVED_3F", category: "quantifier" };

    // Logic 0x40-0x4F
    table[0x40] = CodeEntry { code: 0x40, mnemonic: "AND", category: "logic" };
    table[0x41] = CodeEntry { code: 0x41, mnemonic: "OR", category: "logic" };
    table[0x42] = CodeEntry { code: 0x42, mnemonic: "NOT", category: "logic" };
    table[0x43] = CodeEntry { code: 0x43, mnemonic: "XOR", category: "logic" };
    table[0x44] = CodeEntry { code: 0x44, mnemonic: "IMPLIES", category: "logic" };
    table[0x45] = CodeEntry { code: 0x45, mnemonic: "IFF", category: "logic" };
    table[0x46] = CodeEntry { code: 0x46, mnemonic: "NAND", category: "logic" };
    table[0x47] = CodeEntry { code: 0x47, mnemonic: "NOR", category: "logic" };
    table[0x48] = CodeEntry { code: 0x48, mnemonic: "IF_THEN_ELSE", category: "logic" };
    table[0x49] = CodeEntry { code: 0x49, mnemonic: "COALESCE", category: "logic" };
    table[0x4A] = CodeEntry { code: 0x4A, mnemonic: "IS_NULL", category: "logic" };
    table[0x4B] = CodeEntry { code: 0x4B, mnemonic: "IS_TYPE", category: "logic" };
    table[0x4C] = CodeEntry { code: 0x4C, mnemonic: "RESERVED_4C", category: "logic" };
    table[0x4D] = CodeEntry { code: 0x4D, mnemonic: "RESERVED_4D", category: "logic" };
    table[0x4E] = CodeEntry { code: 0x4E, mnemonic: "RESERVED_4E", category: "logic" };
    table[0x4F] = CodeEntry { code: 0x4F, mnemonic: "RESERVED_4F", category: "logic" };

    // Relational 0x50-0x5F
    table[0x50] = CodeEntry { code: 0x50, mnemonic: "EQ", category: "relational" };
    table[0x51] = CodeEntry { code: 0x51, mnemonic: "NEQ", category: "relational" };
    table[0x52] = CodeEntry { code: 0x52, mnemonic: "LT", category: "relational" };
    table[0x53] = CodeEntry { code: 0x53, mnemonic: "GT", category: "relational" };
    table[0x54] = CodeEntry { code: 0x54, mnemonic: "LTE", category: "relational" };
    table[0x55] = CodeEntry { code: 0x55, mnemonic: "GTE", category: "relational" };
    table[0x56] = CodeEntry { code: 0x56, mnemonic: "APPROX", category: "relational" };
    table[0x57] = CodeEntry { code: 0x57, mnemonic: "CONTAINS", category: "relational" };
    table[0x58] = CodeEntry { code: 0x58, mnemonic: "SUBSET", category: "relational" };
    table[0x59] = CodeEntry { code: 0x59, mnemonic: "SUPERSET", category: "relational" };
    table[0x5A] = CodeEntry { code: 0x5A, mnemonic: "IN_RANGE", category: "relational" };
    table[0x5B] = CodeEntry { code: 0x5B, mnemonic: "MATCHES", category: "relational" };
    table[0x5C] = CodeEntry { code: 0x5C, mnemonic: "STARTS_WITH", category: "relational" };
    table[0x5D] = CodeEntry { code: 0x5D, mnemonic: "ENDS_WITH", category: "relational" };
    table[0x5E] = CodeEntry { code: 0x5E, mnemonic: "BETWEEN", category: "relational" };
    table[0x5F] = CodeEntry { code: 0x5F, mnemonic: "RESERVED_5F", category: "relational" };

    // Temporal 0x60-0x6F
    table[0x60] = CodeEntry { code: 0x60, mnemonic: "PAST", category: "temporal" };
    table[0x61] = CodeEntry { code: 0x61, mnemonic: "PRESENT", category: "temporal" };
    table[0x62] = CodeEntry { code: 0x62, mnemonic: "FUTURE", category: "temporal" };
    table[0x63] = CodeEntry { code: 0x63, mnemonic: "DURATION", category: "temporal" };
    table[0x64] = CodeEntry { code: 0x64, mnemonic: "T_BEFORE", category: "temporal" };
    table[0x65] = CodeEntry { code: 0x65, mnemonic: "T_AFTER", category: "temporal" };
    table[0x66] = CodeEntry { code: 0x66, mnemonic: "T_DURING", category: "temporal" };
    table[0x67] = CodeEntry { code: 0x67, mnemonic: "T_SIMULTANEOUS", category: "temporal" };
    table[0x68] = CodeEntry { code: 0x68, mnemonic: "T_STARTS", category: "temporal" };
    table[0x69] = CodeEntry { code: 0x69, mnemonic: "T_FINISHES", category: "temporal" };
    table[0x6A] = CodeEntry { code: 0x6A, mnemonic: "T_OVERLAPS", category: "temporal" };
    table[0x6B] = CodeEntry { code: 0x6B, mnemonic: "T_MEETS", category: "temporal" };
    table[0x6C] = CodeEntry { code: 0x6C, mnemonic: "T_ELAPSED", category: "temporal" };
    table[0x6D] = CodeEntry { code: 0x6D, mnemonic: "T_NOW", category: "temporal" };
    table[0x6E] = CodeEntry { code: 0x6E, mnemonic: "T_DEADLINE", category: "temporal" };
    table[0x6F] = CodeEntry { code: 0x6F, mnemonic: "RESERVED_6F", category: "temporal" };

    // Modality 0x70-0x7F
    table[0x70] = CodeEntry { code: 0x70, mnemonic: "CERTAIN", category: "modality" };
    table[0x71] = CodeEntry { code: 0x71, mnemonic: "PROBABLE", category: "modality" };
    table[0x72] = CodeEntry { code: 0x72, mnemonic: "POSSIBLE", category: "modality" };
    table[0x73] = CodeEntry { code: 0x73, mnemonic: "UNLIKELY", category: "modality" };
    table[0x74] = CodeEntry { code: 0x74, mnemonic: "UNCERTAIN", category: "modality" };
    table[0x75] = CodeEntry { code: 0x75, mnemonic: "HYPOTHETICAL", category: "modality" };
    table[0x76] = CodeEntry { code: 0x76, mnemonic: "COUNTERFACTUAL", category: "modality" };
    table[0x77] = CodeEntry { code: 0x77, mnemonic: "OBLIGATORY", category: "modality" };
    table[0x78] = CodeEntry { code: 0x78, mnemonic: "PERMITTED", category: "modality" };
    table[0x79] = CodeEntry { code: 0x79, mnemonic: "FORBIDDEN", category: "modality" };
    table[0x7A] = CodeEntry { code: 0x7A, mnemonic: "INFERRED", category: "modality" };
    table[0x7B] = CodeEntry { code: 0x7B, mnemonic: "OBSERVED", category: "modality" };
    table[0x7C] = CodeEntry { code: 0x7C, mnemonic: "REPORTED", category: "modality" };
    table[0x7D] = CodeEntry { code: 0x7D, mnemonic: "PREDICTED", category: "modality" };
    table[0x7E] = CodeEntry { code: 0x7E, mnemonic: "DESIRED", category: "modality" };
    table[0x7F] = CodeEntry { code: 0x7F, mnemonic: "UNDESIRED", category: "modality" };

    // Pragmatic 0x80-0x8F
    table[0x80] = CodeEntry { code: 0x80, mnemonic: "QUERY", category: "pragmatic" };
    table[0x81] = CodeEntry { code: 0x81, mnemonic: "ASSERT", category: "pragmatic" };
    table[0x82] = CodeEntry { code: 0x82, mnemonic: "REQUEST", category: "pragmatic" };
    table[0x83] = CodeEntry { code: 0x83, mnemonic: "COMMAND", category: "pragmatic" };
    table[0x84] = CodeEntry { code: 0x84, mnemonic: "ACKNOWLEDGE", category: "pragmatic" };
    table[0x85] = CodeEntry { code: 0x85, mnemonic: "REJECT", category: "pragmatic" };
    table[0x86] = CodeEntry { code: 0x86, mnemonic: "CLARIFY", category: "pragmatic" };
    table[0x87] = CodeEntry { code: 0x87, mnemonic: "CORRECT", category: "pragmatic" };
    table[0x88] = CodeEntry { code: 0x88, mnemonic: "PROPOSE", category: "pragmatic" };
    table[0x89] = CodeEntry { code: 0x89, mnemonic: "ACCEPT", category: "pragmatic" };
    table[0x8A] = CodeEntry { code: 0x8A, mnemonic: "WARN", category: "pragmatic" };
    table[0x8B] = CodeEntry { code: 0x8B, mnemonic: "PROMISE", category: "pragmatic" };
    table[0x8C] = CodeEntry { code: 0x8C, mnemonic: "INFORM", category: "pragmatic" };
    table[0x8D] = CodeEntry { code: 0x8D, mnemonic: "SUGGEST", category: "pragmatic" };
    table[0x8E] = CodeEntry { code: 0x8E, mnemonic: "GREET", category: "pragmatic" };
    table[0x8F] = CodeEntry { code: 0x8F, mnemonic: "FAREWELL", category: "pragmatic" };

    // Meta 0x90-0x9F
    table[0x90] = CodeEntry { code: 0x90, mnemonic: "CONFIDENCE", category: "meta" };
    table[0x91] = CodeEntry { code: 0x91, mnemonic: "PRIORITY", category: "meta" };
    table[0x92] = CodeEntry { code: 0x92, mnemonic: "SOURCE_AGENT", category: "meta" };
    table[0x93] = CodeEntry { code: 0x93, mnemonic: "DEST_AGENT", category: "meta" };
    table[0x94] = CodeEntry { code: 0x94, mnemonic: "TIMESTAMP_META", category: "meta" };
    table[0x95] = CodeEntry { code: 0x95, mnemonic: "SEQNUM", category: "meta" };
    table[0x96] = CodeEntry { code: 0x96, mnemonic: "HASH_REF", category: "meta" };
    table[0x97] = CodeEntry { code: 0x97, mnemonic: "TOPIC", category: "meta" };
    table[0x98] = CodeEntry { code: 0x98, mnemonic: "CONTEXT_REF", category: "meta" };
    table[0x99] = CodeEntry { code: 0x99, mnemonic: "EPOCH_BOUNDARY", category: "meta" };
    table[0x9A] = CodeEntry { code: 0x9A, mnemonic: "LABEL", category: "meta" };
    table[0x9B] = CodeEntry { code: 0x9B, mnemonic: "VERSION_TAG", category: "meta" };
    table[0x9C] = CodeEntry { code: 0x9C, mnemonic: "TRACE_ID", category: "meta" };
    table[0x9D] = CodeEntry { code: 0x9D, mnemonic: "COST", category: "meta" };
    table[0x9E] = CodeEntry { code: 0x9E, mnemonic: "TTL", category: "meta" };
    table[0x9F] = CodeEntry { code: 0x9F, mnemonic: "RESERVED_9F", category: "meta" };

    // Arithmetic 0xA0-0xBF
    table[0xA0] = CodeEntry { code: 0xA0, mnemonic: "ADD", category: "arithmetic" };
    table[0xA1] = CodeEntry { code: 0xA1, mnemonic: "SUB", category: "arithmetic" };
    table[0xA2] = CodeEntry { code: 0xA2, mnemonic: "MUL", category: "arithmetic" };
    table[0xA3] = CodeEntry { code: 0xA3, mnemonic: "DIV", category: "arithmetic" };
    table[0xA4] = CodeEntry { code: 0xA4, mnemonic: "MOD", category: "arithmetic" };
    table[0xA5] = CodeEntry { code: 0xA5, mnemonic: "POW", category: "arithmetic" };
    table[0xA6] = CodeEntry { code: 0xA6, mnemonic: "SQRT", category: "arithmetic" };
    table[0xA7] = CodeEntry { code: 0xA7, mnemonic: "LOG", category: "arithmetic" };
    table[0xA8] = CodeEntry { code: 0xA8, mnemonic: "LOG10", category: "arithmetic" };
    table[0xA9] = CodeEntry { code: 0xA9, mnemonic: "LOG2", category: "arithmetic" };
    table[0xAA] = CodeEntry { code: 0xAA, mnemonic: "ABS", category: "arithmetic" };
    table[0xAB] = CodeEntry { code: 0xAB, mnemonic: "NEG", category: "arithmetic" };
    table[0xAC] = CodeEntry { code: 0xAC, mnemonic: "ROUND", category: "arithmetic" };
    table[0xAD] = CodeEntry { code: 0xAD, mnemonic: "FLOOR", category: "arithmetic" };
    table[0xAE] = CodeEntry { code: 0xAE, mnemonic: "CEIL", category: "arithmetic" };
    table[0xAF] = CodeEntry { code: 0xAF, mnemonic: "TRUNC", category: "arithmetic" };
    table[0xB0] = CodeEntry { code: 0xB0, mnemonic: "MIN", category: "arithmetic" };
    table[0xB1] = CodeEntry { code: 0xB1, mnemonic: "MAX", category: "arithmetic" };
    table[0xB2] = CodeEntry { code: 0xB2, mnemonic: "SUM", category: "arithmetic" };
    table[0xB3] = CodeEntry { code: 0xB3, mnemonic: "MEAN", category: "arithmetic" };
    table[0xB4] = CodeEntry { code: 0xB4, mnemonic: "MEDIAN", category: "arithmetic" };
    table[0xB5] = CodeEntry { code: 0xB5, mnemonic: "STDDEV", category: "arithmetic" };
    table[0xB6] = CodeEntry { code: 0xB6, mnemonic: "VARIANCE", category: "arithmetic" };
    table[0xB7] = CodeEntry { code: 0xB7, mnemonic: "DOT_PRODUCT", category: "arithmetic" };
    table[0xB8] = CodeEntry { code: 0xB8, mnemonic: "CROSS_PRODUCT", category: "arithmetic" };
    table[0xB9] = CodeEntry { code: 0xB9, mnemonic: "NORM", category: "arithmetic" };
    table[0xBA] = CodeEntry { code: 0xBA, mnemonic: "CLAMP", category: "arithmetic" };
    table[0xBB] = CodeEntry { code: 0xBB, mnemonic: "LERP", category: "arithmetic" };
    table[0xBC] = CodeEntry { code: 0xBC, mnemonic: "SIN", category: "arithmetic" };
    table[0xBD] = CodeEntry { code: 0xBD, mnemonic: "COS", category: "arithmetic" };
    table[0xBE] = CodeEntry { code: 0xBE, mnemonic: "ATAN2", category: "arithmetic" };
    table[0xBF] = CodeEntry { code: 0xBF, mnemonic: "DISTANCE", category: "arithmetic" };

    // Reserved range 0xC0-0xEF
    let mut r = 0xC0usize;
    while r <= 0xEF {
        table[r] = CodeEntry { code: r as u8, mnemonic: "RESERVED", category: "reserved" };
        r += 1;
    }

    // Escape 0xF0-0xFF
    table[0xF0] = CodeEntry { code: 0xF0, mnemonic: "ESCAPE_L1", category: "escape" };
    table[0xF1] = CodeEntry { code: 0xF1, mnemonic: "ESCAPE_L2", category: "escape" };
    table[0xF2] = CodeEntry { code: 0xF2, mnemonic: "ESCAPE_L3", category: "escape" };
    table[0xF3] = CodeEntry { code: 0xF3, mnemonic: "LITERAL_BYTES", category: "escape" };
    table[0xF4] = CodeEntry { code: 0xF4, mnemonic: "CODEBOOK_REF", category: "escape" };
    table[0xF5] = CodeEntry { code: 0xF5, mnemonic: "EXTENSION", category: "escape" };
    table[0xF6] = CodeEntry { code: 0xF6, mnemonic: "EXT_ACK", category: "escape" };
    table[0xF7] = CodeEntry { code: 0xF7, mnemonic: "EXT_NACK", category: "escape" };
    table[0xF8] = CodeEntry { code: 0xF8, mnemonic: "CODEBOOK_DEF", category: "escape" };
    table[0xF9] = CodeEntry { code: 0xF9, mnemonic: "CODEBOOK_ACK", category: "escape" };
    table[0xFA] = CodeEntry { code: 0xFA, mnemonic: "CODEBOOK_NACK", category: "escape" };
    table[0xFB] = CodeEntry { code: 0xFB, mnemonic: "STREAM_ID", category: "escape" };
    table[0xFC] = CodeEntry { code: 0xFC, mnemonic: "XREF", category: "escape" };
    table[0xFD] = CodeEntry { code: 0xFD, mnemonic: "COMMENT", category: "escape" };
    table[0xFE] = CodeEntry { code: 0xFE, mnemonic: "NOP", category: "escape" };
    table[0xFF] = CodeEntry { code: 0xFF, mnemonic: "RESERVED_FF", category: "escape" };

    table
};
