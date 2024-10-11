use clap::Args;

use crate::error::cli::ExecutionError;

#[derive(Args, Debug)]
pub(super) struct ProducerCommand {
}

impl ProducerCommand {
    pub(super) fn execute(&self) -> error_stack::Result<(), ExecutionError> {
        todo!("ProducerCommand")
    }
}