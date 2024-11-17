#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("Profile does not exist: {0}")]
    NotExists(String),
    #[error("Failed to get user input while: {0}")]
    UserInput(&'static str),
}
