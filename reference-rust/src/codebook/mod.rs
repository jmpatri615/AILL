pub mod base;
pub mod nav;
pub mod percept;
pub mod diag;
pub mod plan;

pub use base::*;

/// A domain codebook entry.
#[derive(Debug, Clone)]
pub struct DomainEntry {
    pub code: u16,
    pub mnemonic: &'static str,
    pub value_type: &'static str,
    pub unit: &'static str,
    pub description: &'static str,
}

/// A domain codebook with registry ID and entries.
pub struct DomainCodebook {
    pub registry_id: u8,
    pub name: &'static str,
    entries: &'static [DomainEntry],
}

impl DomainCodebook {
    pub const fn new(registry_id: u8, name: &'static str, entries: &'static [DomainEntry]) -> Self {
        Self { registry_id, name, entries }
    }

    pub fn lookup(&self, code: u16) -> Option<&DomainEntry> {
        self.entries.iter().find(|e| e.code == code)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[DomainEntry] {
        self.entries
    }
}

/// Static domain codebook instances.
pub static NAV1: DomainCodebook = DomainCodebook::new(
    nav::NAV1_REGISTRY_ID,
    nav::NAV1_NAME,
    nav::NAV1_ENTRIES,
);

pub static PERCEPT1: DomainCodebook = DomainCodebook::new(
    percept::PERCEPT1_REGISTRY_ID,
    percept::PERCEPT1_NAME,
    percept::PERCEPT1_ENTRIES,
);

pub static DIAG1: DomainCodebook = DomainCodebook::new(
    diag::DIAG1_REGISTRY_ID,
    diag::DIAG1_NAME,
    diag::DIAG1_ENTRIES,
);

pub static PLAN1: DomainCodebook = DomainCodebook::new(
    plan::PLAN1_REGISTRY_ID,
    plan::PLAN1_NAME,
    plan::PLAN1_ENTRIES,
);

/// All registered domain codebooks.
pub static DOMAIN_REGISTRY: &[&DomainCodebook] = &[&NAV1, &PERCEPT1, &DIAG1, &PLAN1];

/// Look up a domain codebook by registry ID.
pub fn get_domain_codebook(registry_id: u8) -> Option<&'static DomainCodebook> {
    DOMAIN_REGISTRY.iter().find(|cb| cb.registry_id == registry_id).copied()
}
