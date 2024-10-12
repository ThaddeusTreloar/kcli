use thiserror::Error;

pub mod clusters;

#[derive(Debug, Error)]
pub enum InitContextError {
    #[error("Failed to check existence of: {0}")]
    CheckExistence(String),
    #[error("Failed to create directory: {0}")]
    CreateDirectory(String),
    #[error("Failed to create file: {0}")]
    CreateFile(String),
    #[error("Failed to write file: {0}")]
    WriteFile(String),
    #[error("Failed to read file: {0}")]
    ReadFile(String),
    #[error("Failed to load config: {0}")]
    LoadConfig(&'static str),
}

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Failed to get path: {0}")]
    GetPath(String),
}
