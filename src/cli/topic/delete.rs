use clap::Args;

use crate::{cli::Invoke, config::Context, error::cli::config::topic::WriteableTopicError};

#[derive(Debug, Args)]
pub(super) struct DeleteTopic {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
}

impl Invoke for DeleteTopic {
    type E = WriteableTopicError;

    fn invoke(self, ctx: &mut Context) -> error_stack::Result<(), WriteableTopicError> {
        Ok(())
    }
}
