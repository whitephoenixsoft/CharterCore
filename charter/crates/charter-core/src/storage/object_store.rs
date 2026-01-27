use super::ObjectHash;

pub trait ObjectStore {
    fn put(&mut self, hash: ObjectHash, data: &Vec<u8>) -> Result<(), String>;
    fn get(&self, hash: &ObjectHash) -> Result<Vec<u8>, String>;
    //fn contains(&self, hash: &ObjectHash) -> bool;
    //fn all_hashes(&self) -> Vec<ObjectHash>;
}
