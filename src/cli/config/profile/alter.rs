use clap::Args;
use error_stack::ResultExt;
use log::info;

use crate::{
    cli::{
        util::{get_user_choice, get_user_input_confirmation},
        GlobalArgs, Invoke,
    },
    config::{
        profiles::{
            group::GroupSetting,
            reset::{self, ResetStrategy},
            ProfileConfig,
        },
        Context,
    },
    error::cli::config::profile::ProfileError,
    io::output::Output,
};

const OVERWRITE_PROMPT: &str = "Overwrite existing profile? ";

#[derive(Debug, Args)]
pub(super) struct AlterProfile {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    profile: String,
    #[arg(short, long, help = "Reset strategy for this profile.")]
    reset: Option<ResetStrategy>,
    #[arg(
        short,
        long,
        conflicts_with = "remove_group",
        help = "Consumer group for this profile."
    )]
    group: Option<String>,
    #[arg(
        long,
        conflicts_with = "group",
        help = "Consumer group for this profile."
    )]
    remove_group: bool,
}

impl Invoke for AlterProfile {
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
            remove_group,
        } = self;

        let existing_profile = ctx
            .profiles
            .profile_mut(&profile)
            .ok_or(ProfileError::NotExists(profile.clone()))?;

        if let Some(reset) = reset {
            info!(
                "Replacing reset strategy: {} -> {}",
                existing_profile.reset, reset
            );

            existing_profile.reset = reset;
        }

        if let Some(group) = group {
            info!(
                "Replacing consumer group: {} -> {}",
                existing_profile.group, group
            );

            existing_profile.group = GroupSetting::Group(group);
        }

        if remove_group {
            info!(
                "Removing consumer group: {} -> {}",
                existing_profile.group, "Never"
            );

            existing_profile.group = GroupSetting::Never;
        }

        println!(
            "{}",
            global_args
                .out
                .output_string(&existing_profile)
                .expect("Failed to write output.")
        );

        Ok(())
    }
}
