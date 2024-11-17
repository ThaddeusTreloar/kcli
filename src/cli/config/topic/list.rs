use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::TopicError,
};

#[derive(Debug, Args)]
pub(super) struct ListTopic {}

impl Invoke for ListTopic {
    type E = TopicError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), TopicError> {
        let Self {} = self;

        let topics = ctx.topics.topic_configs.keys().collect::<Vec<_>>();

        println!(
            "{}",
            global_args
                .out
                .output_string(&topics)
                .expect("Failed to write output.")
        );

        Ok(())
    }
}
