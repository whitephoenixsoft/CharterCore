use super::ObjectHash;

pub enum RefName {
    Area(String),
}

pub trait RefStore {
    fn set_ref(&mut self, name: RefName, hash: ObjectHash);
    fn get_ref(&self, name: &RefName) -> Option<&ObjectHash>;
    //fn delete_ref(&mut self, name: &RefName);
    //fn list_refs(&self) -> Vec<(RefName, ObjectHash)>;
}
