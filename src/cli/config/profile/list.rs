use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::profile::ProfileError,
};

#[derive(Debug, Args)]
pub(super) struct ListProfile {}

impl Invoke for ListProfile {
    type E = ProfileError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ProfileError> {
        let Self {} = self;

        let profiles = ctx.profiles.profile_configs.keys().collect::<Vec<_>>();

        println!(
            "{}",
            global_args
                .out
                .output_string(&profiles)
                .expect("Failed to write output.")
        );

        Ok(())
    }
}
