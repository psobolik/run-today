/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-07
 */

mod app_command;
mod config;
mod options;
mod macros;

use crate::options::{Commands, Options};
use app_command::{add, last_run, list, remove, run};
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
            verbose,
        } => exit_code = run::run(force, no_stdout, no_stderr, verbose),
        Commands::List => list::list(),
        Commands::LastRun => last_run::last_run(),
        Commands::Add { program } => {
            exit_code = add::add(program);
            list::list();
        }
        Commands::Remove { id } => {
            exit_code = remove::remove(id);
            list::list();
        }
    }
    ExitCode::from(exit_code)
}
