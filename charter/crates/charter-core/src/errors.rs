use thiserror::Error;

#[derive(Error, Debug )]
pub enum ObjectStorageError {
    #[error("Failed to generate has digest")]
    HashGeneration,
}
