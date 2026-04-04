use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    InitializationFailed,
    InvalidInput(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::InitializationFailed => {
                write!(f, "engine initialization failed")
            }
            EngineError::InvalidInput(msg) => {
                write!(f, "invalid input: {}", msg)
            }
        }
    }
}

impl std::error::Error for EngineError {}
