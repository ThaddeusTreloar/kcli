#[derive(Debug, thiserror::Error)]
pub enum ConfigIoError {
    #[error("Failed to read config from file: {0}")]
    Read(String),
    #[error("Failed to write config to file: {0}")]
    Write(String),
    #[error("Failed to parse config file.")]
    Parse,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchClusterError {
    #[error("Failed to get user input.")]
    Input,
    #[error("Cluster does not exist: {0}")]
    NotExists(String),
    #[error("No clusters in config.")]
    NoClusters,
}
