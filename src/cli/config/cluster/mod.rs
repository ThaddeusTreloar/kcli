use clap::{Args, Subcommand};
use create::CreateCluster;
use error_stack::ResultExt;

use crate::error::cli::ExecutionError;

mod create;
mod delete;

#[derive(Args, Debug)]
pub(super) struct ClusterCommand {
    #[command(subcommand)]
    command: ClusterSubCommand
}

#[derive(Debug, Subcommand)]
enum ClusterSubCommand {
    #[command(about = "Create a kcli cluster configurations")]
    Create(CreateCluster),
    #[command(about = "Delete a kcli cluster configuration")]
    Delete{
        #[arg(short, long, default_value = "")]
        cluster: String,
    },
}

impl ClusterCommand {
    pub(super) fn execute(self) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            ClusterSubCommand::Create(command) => command.execute()
                .change_context(ExecutionError::ExecutionFailed("config cluster create")),

            ClusterSubCommand::Delete { cluster } => todo!("Cluster delete")
        }
    }
}