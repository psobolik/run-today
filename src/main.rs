/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-07
 */

mod app_command;
mod config;
mod options;

use crate::options::{Commands, Options};
use chrono::{DateTime, Local};
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
        } => exit_code = app_command::run::do_it(no_stdout, no_stderr, verbose),
        Commands::List => app_command::list::do_it(),
        Commands::LastRun => app_command::last_run::do_it(),
        Commands::Add { program } => {
            exit_code = app_command::add::do_it(program);
            app_command::list::do_it();
        }
        Commands::Remove { id } => {
            exit_code = app_command::remove::do_it(id);
            app_command::list::do_it();
        }
    }
    ExitCode::from(exit_code)
}

fn format_last_run(last_run: Option<DateTime<Local>>) -> String {
    if let Some(lr) = last_run {
        lr.format("%F %R").to_string()
    } else {
        String::from("Never")
    }
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        println!("ℹ️ {message}")
    }};
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        eprintln!("❌ {message}")
    }};
}
