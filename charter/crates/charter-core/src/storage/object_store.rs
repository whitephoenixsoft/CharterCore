use crate::ids::ObjectId;

pub trait ObjectStore {
	fn put(&mut self, obj: Vec<u8>) -> ObjectId;
	fn get(&self, id: &ObjectId) -> Option<Vec<u8>>;
}
