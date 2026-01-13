use serde::Serialize;
use strum_macros::Display;

#[derive(Debug, Serialize)]
pub trait CharterObject {
    fn object_type(&self) -> CharterObjectType;
}

#[derive(Debug, Clone, Copy, Display)]
#[strum(serialize_all = "lowercase")]
pub enum CharterObjectType {
    Area,
    Session,
    Resolution,
}
