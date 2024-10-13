use alter::AlterTopic;
use clap::{Args, Subcommand};
use create::CreateTopic;
use delete::DeleteTopic;
use describe::DescribeTopic;
use error_stack::ResultExt;
use list::ListTopics;

use crate::{config::Context, error::cli::ExecutionError};

use super::Invoke;

mod alter;
mod create;
mod delete;
mod describe;
mod list;

#[derive(Args, Debug)]
pub(super) struct TopicCommand {
    #[command(subcommand)]
    command: TopicSubCommand,
}

#[derive(Subcommand, Debug)]
enum TopicSubCommand {
    #[command(about = "Alter a kafka topic configuration.")]
    Alter(AlterTopic),
    #[command(about = "Create a kafka topic.")]
    Create(CreateTopic),
    #[command(about = "Delete a kafka topic.")]
    Delete(DeleteTopic),
    #[command(about = "Desribe a kafka topic.")]
    Describe(DescribeTopic),
    #[command(about = "List available topics on cluster")]
    List(ListTopics),
}

impl Invoke for TopicCommand {
    type E = ExecutionError;

    fn invoke(self, ctx: &mut Context) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            TopicSubCommand::Alter(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("topic alter")),
            TopicSubCommand::Create(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("topic create")),
            TopicSubCommand::Delete(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("topic delete")),
            TopicSubCommand::Describe(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("topic describe")),
            TopicSubCommand::List(command) => command
                .invoke(ctx)
                .change_context(ExecutionError::ExecutionFailed("topic list")),
        }
    }
}
