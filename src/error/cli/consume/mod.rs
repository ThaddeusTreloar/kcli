#[derive(Debug, thiserror::Error)]
pub enum ConsumerError {
    #[error("Default cluster doesn't exist, default unset.")]
    DefaultNotExists,
    #[error("Cluster does not exist: {0}")]
    ClusterNotExists(String),
    #[error("Failed to get input for args: {0}")]
    InputError(&'static str),
    #[error("Failed to create consumer.")]
    CreateConsumer,
    #[error("Consumer failed while reading.")]
    ConsumerFailure,
    #[error("Failed to deserialise message key.")]
    KeyDeserialisationFailure,
    #[error("Failed to deserialise message key.")]
    ValueDeserialisationFailure,
}