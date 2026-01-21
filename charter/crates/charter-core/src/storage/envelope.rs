use super::core::{
	   ObjectHash, ObjectData
};
use super::hashing::{
    HashVersion, HashAlgorithm, hash_object, get_canonical_json
};
use crate::types::CharterObjectKind;
use serde::Serialize;

#[derive(Serialize)]
pub struct ObjectEnvelope {
    pub hash_version: HashVersion,
    pub hash_algorithm: HashAlgorithm,
    pub object_hash: ObjectHash,
	   #[serde(flatten)]
    pub object: CharterObjectKind,
}

/*
impl<T> ObjectEnvelope<T> where T : Serialize {
    pub fn new(
        self,
        object_type: CharterObjectKind,
        value: T,
    ) -> Self {
    	   let hash_version = HashVersion::V1;
    	   let hash_algorithm = HashAlgorithm::Sha256;
    	   let hash = hash_object(hash_version, hash_algorithm, object_type, &value)
    	       .expect("Failed go generate hash digest");
    
        Self {
            hash_version,
            hash_algorithm,
            object_type,
            object_hash: hash,
            object: value,
        }
    }
	   
	    pub fn as_bytes(&self) -> Result<Vec<u8>, String> {
        get_canonical_json(&self.object)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    #[test]
    pub fn test_as_bytes() {
    	   #[define(Serialize)]
    	   struct TestObject { value: String };
    	   let test = TestObject { value: "a" };
    	   let envelope = ObjectEnvelope::new();
    	   let expected = b
    	   assert_eq!("v1", HashVersion::V1.to_string());
    }
}
