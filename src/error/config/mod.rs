use thiserror::Error;

pub (crate) mod clusters;

#[derive(Debug, Error)]
pub(crate) enum InitConfigError {
    #[error("Failed to check existence of: {0}")]
    CheckExistence(String),
    #[error("Failed to create directory: {0}")]
    CreateDirectory(String),
    #[error("Failed to create file: {0}")]
    CreateFile(String),
    #[error("Failed to write file: {0}")]
    WriteFile(String),
}