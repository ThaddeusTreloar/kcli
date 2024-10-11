

#[derive(Debug, thiserror::Error)]
pub (crate) enum CreateClusterError {
    #[error("A cluster with the name: {0}, already exists.")]
    AlreadyExists(String),
    #[error("Failed to provide bootstrap servers.")]
    MissingBootstrapServers,
    #[error("Failed to write config.")]
    WriteError,
    #[error("Failed to get input for args: {0}")]
    InputError(&'static str),
    #[error("Failed to parse: {input}, as type: {t}")]
    ParseError{
        input: String,
        t: String,
    },
}