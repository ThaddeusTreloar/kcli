use serde::{Deserialize, Serialize};

use crate::{cli::util::get_user_input, config::FromUserInput, error::cli::util::UserInputError};

const GET_USERNAME_PROMPT: &str = "Enter username:";
const GET_PASSWORD_PROMPT: &str = "Enter password:";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SaslPlain {
    username: String,
    password: String,
}

impl FromUserInput for SaslPlain {
    type E = UserInputError;

    fn from_user_input() -> error_stack::Result<Self, Self::E> where Self: Sized {
        let username = get_user_input(GET_USERNAME_PROMPT)?;
        let password = get_user_input(GET_PASSWORD_PROMPT)?;

        Ok(
            Self { username, password }
        )
    }
}