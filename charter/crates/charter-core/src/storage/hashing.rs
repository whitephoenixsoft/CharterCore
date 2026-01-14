use super::core::ObjectHash;
use crate::types::CharterObjectType;
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
    pub algorithm: HashAlgorithm,
    pub object_type: CharterObjectType,
    pub canonical_json: &'a [u8],
}

fn get_canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    // single implementation
    // sorted keys, no whitespace, stable
    let json = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(json)
}

fn compute_hash(input: &HashInput) -> Result<ObjectHash, String> {
    let mut bytes = Vec::new();
    bytes.extend(format!("charter:{:?}\n", input.version).as_bytes());
    bytes.extend(format!("type:{:?}\n", input.object_type.to_string()).as_bytes());
    bytes.extend(format!("len:{:?}\n", input.canonical_json.len()).as_bytes());
    bytes.extend(&*input.canonical_json);

    let mut object_hash: ObjectHash;
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

    #[derive(Serialize)]
    struct ComplexState {
        // Requirements checklist:
        // 1. Lexicographical sorting: "beta" should come after "alpha"
        // 2. Unambiguous numbers: 3000.0 should become 3000
        beta: f64,
        alpha: String,
        // 3. Arrays: Order must be preserved
        data_points: Vec<u32>,
        // 4. Nested Objects: Keys must be sorted recursively
        metadata: Metadata,
    }
    
    #[derive(Serialize)]
    struct Metadata {
        version: u8,
        author: String,
    }
    
    #[test]
    fn test_canonical_json_requirements() {
        let state = ComplexState {
            beta: 3000.0,
            alpha: "engine_v1".to_string(),
            data_points: vec![10, 20, 30],
            metadata: Metadata {
                version: 1,
                author: "Admin".to_string(),
            },
        };

        //let result_bytes = to_vec(&state).expect("Failed to canonicalize");
        let result_bytes = get_canonical_json(&state);
        let result_str = std::str::from_utf8(&result_bytes).expect("Not valid UTF-8");

        // REQUIREMENT CHECK:
        // - "alpha" comes before "beta" (Lexicographical)
        // - "author" comes before "version" inside "metadata" (Recursive Lexicographical)
        // - 3000.0 is serialized as 3000 (Unambiguous Number)
        // - No spaces after colons or commas (No whitespace)
        // - Arrays [10,20,30] remain in that exact order
        let expected = r#"{"alpha":"engine_v1","beta":3000,"data_points":[10,20,30],"metadata":{"author":"Admin","version":1}}"#;

        assert_eq!(result_str, expected);
    }
}