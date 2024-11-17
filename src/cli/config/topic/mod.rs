use add::AddTopic;
use alter::AlterTopic;
use clap::{Args, Subcommand};
use describe::DescribeTopic;
use error_stack::ResultExt;
use import::ImportTopic;
use list::ListTopic;
use remove::RemoveTopic;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::ExecutionError,
};

pub mod add;
pub mod alter;
pub mod describe;
pub mod import;
pub mod list;
pub mod remove;

#[derive(Args, Debug)]
pub(super) struct TopicCommand {
    #[command(subcommand)]
    command: TopicSubCommand,
}

#[derive(Debug, Subcommand)]
enum TopicSubCommand {
    #[command(about = "Add a topic configuration")]
    Add(AddTopic),
    #[command(about = "Alter configuration for a topic")]
    Alter(AlterTopic),
    #[command(about = "Describe topic configurations")]
    Describe(DescribeTopic),
    #[command(about = "Import a topic configuration")]
    Import(ImportTopic),
    #[command(about = "List configured topics")]
    List(ListTopic),
    #[command(about = "Remove a topic configuration")]
    Remove(RemoveTopic),
}

impl Invoke for TopicCommand {
    type E = ExecutionError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            TopicSubCommand::Add(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic add")),
            TopicSubCommand::Alter(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic alter")),
            TopicSubCommand::Describe(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic describe")),
            TopicSubCommand::Import(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic import")),
            TopicSubCommand::List(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic list")),
            TopicSubCommand::Remove(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic remove")),
        }
    }
}
