use super::ObjectHash;

pub trait ObjectStore {
    fn put(&mut self, hash: ObjectHash, data: String) -> Result<(), String>;
    fn get(&self, hash: &ObjectHash) -> Result<String, String>;
    //fn contains(&self, hash: &ObjectHash) -> bool;
    //fn all_hashes(&self) -> Vec<ObjectHash>;
}
