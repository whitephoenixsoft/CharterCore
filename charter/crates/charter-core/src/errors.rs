use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Could not create area")]
    CreateAreaError,
}

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
