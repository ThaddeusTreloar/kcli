
#[derive(Debug, thiserror::Error)]
pub enum OutputError {
    #[error("Failed to serialise object.")]
    Serialise,
}

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Failed to deserialise object.")]
    Deserialise,
}

#[derive(Debug, thiserror::Error)]
pub enum SerdeError {
    #[error("Failed to serialise object.")]
    Serialise,
    #[error("Failed to deserialise object.")]
    Deserialise,
}