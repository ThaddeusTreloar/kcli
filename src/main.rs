use clap::Parser;
use cli::Cli;
use config::init_config;
use error::handle_expect_report;
use util::init_logging;

mod cli;
mod config;
mod error;
mod util;

fn main() {
    init_logging();

    let _ = init_config().inspect_err(handle_expect_report);

    let _ = Cli::parse().execute().inspect_err(handle_expect_report);
}
