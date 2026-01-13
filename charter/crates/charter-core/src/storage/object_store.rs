use super::core::{
	   ObjectHash, ObjectData
};
use super::hashing::{
	   HashVersion, HashAlgorithm, hash_object
};
use crate::types::CharterObjectType;
use serde::Serialize;

pub trait ObjectStore {
	   fn put(&mut self, obj: ObjectData) -> ObjectHash;
	   fn get(&self, hash: &ObjectHash) -> Option<&ObjectData>;
	   //fn contains(&self, hash: &ObjectHash) -> bool;
	   //fn all_hashes(&self) -> Vec<ObjectHash>;
}

pub struct ObjectEnvelope<T: Serialize> {
    pub hash_version: HashVersion,
    pub hash_algorithm: HashAlgorithm,
    pub object_type: CharterObjectType,
    pub object_hash: ObjectHash,
    pub object: T,
}

impl<T> ObjectEnvelope<T> where T : Serialize {
    pub fn new(
        self,
        object_type: CharterObjectType,
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
}


