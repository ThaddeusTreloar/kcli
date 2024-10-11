pub (crate) mod config;
pub (crate) mod util;


#[derive(Debug, thiserror::Error)]
pub(crate) enum ExecutionError {
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(&'static str),
}