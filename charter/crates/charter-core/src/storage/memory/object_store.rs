use crate::storage::ObjectHash;
use crate::storage::object_store::ObjectStore;
use crate::errors::StorageError;
use std::collections::HashMap;
use std::cell::RefCell;


pub struct MemoryObjectStore {
    storage: RefCell<HashMap<ObjectHash, Vec<u8>>>,
}

impl MemoryObjectStore {
    pub fn new() -> Self {
        Self { storage: RefCell::new(HashMap::new()) }
    }
}

impl ObjectStore for MemoryObjectStore {
    fn put(&mut self, hash: ObjectHash, data: impl AsRef<[u8]>) -> Result<(), StorageError> {
        let input = data.as_ref(); 
        self.storage.borrow_mut().insert(hash, input.clone());
        Ok(())
    }

    fn get(&self, hash: &ObjectHash) -> Result<Vec<u8>, StorageError> {
        self.storage.borrow().get(hash)
            .cloned()
            .ok_or_else(|| StorageError::HashNotFound)
    }
}
