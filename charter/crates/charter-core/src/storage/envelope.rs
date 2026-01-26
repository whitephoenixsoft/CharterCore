use super::ObjectHash;
use super::hashing::{
    HashVersion, HashAlgorithm, hash_object, get_canonical_json
};
use super::CharterObjectKind;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectEnvelope<T> {
    pub hash_version: HashVersion,
    pub hash_algorithm: HashAlgorithm,
    pub object_hash: ObjectHash, 
    pub object: T,
}

impl ObjectEnvelope<CharterObjectKind> {
    pub fn get_hash_version() -> HashVersion {
        HashVersion::V1
    }

    pub fn get_hash_algorithm() -> HashAlgorithm {
        HashAlgorithm::Sha256
    }

    pub fn new(
        object: CharterObjectKind
    ) -> Result<Self, serde_json::Error> {
        let hash_version = Self::get_hash_version();
        let hash_algorithm = Self::get_hash_algorithm();
        let hash = hash_object(hash_version, hash_algorithm, &object)?;
    
        Self {
            hash_version,
            hash_algorithm,
            object_hash: hash,
            object,
        }
    }
	   
    pub fn verify(&self) -> Result<bool, serde_json::Error> {
        let hash_version = Self::get_hash_version();
        let hash_algorithm = Self::get_hash_algorithm();
        let recomputed  = hash_object(hash_version, hash_algorithm, &object)?;
        Ok(self.object_hash == recomputed)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    /*
    #[test]
    pub fn test_as_bytes() {
    	   #[define(Serialize)]
    	   struct TestObject { value: String };
    	   let test = TestObject { value: "a" };
    	   let envelope = ObjectEnvelope::new();
    	   let expected = b
    	   assert_eq!("v1", HashVersion::V1.to_string());
    }
    */
}
