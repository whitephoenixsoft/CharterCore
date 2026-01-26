pub mod area;
pub mod audit_store;
pub mod core;
pub mod envelope;
pub mod hashing;
pub mod memory;
pub mod metadata_store;
pub mod object_store;
pub mod ref_store;

pub use area::AreaObject;

use crate::model::CharterModelKind;
use crate::model::ids::*;
use serde::{Serialize, Deserialize};
use strum::Display;
use enum_dispatch::enum_dispatch;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectHash(pub String);

impl From<String> for ObjectHash {
    fn from(s: String) -> Self {
        ObjectHash(s.clone())
    }
}


#[derive(Debug, Clone, Display, Serialize, Deserialize)]
//#[strum(serialize_all = "lowercase")]
#[serde(tag = "object_type", content = "object", rename_all = "lowercase")]
pub enum CharterObjectKind {
    Area(AreaObject),
    //Session,
    //Resolution,
}

// Map Domain -> Persistence
impl From<CharterModelKind> for CharterObjectKind {
    fn from(o: CharterModelKind) -> Self {
        match o {
            CharterModelKind::Area(a) => CharterObjectKind::Area(AreaObject::from(a)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    /*
    #[test]
    pub fn to_string_area() {
        assert_eq!("area", CharterObjectKind::Area.to_string())
    }
    */
}
