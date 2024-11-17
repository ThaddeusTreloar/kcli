use clap::Args;
use error_stack::ResultExt;

use crate::{
    cli::{
        util::{get_user_choice, get_user_input_confirmation},
        GlobalArgs, Invoke,
    },
    config::{
        profiles::{group::GroupSetting, reset::ResetStrategy, ProfileConfig},
        Context,
    },
    error::cli::config::profile::ProfileError,
    io::output::Output,
};

const OVERWRITE_PROMPT: &str = "Overwrite existing profile? ";

#[derive(Debug, Args)]
pub(super) struct AddProfile {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    profile: String,
    #[arg(short, long, help = "Reset strategy for this profile.")]
    reset: Option<ResetStrategy>,
    #[arg(short, long, help = "Consumer group for this profile.")]
    group: Option<String>,
}

impl Invoke for AddProfile {
    type E = ProfileError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), ProfileError> {
        let Self {
            profile,
            reset,
            group,
        } = self;

        if ctx.profiles.contains_profile(&profile)
            && !get_user_input_confirmation(OVERWRITE_PROMPT)
                .change_context(ProfileError::UserInput("Confirm overwrite"))?
        {
            return Ok(());
        }

        let new_profile = ProfileConfig::default()
            .with_maybe_reset(reset)
            .with_maybe_group(group);

        println!(
            "{}",
            global_args
                .out
                .output_string(&new_profile)
                .expect("Failed to write output.")
        );

        ctx.profiles.add_profile(&profile, new_profile);

        Ok(())
    }
}
