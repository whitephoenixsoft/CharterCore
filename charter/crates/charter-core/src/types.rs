use serde::Serialize;
use strum::Display;

pub trait CharterObject {
    fn object_type(&self) -> CharterObjectKind;
}

#[derive(Debug, Clone, Copy, Display)]
#[strum(serialize_all = "lowercase")]
pub enum CharterObjectKind {
    Area,
    //Session,
    //Resolution,
}


#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    #[test]
    pub fn to_string_area() {
        assert_eq!("area", CharterObjectKind::Area.to_string())
    }
}

