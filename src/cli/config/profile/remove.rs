use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::profile::ProfileError,
};

#[derive(Debug, Args)]
pub(super) struct RemoveProfile {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    profile: String,
}

impl Invoke for RemoveProfile {
    type E = ProfileError;

    fn invoke(self, ctx: &mut Context, _: &GlobalArgs) -> error_stack::Result<(), ProfileError> {
        let Self { profile } = self;

        ctx.profiles.profile_configs.remove(&profile);

        Ok(())
    }
}
