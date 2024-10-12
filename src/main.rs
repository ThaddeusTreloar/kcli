use std::process::exit;

use clap::Parser;
use cli::{Cli, Invoke};
use config::Context;
use error::handle_expect_report;
use util::init_logging;

mod cli;
mod config;
mod error;
mod io;
mod util;

fn main() {
    init_logging();

    let ctx = match Context::init() {
        Ok(ctx) => ctx,
        Err(e) => {
            handle_expect_report(&e);
            exit(1);
        },
    };

    let _ = Cli::parse().invoke(ctx).inspect_err(handle_expect_report);
}
