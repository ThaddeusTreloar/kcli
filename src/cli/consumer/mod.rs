use clap::Args;

use crate::error::cli::ExecutionError;

#[derive(Args, Debug)]
pub(super) struct ConsumerCommand {
}

impl ConsumerCommand {
    pub(super) fn execute(&self) -> error_stack::Result<(), ExecutionError> {
        todo!("ConsumerCommand")
    }
}