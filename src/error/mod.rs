use std::{error::Error, process::exit};

use error_stack::Report;

pub(crate) mod cli;
pub(crate) mod config;

pub (crate) fn handle_expect_report<E>(e: &Report<E>) 
where E: Error
{
    log::error!("Fatal error: {:?}", e);
    exit(1)
}