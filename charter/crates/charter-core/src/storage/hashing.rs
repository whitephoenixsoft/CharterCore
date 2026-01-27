use super::ObjectHash;
use super::CharterObjectKind;
use serde::{Serialize, Deserialize};
use serde::de::Error;
use serde_json_canonicalizer::to_vec;
use serde_json::Value;
use strum::Display;
use std::fmt;

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
    pub fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let mut bytes = Vec::new();
        
        bytes.extend(format!("charter:{}\n", self.version.to_string()).as_bytes());
        bytes.extend(format!("type:{}\n", self.object_type).as_bytes());
        bytes.extend(format!("len:{}\n", self.canonical_json.len()).as_bytes());
        bytes.extend(&*self.canonical_json);
        
        Ok(bytes)
    }
}

pub fn extract_enum_tag<T: Serialize>(payload: &T, tag: &str) -> Result<String, serde_json::Error> {
    let value = serde_json::to_value(payload)?; 
    match value.get(tag) {
        Some(Value::String(s)) => Ok(s.clone()),
        _ => Err(serde_json::Error::custom("missing enum tag")),
    }
}

pub fn get_canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    let json_bytes = to_vec(value);
    json_bytes
}

pub fn compute_hash(algorithm: &HashAlgorithm, bytes: &Vec<u8>) -> ObjectHash {
    let object_hash: ObjectHash;
    match algorithm {
        HashAlgorithm::Sha256 => {
            use sha2::{Digest, Sha256};
        
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            
            object_hash = ObjectHash(format!("{:x}", hasher.finalize()))
        }
    }
    
    object_hash
}

pub fn hash_object<T: Serialize>(hash_version: HashVersion, hash_algorithm: HashAlgorithm, value: &T) -> Result<ObjectHash, serde_json::Error> {
    let enum_name = extract_enum_tag(value, "type")?;
    let json = get_canonical_json(value)?;

    let bytes = HashInput {
        version: hash_version,
        algorithm: hash_algorithm,
        object_type: enum_name,
        canonical_json: &json,
    }.as_bytes()?;

    Ok(compute_hash(&hash_algorithm, &bytes))
}


#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

    #[test]
    pub fn test_hashversion_v1() {
        assert_eq!("v1", HashVersion::V1.to_string());
    }

    #[test]
    pub fn test_hastalgorithm_sha256() {
        assert_eq!("sha256", HashAlgorithm::Sha256.to_string());
    }
}

