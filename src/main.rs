/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-07
 */

mod app_command;
mod data;
mod options;
mod macros;

use crate::options::{Commands, Options};
use app_command::{do_add_command, do_last_run_command, do_list_command, do_remove_command, do_run_command};
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
        } => exit_code = do_run_command(force, no_stdout, no_stderr, quiet),
        Commands::List => do_list_command(),
        Commands::LastRun => do_last_run_command(),
        Commands::Add { program } => {
            exit_code = do_add_command(program);
            do_list_command();
        }
        Commands::Remove { id } => {
            exit_code = do_remove_command(id);
            do_list_command();
        }
    }
    ExitCode::from(exit_code)
}
