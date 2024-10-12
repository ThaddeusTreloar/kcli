

#[derive(Debug, thiserror::Error)]
pub enum ClusterConfigIoError {
    #[error("Failed to read config from file: {0}")]
    Read(String),
    #[error("Failed to write config to file: {0}")]
    Write(String),
    #[error("Failed to parse config file.")]
    Parse,
}