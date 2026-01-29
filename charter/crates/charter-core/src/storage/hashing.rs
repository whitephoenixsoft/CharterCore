use super::ObjectHash;
use super::CharterObjectKind;
use serde::{Serialize, Deserialize};
use serde::de::Error;
use serde_json_canonicalizer::to_vec;
use serde_json::Value;
use strum::Display;
use std::fmt;
use crate::errors::StorageError;

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum HashVersion {
    V1,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
}

pub struct HashInput<'a> {
    pub version: HashVersion,
    pub algorithm: HashAlgorithm,
    pub object_type: String,
    pub canonical_json: &'a [u8],
}

impl<'a> HashInput<'a> {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend(format!("charter:{}\n", self.version.to_string()).as_bytes());
        bytes.extend(format!("type:{}\n", self.object_type).as_bytes());
        bytes.extend(format!("len:{}\n", self.canonical_json.len()).as_bytes());
        bytes.extend(&*self.canonical_json);
        
        bytes
    }
}

pub fn extract_enum_tag<T: Serialize>(payload: &T, tag: &str) -> Result<String, StorageError> {
    let value = serde_json::to_value(payload).map_err(|_| StorageError::HashGeneration); 
    match value?.get(tag) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(StorageError::HashGeneration),
    }
}

pub fn get_canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, StorageError> {
    let json_bytes = to_vec(value).map_err(|_| StorageError::HashGeneration);
    json_bytes
}

pub fn compute_hash(algorithm: &HashAlgorithm, bytes: &Vec<u8>) -> Result<ObjectHash, StorageError> {
    let object_hash: ObjectHash;
    match algorithm {
        HashAlgorithm::Sha256 => {
            use sha2::{Digest, Sha256};
        
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            
            object_hash = ObjectHash(format!("{:x}", hasher.finalize()))
        }
        _ => return Err(StorageError::HashGeneration)
    }
    
    Ok(object_hash)
}

pub fn hash_object<T: Serialize>(hash_version: HashVersion, hash_algorithm: HashAlgorithm, value: &T) -> Result<ObjectHash, StorageError> {
    let enum_name = extract_enum_tag(value, "type")?;
    let json = get_canonical_json(value)?;

    let bytes = HashInput {
        version: hash_version,
        algorithm: hash_algorithm,
        object_type: enum_name,
        canonical_json: &json,
    }.as_bytes();

    compute_hash(&hash_algorithm, &bytes)
}


#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    #[test]
    pub fn test_hashversion_v1() {
        assert_eq!("v1", HashVersion::V1.to_string());
    }

    #[test]
    pub fn test_hashalgorithm_sha256() {
        assert_eq!("sha256", HashAlgorithm::Sha256.to_string());
    }

    #[test]
    pub fn test_hashinput_asbytes() {
        let input = HashInput {
            version: HashVersion::V1,
            algorithm: HashAlgorithm::Sha256,
            object_type: "test".to_string(),
            canonical_json: "{}".as_bytes(),
        };
        let expected = b"charter:v1\ntype:test\nlen:2\n{}";

        assert_eq!(input.as_bytes(), expected.to_vec());
    }

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
    }

    #[test]
    pub fn test_extract_enum_tag_when_tag_exists() {
        const TAG_VALUE: &str  = "success";
        let test = TestStruct {
            name: String::from(TAG_VALUE),
        };

        let result = extract_enum_tag(&test, "name").ok();

        assert_eq!(Some(String::from(TAG_VALUE)), result);
    }

    #[test]
    pub fn test_extract_enum_tag_when_tag_does_not_exist() {
        const TAG_VALUE: &str  = "success";
        let test = TestStruct {
            name: String::from(TAG_VALUE),
        };

        assert!(extract_enum_tag(&test, "a").is_err());
    }
}

