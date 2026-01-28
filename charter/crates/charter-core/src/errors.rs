use thiserror::Error;

#[derive(Error, Debug )]
pub enum StorageError {
    #[error("Failed to generate has digest")]
    HashGeneration,
    #[error("Hash not found")]
    HashNotFound,
    #[error("Could not open file")]
    FileOpenError,
    #[error("Could not write file")]
    FileWriteError,
}
