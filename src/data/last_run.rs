/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use super::consts;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use directories::BaseDirs;

fn data_path() -> Option<PathBuf> {
    BaseDirs::new().map(|base_dirs| {
        Path::new(base_dirs.data_local_dir())
            .join(consts::FOLDER_NAME)
    })
}

pub fn format_last_run(last_run: Option<DateTime<Local>>) -> String {
    if let Some(lr) = last_run {
        lr.format("%F %R").to_string()
    } else {
        String::from("Never")
    }
}

pub fn load() -> Option<DateTime<Local>> {
    if let Some(data_path) = data_path() {
        // We only panic if the file contains unparseable data.
        if let Ok(mut file) = File::open(data_path.join(consts::LAST_RUN_FILE)) {
            let mut json = String::new();
            let _ = file.read_to_string(&mut json);
            if let Ok(result) = serde_json::from_str(&json) {
                result
            } else {
                panic!("Error loading last run data");
            }
        } else {
            None
        }
    } else {
        None
    }
}

pub fn store(last_run: &Option<DateTime<Local>>) -> Result<(), Error> {
    if let Some(data_path) = data_path() {
        std::fs::create_dir_all(&data_path)?;
        let mut file = File::create(data_path.join(consts::LAST_RUN_FILE))?;
        if let Ok(json) = serde_json::to_string(&last_run) {
            write!(file, "{}", json)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Error formatting last run date",
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Error getting configuration file path",
        ))
    }
}
