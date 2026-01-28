use super::ObjectHash;
use crate::errors::StorageError;

pub trait ObjectStore {
    fn put(&mut self, hash: ObjectHash, data: impl AsRef<[u8]>) -> Result<(), StorageError>;
    fn get(&self, hash: &ObjectHash) -> Result<Vec<u8>, StorageError>;
    //fn contains(&self, hash: &ObjectHash) -> Result<bool, StorageError>;
    //fn all_hashes(&self) -> Result<Vec<ObjectHash>, StorageError>;
}
