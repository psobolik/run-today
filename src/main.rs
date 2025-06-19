/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-07
 */

mod app_command;
mod data;
mod macros;
mod options;

use crate::options::{Commands, Options};
use app_command::{
    add, last_run, list, remove, run, info,
};
use clap::Parser;
use std::process::ExitCode;

pub extern crate colored;

fn main() -> ExitCode {
    let mut exit_code: u8 = 0;

    // This won't return if the options are invalid or help was requested
    let options = Options::parse();
    match options.command {
        Commands::Run {
            force,
            no_stdout,
            no_stderr,
            quiet,
        } => exit_code = run(force, no_stdout, no_stderr, quiet),
        Commands::List => list(),
        Commands::LastRun => last_run(),
        Commands::Add { program } => {
            exit_code = add(program);
            list();
        }
        Commands::Remove { id } => {
            exit_code = remove(id);
            list();
        }
        Commands::Info => info(),
    }
    ExitCode::from(exit_code)
}
