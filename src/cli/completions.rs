use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::error::cli::ExecutionError;

use super::Cli;

#[derive(Args, Debug)]
pub(super) struct CompletionsCommand {
    #[arg(
        index = 1,
        default_value = "zsh",
        help = "Target shell for completions"
    )]
    shell: Shell,
}

impl CompletionsCommand {
    pub(super) fn execute(self) -> error_stack::Result<(), ExecutionError> {
        generate(
            self.shell,
            &mut Cli::command(),
            "kcli",
            &mut std::io::stdout(),
        );

        Ok(())
    }
}
