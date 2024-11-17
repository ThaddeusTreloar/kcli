use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::profile::ProfileError,
};

#[derive(Debug, Args)]
pub(super) struct DescribeProfile {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    profile: Option<String>,
}

impl Invoke for DescribeProfile {
    type E = ProfileError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ProfileError> {
        let Self { profile } = self;

        match profile {
            Some(name) => {
                let profile = ctx
                    .profiles
                    .profile(&name)
                    .ok_or(ProfileError::NotExists(name))?;

                println!(
                    "{}",
                    global_args
                        .out
                        .output_string(&profile)
                        .expect("Failed to write output.")
                );
            }
            None => {
                let profiles = &ctx.profiles.profile_configs;

                println!(
                    "{}",
                    global_args
                        .out
                        .output_string(&profiles)
                        .expect("Failed to write output.")
                );
            }
        }

        Ok(())
    }
}
