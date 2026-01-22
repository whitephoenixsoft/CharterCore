use super::ObjectHash;
use crate::model::ids::AreaId;

/// List of types with each having a ref path.
pub enum RefName {
    AreaRef { area_id: AreaId, path: String },
}

pub trait RefStore {
    fn set_ref(&mut self, name: RefName, hash: ObjectHash);
    fn get_ref(&self, name: &RefName) -> Option<&ObjectHash>;
    //fn delete_ref(&mut self, name: &RefName);
    //fn list_refs(&self) -> Vec<(RefName, ObjectHash)>;
}
