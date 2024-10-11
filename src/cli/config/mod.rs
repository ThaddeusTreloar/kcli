use clap::{command, Args, Subcommand};
use cluster::ClusterCommand;
use profile::ProfileCommand;
use registry::RegistryCommand;

use crate::error::cli::ExecutionError;

mod cluster;
mod profile;
mod registry;

#[derive(Args, Debug)]
pub(super) struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubCommand,
}

#[derive(Subcommand, Debug)]
enum ConfigSubCommand {
    #[command(about = "Manager kcli cluster configurations")]
    Cluster(ClusterCommand),
    #[command(about = "Manager kcli schema registry configurations")]
    Registry(RegistryCommand),
    #[command(about = "Manager kcli profile configurations")]
    Profile(ProfileCommand),
}

impl ConfigCommand {
    pub(super) fn execute(self) -> error_stack::Result<(), ExecutionError> {
        match self.command {
            ConfigSubCommand::Cluster(command) => command.execute(),
            ConfigSubCommand::Registry(_) => todo!("Profile"),
            ConfigSubCommand::Profile(_) => todo!("Profile"),
        }
    }
}