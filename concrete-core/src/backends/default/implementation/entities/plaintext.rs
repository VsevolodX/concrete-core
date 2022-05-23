use crate::commons::crypto::encoding::Plaintext as ImplPlaintext;
use crate::specification::entities::markers::PlaintextKind;
use crate::specification::entities::{AbstractEntity, PlaintextEntity};

/// A structure representing a plaintext with 32 bits of precision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plaintext32(pub(crate) ImplPlaintext<u32>);
impl AbstractEntity for Plaintext32 {
    type Kind = PlaintextKind;
}
impl PlaintextEntity for Plaintext32 {}

/// A structure representing a plaintext with 64 bits of precision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plaintext64(pub(crate) ImplPlaintext<u64>);
impl AbstractEntity for Plaintext64 {
    type Kind = PlaintextKind;
}
impl PlaintextEntity for Plaintext64 {}
