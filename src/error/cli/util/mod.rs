


#[derive(Debug, thiserror::Error)]
pub (crate) enum UserInputError {
    #[error("Failed to get user input.")]
    FailedToGetUserInput,
    #[error("Failed to parse user input: {input}, as type: {t}")]
    FailedToGetParseInput{
        input: String,
        t: String,
    },
}