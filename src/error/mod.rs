use std::{error::Error, process::exit};

use error_stack::Report;

pub mod cli;
pub mod config;
pub mod io;

pub fn handle_expect_report<E>(e: &Report<E>) 
where E: Error
{
    log::error!("Fatal error: {:?}", e);
    exit(1)
}
