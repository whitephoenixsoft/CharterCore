use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use serde_json::Value;

pub struct OneFileObjectStore {
    root: PathBuf,
    filename: String
}

impl OneFileObjectStore {
    pub fn new(root: impl AsRef<Path>, filename: String) -> std::io::Result<Self> {
        fs::create_dir_all(&root)?;
        Ok(Self {
            root: root.as_ref().to_path_bug(),
            filename: filename.clone(),
        })
    }
}

impl ObjectStore for OneFileObjectStore {
    fn put(&mut self, hash: ObjectHash, data: impl AsRef<[u8]>) -> Result<(), StorageError> {
        let path = self.root.join(filename);

        let mut file: fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect(StorageError::FileOpenError);
        
        file.write_all(data.as_ref()).expect(StorageError::FileWriteError);

        Ok(())
    }

    fn get(&self, hash: &ObjectHash) -> Result<Vec<u8>, StorageError> {
        let path = self.root.join(filename);

        let mut file = fs::File::open(path).expect(StorageError::FileOpenError);
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line_content) = line {
                if let Ok(json) = serde_json::from_str::<Value>(&line_content) {
                    if json["object_hash"] == hash {
                        return Ok(json);
                    }
                }
            }
        }
        Err(StorageError::HashNotFound)
    }

       
