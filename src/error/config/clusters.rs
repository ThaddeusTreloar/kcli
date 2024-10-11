

#[derive(Debug, thiserror::Error)]
pub (crate) enum ClusterConfigIoError {
    #[error("Failed to resolve config file location from path: {0}")]
    ResolutionError(String),
    #[error("Failed to read config from file: {0}")]
    ReadError(String),
    #[error("Failed to write config to file: {0}")]
    WriteError(String),
    #[error("Failed to parse config file.")]
    ParseError,
}