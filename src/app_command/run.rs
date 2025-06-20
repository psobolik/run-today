/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::colored::Colorize;
use crate::data::{last_run::LastRun, program::Program, programs::Programs};
use crate::{print_error, print_info};
use chrono::{DateTime, Datelike, Local};

pub fn command(force: bool, no_stdout: bool, no_stderr: bool, quiet: bool) -> u8 {
    // Returns 0 if no errors
    // 1 if the last run can't be stored
    // 2 + the number of programs that failed, if any failed
    let mut exit_code: u8 = 0;

    if !quiet {
        print_info!("Run Today");
        print_info!("Last run: {}", LastRun::display());
    }
    if force || should_run(&LastRun::load()) {
        exit_code = run_programs(&Programs::load(), no_stdout, no_stderr, quiet);
        exit_code += match LastRun::store(&Some(Local::now())) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    } else if !quiet {
        print_info!("Doing nothing (already run today)");
    }
    exit_code
}

fn should_run(option: &Option<DateTime<Local>>) -> bool {
    // Assumes last day option isn't in the future
    if let Some(last_run) = option {
        let now = Local::now();
        now.year() != last_run.year()
            || now.month() != last_run.month()
            || now.day() != last_run.day()
    } else {
        true // Should run if there's no last run date
    }
}

fn run_programs(programs: &Vec<Program>, no_stdout: bool, no_stderr: bool, quiet: bool) -> u8 {
    fn run_program(program: &Program, no_stdout: bool, no_stderr: bool, quiet: bool) -> u8 {
        let mut exit_code = 0;

        if !quiet {
            print_info!("Running {}", program.to_string().italic());
        }

        let mut process_command = std::process::Command::new(program.name());
        if let Some(args) = program.args() {
            for arg in args {
                process_command.arg(arg);
            }
        }

        match process_command.output() {
            Ok(output) => {
                if !no_stdout {
                    if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
                        print!("{stdout}")
                    }
                }
                if !no_stderr {
                    if let Ok(stderr) = std::str::from_utf8(&output.stderr) {
                        eprint!("{stderr}")
                    }
                }
            }
            Err(error) => {
                if !quiet {
                    print_error!(
                        "Error running program {}: {error}",
                        program.to_string().italic()
                    );
                }
                exit_code = 1;
            }
        }
        exit_code
    }

    let mut errors = 0;
    for program in programs {
        errors += run_program(program, no_stdout, no_stderr, quiet);
    }
    if errors > 0 {
        errors + 2
    } else {
        errors
    }
}
