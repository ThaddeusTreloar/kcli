use std::{any::type_name, error::Error, net::{SocketAddr, ToSocketAddrs}, str::FromStr};
use error_stack::ResultExt;

use inquire::{Confirm, Select, Text};

use crate::error::cli::util::UserInputError;

pub (super) fn get_user_input_vec<T>(prompt: &str) -> error_stack::Result<Vec<T>, UserInputError> 
where 
    T: FromStr,
    T::Err: Error + Send + Sync + 'static
{
    let mut buff = Vec::new();

    while let Some(user_response) = {
        let r = Text::new(prompt)
            .prompt()
            .change_context(UserInputError::FailedToGetUserInput)?;

        if r.is_empty() {
            None
        } else {
            Some(r)
        }
    } {
        let parsed_input: T = T::from_str(&user_response)
            .change_context(
                UserInputError::FailedToGetParseInput{
                    input: user_response.to_owned(),
                    t: type_name::<T>().to_owned(),
                }
            )?;

        buff.push(parsed_input);
    }

    Ok(buff)
}

pub (super) fn get_user_input_choice(prompt: &str) -> error_stack::Result<bool, UserInputError> 
{
    Confirm::new(prompt)
        .prompt()
        .change_context(UserInputError::FailedToGetUserInput)
}