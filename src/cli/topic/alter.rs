use clap::Args;

use crate::{
    cli::{GlobalArgs, Invoke},
    config::Context,
    error::cli::config::topic::WriteableTopicError,
};

#[derive(Debug, Args)]
pub(super) struct AlterTopic {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
}

impl Invoke for AlterTopic {
    type E = WriteableTopicError;

    fn invoke(
        self,
        ctx: &mut Context,
        global_args: &GlobalArgs,
    ) -> error_stack::Result<(), WriteableTopicError> {
        Ok(())
    }
}
