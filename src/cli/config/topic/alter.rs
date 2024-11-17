use clap::Args;
use error_stack::ResultExt;
use log::info;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::TopicError,
    io::{output::Output, serde::Serde},
};

#[derive(Debug, Args)]
pub(super) struct AlterTopic {
    #[arg(index = 1, help = "Name for the topic.")]
    topic: String,
    #[arg(long, short, help = "Default key serialiser for this topic.")]
    key_serde: Option<Serde>,
    #[arg(long, short, help = "Default value serialiser for this topic.")]
    value_serde: Option<Serde>,
    #[arg(
        long,
        short,
        conflicts_with = "remove_profile",
        help = "Default profile for this topic."
    )]
    profile: Option<String>,
    #[arg(
        long,
        short,
        conflicts_with = "profile",
        help = "Default profile for this topic."
    )]
    remove_profile: bool,
}

impl Invoke for AlterTopic {
    type E = TopicError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), TopicError> {
        let Self {
            topic,
            key_serde,
            value_serde,
            profile,
            remove_profile,
        } = self;

        let existing_topic = ctx
            .topics
            .topic_mut(&topic)
            .ok_or(TopicError::NotExists(topic.clone()))?;

        if let Some(profile) = profile {
            ctx.profiles
                .profile(&profile)
                .ok_or(TopicError::ProfileNotExists(profile.clone()))?;

            let old = existing_topic.default_profile.replace(profile.clone());

            info!(
                "Replacing profile: {} -> {}",
                old.unwrap_or("None".to_owned()),
                profile
            );
        }

        if remove_profile {
            info!(
                "Removing profile: {} -> None",
                existing_topic
                    .default_profile()
                    .unwrap_or(&"None".to_owned()),
            );

            existing_topic.default_profile.take();
        }

        if let Some(serde) = key_serde {
            let old = existing_topic.key_serde;

            info!("Replacing key serde: {} -> {}", old, serde);

            existing_topic.key_serde = serde;
        }

        if let Some(serde) = value_serde {
            let old = existing_topic.value_serde;

            info!("Replacing key serde: {} -> {}", old, serde);

            existing_topic.value_serde = serde;
        }

        println!(
            "{}",
            global_args
                .out
                .output_string(&existing_topic)
                .expect("Failed to Write.")
        );

        Ok(())
    }
}
