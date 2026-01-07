use std::collections::HashMap;
use crate::types::ObjectHash;
use crate::types::hash_bytes;
use crate::storage::core::CharterObject;
use crate::storage::object_store::ObjectStore;

pub struct MemoryObjectStore {
    objects: HashMap<ObjectHash, Box<dyn CharterObject>>;
}

impl MemoryObjectStore {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new()
        }
    }

    pub fn count(&self) -> usize {
        self.objects.len()
    }

    pub fn has_object(&self, hash: ObjectHash) -> bool {
        self.objects.contains_key(&hash)
    }
}

impl ObjectStore for MemoryObjectStore {
    pub fn put(&mut self, obj: Box<dyn CharterObject>) -> ObjectHash {
        let hash = hash_bytes(*obj.canonical_bytes());
        let hash = ObjectHash(hash);
        self.objects.insert(hash, obj);
        hash
    }

    pub fn get(&self, hash: &ObjectHash) -> Option<&Box<dyn CharterObject>> {
        self.objects.get(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ids::AreaId;

    fn get_area_id(label: &str) -> AreaId {
       AreaId(label)
    }

    fn get_timestamp() -> String {
        "2026-01-01T00:00:00.00Z".to_string()
    }

    fn get_area_root(area_id: AreaId) -> AreaRoot {
        AreaRoot {
            area_id,
            created_at: get_timestamp()
        }
    }

    fn get_test_object(label: &str) -> CharterObject {
            get_area_root(
                get_area_id(label)
            )
    }

    #[test]
    fn put_when_new_object_then_returns_hash() {
        let mut engine = MemoryObjectStore::new();
        let obj = get_test_object("some id");
        let hash = engine.put(Box::new(obj));

        assert(!hash.0.is_empty());
    }
}
