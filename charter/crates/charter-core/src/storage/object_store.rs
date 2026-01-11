use super::core::ObjectHash;
use crate::types::CharterObjectType;

pub trait ObjectStore {
	fn put(&mut self, obj: CharterObjectType) -> ObjectHash;
	fn get(&self, hash: &ObjectHash) -> Option<&CharterObjectType>;
        //fn contains(&self, hash: &ObjectHash) -> bool;
        //fn all_hashes(&self) -> Vec<ObjectHash>;
}