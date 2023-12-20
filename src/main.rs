/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-07
 */

mod app_command;
mod config;
mod options;

use crate::options::{Commands, Options};
use app_command::{add::add, last_run::last_run, list::list, remove::remove, run::run};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut exit_code: u8 = 0;

    // This won't return if the options are invalid or help was requested
    let options = Options::parse();
    match options.command {
        Commands::Run {
            no_stdout,
            no_stderr,
            verbose,
        } => exit_code = run(no_stdout, no_stderr, verbose),
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
    }
    ExitCode::from(exit_code)
}

pub extern crate colored;

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        println!("{} {message}", $crate::colored::Colorize::bright_green("->"))
    }};
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        eprintln!("{} {message}", $crate::colored::Colorize::bright_red("->"))
    }};
}
