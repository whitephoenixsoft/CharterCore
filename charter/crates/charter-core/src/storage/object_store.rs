use crate::ids::ObjectHash;

pub trait ObjectStore {
	fn put(&mut self, obj: Vec<u8>) -> ObjectHash;
	fn get(&self, id: &ObjectHash) -> Option<Vec<u8>>;
}
