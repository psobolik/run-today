/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */
use super::consts;
use crate::data::paths::Paths;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Clone)]
pub struct LastRun;

impl LastRun {
    pub fn load() -> Option<DateTime<Local>> {
        // On load, no folder or no file is okay; we take it to mean the program has never run.
        // We panic if the file contains unparseable data, though.
        if let Some(data_path) = Paths::data_path() {
            if let Ok(mut file) = File::open(data_path.join(consts::LAST_RUN_FILE)) {
                let mut json = String::new();
                let _ = file.read_to_string(&mut json);
                match serde_json::from_str(&json) {
                    Ok(last_run) => last_run,
                    Err(error) => panic!("Failed to parse last run: {}", error),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn store(last_run: &Option<DateTime<Local>>) -> Result<(), Error> {
        if let Some(data_path) = Paths::data_path() {
            std::fs::create_dir_all(&data_path)?;
            let mut file = File::create(data_path.join(consts::LAST_RUN_FILE))?;
            let json = serde_json::to_string(last_run)?;
            write!(file, "{}", json)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Error getting configuration file path",
            ))
        }
    }
    pub fn display() -> String {
        match LastRun::load() {
            None => "Never".to_string(),
            Some(last_run) => last_run.format("%F %R").to_string(),
        }
    }
}
