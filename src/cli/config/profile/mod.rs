use add::AddProfile;
use alter::AlterProfile;
use clap::{Args, Subcommand};
use describe::DescribeProfile;
use error_stack::ResultExt;
use import::ImportProfile;
use list::ListProfile;
use remove::RemoveProfile;

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
pub(super) struct ProfileCommand {
    #[command(subcommand)]
    command: ProfileSubCommand,
}

#[derive(Debug, Subcommand)]
enum ProfileSubCommand {
    #[command(about = "Add a profile configuration")]
    Add(AddProfile),
    #[command(about = "Alter configuration for a profile")]
    Alter(AlterProfile),
    #[command(about = "Describe profile configurations")]
    Describe(DescribeProfile),
    #[command(about = "Import a profile configuration")]
    Import(ImportProfile),
    #[command(about = "List configured profiles")]
    List(ListProfile),
    #[command(about = "Remove a profile configuration")]
    Remove(RemoveProfile),
}

impl Invoke for ProfileCommand {
    type E = ExecutionError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            ProfileSubCommand::Add(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic add")),
            ProfileSubCommand::Alter(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic alter")),
            ProfileSubCommand::Describe(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic describe")),
            ProfileSubCommand::Import(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic import")),
            ProfileSubCommand::List(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic list")),
            ProfileSubCommand::Remove(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config topic remove")),
        }
    }
}
