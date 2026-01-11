
#[derive(Debug, Clone, Copy)]
pub enum CharterObjectType {
    Area,
    Session,
    Resolution,
}

fn charter_object_type_string(object_type: CharterObjectType) -> &'static str {
    match object_type{
        CharterObjectType::Area => "area",
        CharterObjectType::Session => "session",
        CharterObjectType::Resolution => "resolution",
    }
}
