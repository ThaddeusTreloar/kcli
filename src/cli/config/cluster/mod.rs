use clap::{Args, Subcommand};
use create::CreateCluster;
use error_stack::ResultExt;

use crate::error::cli::ExecutionError;

mod create;

#[derive(Args, Debug)]
pub(super) struct ClusterCommand {
    #[command(subcommand)]
    command: ClusterSubCommand
}

#[derive(Debug, Subcommand)]
enum ClusterSubCommand {
    Create(CreateCluster),
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