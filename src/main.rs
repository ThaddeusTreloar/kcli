use clap::Parser;
use cli::Cli;

mod cli;
mod config;
mod error;
mod io;
mod util;

fn main() {
    Cli::parse().invoke_root();
}
