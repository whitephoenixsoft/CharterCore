#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ObjectHash(pub String);

pub type Label = String;
pub type DisplayName = String;
pub type Annotation = String;

pub fn hash_bytes(bytes: &[u8]) -> String {
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn hash_object<T: serde::Serialize>(value: &T) -> Result<ObjectHash, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    let hash = hash_bytes(&bytes);
    Ok(ObjectHash(hash))
}
