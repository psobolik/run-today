/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-08
 */

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about("Runs a list of commands no more than once a day."))]
pub struct Options {
    #[command(subcommand)]
    pub(crate) command: Commands,
}
#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Run each command in the list if this program hasn't been run today.
    Run {
        #[arg(
            long,
            help = "Run commands without regard for when this program was last run."
        )]
        force: bool,

        #[arg(long, help = "Don't print program output.")]
        no_stdout: bool,

        #[arg(long, help = "Don't print program errors.")]
        no_stderr: bool,

        #[arg(short, long, help = "Don't print additional information.")]
        quiet: bool,
    },
    /// Print the last time this program was run.
    LastRun,
    /// Print the list of commands, with their IDs.
    List,
    /// Add a command to the list.
    #[command(arg_required_else_help = true)]
    Add {
        /// The command to add, including parameters.
        ///
        /// Include the command argument in double quotes, so it is passed to the program as
        /// a single parameter.
        ///
        /// If the command contains a space, or is in a folder with spaces, quote it with single quotes.
        ///
        /// For example:
        ///
        /// run-today add "'C:\Program Files\Something\something.exe' -o foo --bar=bum"
        #[arg(required = true)]
        command: String,
    },
    /// Remove a command from the list, by ID.
    #[command(alias("delete"), arg_required_else_help = true)]
    Remove {
        /// The ID of the command to remove.
        ///
        /// Run `run-today list` to see the commands' and their IDs.
        #[arg(required = true)]
        id: usize,
    },
    /// Print information about the app.
    #[command()]
    Info,
}
