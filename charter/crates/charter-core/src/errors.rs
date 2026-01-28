use thiserror::Error;

#[derive(Error, Debug )]
pub enum StorageError {
    #[error("Failed to generate has digest")]
    HashGeneration,
    #[error("Hash not found")]
    HashNotFound,
}
