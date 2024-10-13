use clap::Args;

use crate::{cli::Invoke, config::Context, error::cli::config::topic::ReadOnlyTopicError};

#[derive(Debug, Args)]
pub(super) struct DescribeTopic {
    #[arg(index = 1, help = "Logical name for the cluster.")]
    name: String,
}

impl Invoke for DescribeTopic {
    type E = ReadOnlyTopicError;

    fn invoke(self, mut ctx: &mut Context) -> error_stack::Result<(), ReadOnlyTopicError> {
        Ok(())
    }
}
