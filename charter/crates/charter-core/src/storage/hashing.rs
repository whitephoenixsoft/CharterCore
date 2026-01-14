use super::core::ObjectHash;
use crate::types::CharterObjectType;
use serde::Serialize;
use serde_json_canonicalizer::to_vec;

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
    pub algorithm: HashAlgorithm,
    pub object_type: CharterObjectType,
    pub canonical_json: &'a [u8],
}

pub fn get_canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    // single implementation
    // sorted keys, no whitespace, stable
    let result = to_vec(value);
    match result {
        Ok(json) => Ok(json),
        Err(e) => Err(e.to_string())
    }
}

fn compute_hash(input: &HashInput) -> Result<ObjectHash, String> {
    let mut bytes = Vec::new();
    bytes.extend(format!("charter:{:?}\n", input.version).as_bytes());
    bytes.extend(format!("type:{:?}\n", input.object_type.to_string()).as_bytes());
    bytes.extend(format!("len:{:?}\n", input.canonical_json.len()).as_bytes());
    bytes.extend(&*input.canonical_json);

    let object_hash: ObjectHash;
    match input.algorithm {
        HashAlgorithm::Sha256 => {
            use sha2::{Digest, Sha256};
        
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            
            object_hash = ObjectHash(format!("{:x}", hasher.finalize()))
        }
    }
    
    Ok(object_hash)
}

pub fn hash_object<T: Serialize>(hash_version: HashVersion, hash_algorithm: HashAlgorithm, object_type: CharterObjectType, value: &T) -> Result<ObjectHash, String> {
    let json = get_canonical_json(value)?;

    let input = HashInput {
        version: hash_version,
        algorithm: hash_algorithm,
        object_type: object_type,
        canonical_json: &json,
    };

    compute_hash(&input)
}

#[cfg(test)]
mod tests {
    use super::*; // Imports everything from the parent module

}