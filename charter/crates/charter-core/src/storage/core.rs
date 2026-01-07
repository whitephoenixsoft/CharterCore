use crate::model::ids::AreaId;
use crate::time::Timestamp;
use serde::Serialize;
use crate::ids::ObjectHash;
use crate::types:hash_object;

#[derive(Debug, Serialize)]
pub trait CharterObject {
    /// Canonical byte representation used for hashing
    fn canonical_bytes(&self) -> Vec<u8>;

    /// Logical object kind (for debugging / tooling)
    fn kind(&self) -> CharterObjectKind;
}

pub enum CharterObjectKind {
    Area,
    Session,
    Candidate,
    Resolution,
}


#[derive(Debug, Serialize)]
pub struct AreaRoot {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}

impl CharterObject for AreaRoot {
    fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self)
            .expect("AreaRoot must serialize deterministically")
    }

    fn kind(&self) -> CharterObjectKind {
        CharterObjectKind::Area
    }
}
