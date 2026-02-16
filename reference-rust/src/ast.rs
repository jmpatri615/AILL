use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Literal value types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float16(f32),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Timestamp(i64),
    Null,
}

/// AST node types for decoded AILL expressions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "node_type")]
pub enum AstNode {
    Utterance {
        meta: MetaHeader,
        body: Vec<AstNode>,
    },
    Literal {
        value_type: String,
        value: LiteralValue,
    },
    Struct {
        fields: BTreeMap<u16, AstNode>,
    },
    List {
        count: u16,
        elements: Vec<AstNode>,
    },
    Map {
        count: u16,
        pairs: Vec<(AstNode, AstNode)>,
    },
    Pragmatic {
        act: String,
        expression: Box<AstNode>,
    },
    Modal {
        modality: String,
        expression: Box<AstNode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<f64>,
    },
    Temporal {
        modifier: String,
        expression: Box<AstNode>,
    },
    DomainRef {
        level: u8,
        domain_code: u16,
    },
    ContextRef {
        sct_index: u32,
    },
    Code {
        code: u8,
        mnemonic: String,
    },
    Annotated {
        code: u8,
        mnemonic: String,
    },
}

/// Decoded meta header.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaHeader {
    pub confidence: f32,
    pub priority: u8,
    pub timestamp_us: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_agent: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_agent: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seqnum: Option<u32>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, AnnotationValue>,
}

impl Default for MetaHeader {
    fn default() -> Self {
        Self {
            confidence: 1.0,
            priority: 3,
            timestamp_us: 0,
            source_agent: None,
            dest_agent: None,
            seqnum: None,
            annotations: BTreeMap::new(),
        }
    }
}

/// Values that can appear in meta annotations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnnotationValue {
    U16(u16),
    U64(u64),
    Pair(u16, u16),
}

/// A decoded epoch with verified CRC.
#[derive(Debug, Clone)]
pub struct DecodedEpoch {
    pub seq_num: u16,
    pub payload: Vec<u8>,
    pub crc_ok: bool,
}
