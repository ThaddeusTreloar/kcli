use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::TopicError,
};

#[derive(Debug, Args)]
pub(super) struct ImportTopic {}

impl Invoke for ImportTopic {
    type E = TopicError;

    fn invoke(self, _: &mut Context, _: &GlobalArgs) -> error_stack::Result<(), TopicError> {
        todo!("IMPORT TOPICS");
    }
}
