use crate::model::ids::AreaId;
use crate::time::Timestamp;
use serde::Serialize;
//use crate::ids::ObjectHash;

/*
pub trait CharterObject {
    /// Canonical bytes used for hashing and storage
    pub fn canonical_bytes(&self) -> Vec<u8>;

    /// Deterministic object hash
    pub fn object_hash(&self) -> ObjectHash {
        hash_bytes(&self.canonical_bytes())
    }

    /// Logical object kind (for debugging / tooling)
    pub fn kind(&self) -> CharterObjectKind;
}

pub enum CharterObjectKind {
    Area,
    Session,
    Candidate,
    Resolution,
}
*/
pub enum CharterObject {
    AreaRoot(AreaRoot),
}

#[derive(Debug)]
pub struct AreaRoot {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}
