use super::core::{
	   ObjectHash
};
use super::object_store::ObjectStore;
use std::collections::HashMap;
use std::cell::RefCell;


pub struct MemoryStore {
    storage: RefCell<HashMap<ObjectHash, String>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self { storage: RefCell::new(HashMap::new()) }
    }
}

impl ObjectStore for MemoryStore {
    fn put(&mut self, hash: ObjectHash, data: String) -> Result<(), String> {
        self.storage.borrow_mut().insert(hash, data);
        Ok(())
    }

    fn get(&self, hash: &ObjectHash) -> Result<String, String> {
        self.storage.borrow().get(hash)
            .cloned()
            .ok_or_else(|| "Hash not found".to_string())
    }
}
