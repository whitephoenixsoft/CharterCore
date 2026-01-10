use crate::types::ObjectHash;
use super::core::CharterObject;

pub trait ObjectStore {
	fn put(&mut self, obj: Box<dyn CharterObject>) -> ObjectHash;
	fn get(&self, hash: &ObjectHash) -> Option<&Box<dyn CharterObject>>;
        //fn contains(&self, hash: &ObjectHash) -> bool;
        //fn all_hashes(&self) -> Vec<ObjectHash>;
}
