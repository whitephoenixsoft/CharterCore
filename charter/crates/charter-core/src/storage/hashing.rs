use super::ids::ObjectHash;
use super::types::CharterObjectType;
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum HashVersion {
    V1,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HashAlgorithm {
    Sha256,
}

struct HashInput<'a> {
    pub version: HashVersion,
    pub object_type: CharterObjectType,
    pub canonical_json: &'a [u8],
}

pub struct ObjectEnvelope<T> {
    pub hash_version: HashVersion,
    pub hash_algorithm: HashAlgorithm,
    pub object_type: CharterObjectType,
    pub object_hash: ObjectHash,
    pub object: T,
}

pub fn canonical_json<T: Serialize>(value: &T) -> Vec<u8> {
    // single implementation
    // sorted keys, no whitespace, stable
    let json = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    json
}

pub fn compute_hash(input: HashInput) -> ObjectHash {
    let mut bytes = Vec::new();
    bytes.extend(format!("charter:{:?}\n", input.version).as_bytes());
    bytes.extend(format!("type:{:?}\n", charter_object_type_string(input.object_type).as_bytes());
    bytes.extend(format!("len:{:?}\n", input.canonical_json.len()).as_bytes());
    bytes.extend(&input.canonical_json);

    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    
    ObjectHash(format!("{:x}", hasher.finalize()))
}

pub fn hash_object<T: Serialize>(object_type: CharterObjectType, value: &T) -> ObjectHash {
    let json = canonical_json(value);

    let input = HashInput {
        version: HashVersion::V1,
        object_type,
        canonical_json: &json,
    };

    compute_hash(&input)
}

pub fn envelope_object<T: Serialize>(
    object_type: CharterObjectType,
    value: T,
) -> ObjectEnvelope<T> {
    let hash = hash_object(object_type, &value);

    ObjectEnvelope {
        hash_version: HashVersion::V1,
        hash_algorithm: HashAlgorithm::Sha256,
        object_type,
        object_hash: hash,
        object: value,
    }
}
