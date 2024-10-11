use clap::{CommandFactory, Parser};
use clap_complete::{generate, generate_to, Shell};
use cli::Cli;
use config::init_config;
use error::handle_expect_report;
use util::init_logging;

mod cli;
mod config;
mod error;
mod io;
mod util;

fn main() {
    init_logging();

    let _ = init_config().inspect_err(handle_expect_report);

    let _ = Cli::parse().execute().inspect_err(handle_expect_report);
}
