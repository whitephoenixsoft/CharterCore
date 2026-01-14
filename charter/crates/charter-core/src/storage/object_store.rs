use super::core::{
	   ObjectHash, ObjectData
};

pub trait ObjectStore {
	   fn put(&mut self, obj: ObjectData) -> ObjectHash;
	   fn get(&self, hash: &ObjectHash) -> Option<&ObjectData>;
	   //fn contains(&self, hash: &ObjectHash) -> bool;
	   //fn all_hashes(&self) -> Vec<ObjectHash>;
}


