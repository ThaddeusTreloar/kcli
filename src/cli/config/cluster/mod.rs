use add::AddCluster;
use alter::AlterCluster;
use clap::{Args, Subcommand};
use default::DefaultCluster;
use describe::DescribeCluster;
use error_stack::ResultExt;
use list::ListCluster;
use remove::RemoveCluster;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::ExecutionError,
};

mod add;
mod alter;
mod default;
mod describe;
mod list;
mod remove;
mod util;

#[derive(Args, Debug)]
pub(super) struct ClusterCommand {
    #[command(subcommand)]
    command: ClusterSubCommand,
}

#[derive(Debug, Subcommand)]
enum ClusterSubCommand {
    #[command(about = "Create a kcli cluster configurations")]
    Add(AddCluster),
    #[command(about = "Delete a kcli cluster configuration")]
    Remove(RemoveCluster),
    #[command(about = "Alter properties for a cluster configuration")]
    Alter(AlterCluster),
    #[command(about = "List cluster configurations")]
    Describe(DescribeCluster),
    #[command(about = "List cluster configurations")]
    List(ListCluster),
    #[command(about = "List cluster configurations")]
    Default(DefaultCluster),
}

impl Invoke for ClusterCommand {
    type E = ExecutionError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            ClusterSubCommand::Add(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster create")),
            ClusterSubCommand::Remove(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster remove")),
            ClusterSubCommand::Alter(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster set")),
            ClusterSubCommand::Describe(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster describe")),
            ClusterSubCommand::List(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster list")),
            ClusterSubCommand::Default(command) => command
                .invoke(ctx, global_args)
                .change_context(ExecutionError::ExecutionFailed("config cluster list")),
        }
    }
}
