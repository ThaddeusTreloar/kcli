pub mod config;
pub mod consume;
pub mod util;


#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(&'static str),
}