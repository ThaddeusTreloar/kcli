use clap::Args;
use error_stack::ResultExt;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::{topics::TopicConfig, Context},
    error::cli::config::topic::TopicError,
    io::{output::Output, serde::Serde},
};

#[derive(Debug, Args)]
pub(super) struct AddTopic {
    #[arg(index = 1, help = "Name for the topic.")]
    topic: String,
    #[arg(
        long,
        short,
        default_value_t,
        help = "Default key serialiser for this topic."
    )]
    key_serde: Serde,
    #[arg(
        long,
        short,
        default_value_t,
        help = "Default value serialiser for this topic."
    )]
    value_serde: Serde,
    #[arg(long, short, help = "Default profile for this topic.")]
    profile: Option<String>,
}

impl Invoke for AddTopic {
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
        } = self;

        if ctx.topics.contains_topic(&topic) {
            Err(TopicError::AlreadyExists(topic.clone()))?
        }

        if let Some(profile) = &profile {
            ctx.profiles
                .profile(profile)
                .ok_or(TopicError::ProfileNotExists(profile.clone()))?;
        }

        let new_topic = TopicConfig {
            default_profile: profile,
            key_serde,
            value_serde,
        };

        println!(
            "{}",
            global_args
                .out
                .output_string(&new_topic)
                .expect("Failed to Write.")
        );

        ctx.topics.add_topic(&topic, new_topic);

        Ok(())
    }
}
