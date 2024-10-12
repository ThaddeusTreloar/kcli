use clap::Args;

use crate::error::cli::ExecutionError;

#[derive(Args, Debug)]
pub(super) struct GroupCommand {}

impl GroupCommand {
    pub(super) fn execute(&self) -> error_stack::Result<(), ExecutionError> {
        todo!("GroupCommand")
    }
}
