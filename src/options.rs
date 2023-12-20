/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-08
 */

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    long_about("Runs a list of programs if this program hasn't been run today")
)]
pub struct Options {
    #[command(subcommand)]
    pub(crate) command: Commands,
}
#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Run each program in the list if this program hasn't been run today
    Run {
        #[arg(long, help = "Run programs without regard for when this program was last run")]
        force: bool,

        #[arg(long, help = "Don't print program output")]
        no_stdout: bool,

        #[arg(long, help = "Don't print program errors")]
        no_stderr: bool,

        #[arg(short, long, help = "Display additional information")]
        verbose: bool,
    },
    /// Print the last time this program was run
    LastRun,
    /// Print the list of programs
    List,
    /// Add a program to the list
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(required = true)]
        program: String,
    },
    /// Remove a program from the list, by ID
    #[command(arg_required_else_help = true)]
    Remove {
        #[arg(required = true)]
        id: usize,
    },
}
