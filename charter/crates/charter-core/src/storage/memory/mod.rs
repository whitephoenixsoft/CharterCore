/*use std::collections::HashMap;
use crate::types::ObjectHash;
use crate::types::hash_object;
use crate::storage::core::CharterObject;
use crate::storage::object_store::ObjectStore;

pub struct MemoryObjectStore {
    objects: HashMap<ObjectHash, Box<dyn CharterObject>>,
}

impl MemoryObjectStore {
    pub fn new() {
        Self {
            HashMap::new()
        }
    }
}

impl ObjectStore for MemoryObjectStore {
    pub fn put(&mut self, obj: Box<dyn CharterObject>) -> ObjectHash {
        let content = obj.canonical_bytes();
        let bytes = build_hash_bytes(
            HashVersion::V1,
            obj.kind(),
            &content,
        );

        let hash = hash_bytes(&bytes);
        self.objects.insert(hash.clone(), obj);
        hash
    }

    pub fn get(&self, hash: &ObjectHash) -> Option<Box<dyn CharterObject>> {
        self.objects.get(hash)
    }
}
*/
