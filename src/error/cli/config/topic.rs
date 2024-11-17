#[derive(Debug, thiserror::Error)]
pub enum WriteableTopicError {
    #[error("Topic does not exist: {0}")]
    NotExists(String),
    #[error("Failed to write config.")]
    WriteError,
    #[error("Failed to get input for args: {0}")]
    InputError(&'static str),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadOnlyTopicError {
    #[error("Cluster does not exist: {0}")]
    ClusterNotExists(String),
    #[error("Failed to get default cluster or cluster from user select.")]
    FetchDefaultOrSelect,
    #[error("Error while creating admin client.")]
    CreateAdminClient,
    #[error("Error while calling admin client.")]
    AdminClient,
    #[error("No topics match search parameters, {0}")]
    NotExists(String),
    #[error("Error while writing output.")]
    Output,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Failed to compile regex: {0}")]
    CompileRegex(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TopicError {
    #[error("Topic already exists: {0}")]
    AlreadyExists(String),
    #[error("Topic does not exist: {0}")]
    NotExists(String),
    #[error("Profile does not exist: {0}")]
    ProfileNotExists(String),
    #[error("Default profile not set.")]
    NotSet,
}
