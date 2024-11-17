use alter::AlterTopic;
use clap::{Args, Subcommand};
use create::CreateTopic;
use delete::DeleteTopic;
use describe::DescribeTopic;
use error_stack::ResultExt;
use list::ListTopics;

use crate::{config::Context, error::cli::ExecutionError};

use super::{GlobalArgs, Invoke};

mod alter;
mod create;
mod delete;
mod describe;
mod list;

const INTERNAL_TOPIC_REGEX: &str =
    r"^__consumer_offsets$|^__transaction_state$|^__share_group_state$|^__cluster_metadata$";

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

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            TopicSubCommand::Alter(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("topic alter")),
            TopicSubCommand::Create(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("topic create")),
            TopicSubCommand::Delete(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("topic delete")),
            TopicSubCommand::Describe(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("topic describe")),
            TopicSubCommand::List(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("topic list")),
        }
    }
}
