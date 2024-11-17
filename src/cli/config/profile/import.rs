use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::{profiles::reset::ResetStrategy, Context},
    error::cli::config::profile::ProfileError,
};

#[derive(Debug, Args)]
pub(super) struct ImportProfile {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
    #[arg(short, long, help = "Reset strategy for this profile.")]
    reset: Option<ResetStrategy>,
    #[arg(short, long, help = "Consumer group for this profile.")]
    group: Option<String>,
}

impl Invoke for ImportProfile {
    type E = ProfileError;

    fn invoke(self, _: &mut Context, _: &GlobalArgs) -> error_stack::Result<(), ProfileError> {
        todo!("IMPORT PROFILE");
    }
}
