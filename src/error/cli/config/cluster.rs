#[derive(Debug, thiserror::Error)]
pub enum AddClusterError {
    #[error("A cluster with the name: {0}, already exists.")]
    AlreadyExists(String),
    #[error("Failed to provide bootstrap servers.")]
    MissingBootstrapServers,
    #[error("Failed to write config.")]
    WriteError,
    #[error("Failed to get input for args: {0}")]
    InputError(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum WritableClusterError {
    #[error("Cluster does not exist: {0}")]
    NotExists(String),
    #[error("Failed to write config.")]
    WriteError,
    #[error("Failed to get input for args: {0}")]
    InputError(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadOnlyClusterError {
    #[error("Cluster does not exist: {0}")]
    NotExists(String),
    #[error("Error while writing output.")]
    Output,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateServersError {
    #[error("Validation Failed")]
    ValidationFailed,
}
