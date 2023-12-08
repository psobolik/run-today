/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-08
 */

mod last_run;
pub mod program;
mod programs;

use crate::config::program::Program;
use chrono::{DateTime, Local};
use directories::UserDirs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = ".config";
const CONFIG_APP: &str = "run-today";
const PROGRAMS_FILE: &str = "programs.json";
const LAST_RUN_FILE: &str = "last-run.json";

pub fn load_programs() -> Vec<Program> {
    if let Some(config_path) = config_path() {
        programs::load(&config_path)
    } else {
        vec![]
    }
}
pub fn store_programs(programs: &[Program]) -> Result<(), Error> {
    if let Some(config_path) = config_path() {
        programs::store(programs, &config_path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Error getting configuration file path",
        ))
    }
}

pub fn load_last_run() -> Option<DateTime<Local>> {
    if let Some(config_path) = config_path() {
        last_run::load(&config_path)
    } else {
        None
    }
}
pub fn store_last_run(last_run: &Option<DateTime<Local>>) -> Result<(), Error> {
    if let Some(config_path) = config_path() {
        last_run::store(last_run, &config_path)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Error getting configuration file path",
        ))
    }
}

fn config_path() -> Option<PathBuf> {
    UserDirs::new().map(|user_dirs| {
        Path::new(user_dirs.home_dir())
            .join(CONFIG_PATH)
            .join(CONFIG_APP)
    })
}
