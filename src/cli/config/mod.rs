use clap::{command, Args, Subcommand};
use cluster::ClusterCommand;
use profile::ProfileCommand;
use topic::TopicCommand;

use crate::{config::Context, error::cli::ExecutionError};

use super::{GlobalArgs, Invoke};

mod cluster;
mod profile;
mod topic;

#[derive(Args, Debug)]
pub(super) struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubCommand,
}

#[derive(Subcommand, Debug)]
enum ConfigSubCommand {
    #[command(about = "Manage kcli cluster configurations")]
    Cluster(ClusterCommand),
    #[command(about = "Manage kcli profile configurations")]
    Profile(ProfileCommand),
    #[command(about = "Manage kcli topic configurations")]
    Topic(TopicCommand),
}

impl Invoke for ConfigCommand {
    type E = ExecutionError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            ConfigSubCommand::Cluster(command) => command.invoke(ctx, global_args),
            ConfigSubCommand::Profile(command) => command.invoke(ctx, global_args),
            ConfigSubCommand::Topic(command) => command.invoke(ctx, global_args),
        }
    }
}
