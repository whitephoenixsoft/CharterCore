use std::collections::HashMap;
use crate::types::ObjectHash;
use crate::types::hash_object;
use crate::storage::core::CharterObject;
use crate::storage::object_store::ObjectStore;

pub struct MemoryObjectStore {
    objects: HashMap<CharterObject, ObjectHash>;
}

impl MemoryObjectStore {
    pub fn new() {
        Self {
            HashMap::new()
        }
    }
}

impl ObjectStore for MemoryObjectStore {
    pub fn put(&mut self, obj: CharterObject) -> ObjectHash {
        let hash = hash_object(obj);
        self.objects.insert(hash, CharterObject);
        hash
    }

    pub fn get(&self, hash: &ObjectHash) -> Option<&CharterObject> {
        selft.objects.get(hash)
    }
}

