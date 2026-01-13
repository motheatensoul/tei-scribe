use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SagaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("XML error: {0}")]
    Xml(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Command error: {0}")]
    Command(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Internal error: {0}")]
    Internal(String),
}

// We need to implement Serialize for SagaError so it can be returned via Tauri commands
impl Serialize for SagaError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SagaError>;
