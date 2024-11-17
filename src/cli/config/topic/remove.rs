use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::TopicError,
};

#[derive(Debug, Args)]
pub(super) struct RemoveTopic {
    #[arg(index = 1, help = "Logical name for the topic.")]
    topic: String,
}

impl Invoke for RemoveTopic {
    type E = TopicError;

    fn invoke(self, ctx: &mut Context, _: &GlobalArgs) -> error_stack::Result<(), TopicError> {
        let Self { topic } = self;

        ctx.topics.topic_configs.remove(&topic);

        Ok(())
    }
}
