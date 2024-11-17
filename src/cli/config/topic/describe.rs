use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::TopicError,
};

#[derive(Debug, Args)]
pub(super) struct DescribeTopic {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    topic: Option<String>,
}

impl Invoke for DescribeTopic {
    type E = TopicError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), TopicError> {
        let Self { topic } = self;

        match topic {
            Some(name) => {
                let topic = ctx.topics.topic(&name).ok_or(TopicError::NotExists(name))?;

                println!(
                    "{}",
                    global_args
                        .out
                        .output_string(&topic)
                        .expect("Failed to write output.")
                );
            }
            None => {
                let topics = &ctx.topics.topic_configs;

                println!(
                    "{}",
                    global_args
                        .out
                        .output_string(&topics)
                        .expect("Failed to write output.")
                );
            }
        }

        Ok(())
    }
}
