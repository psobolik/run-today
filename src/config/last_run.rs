/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use super::consts;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

pub fn load(path: &Path) -> Option<DateTime<Local>> {
    // We only panic if the file contains unparseable data.
    if let Ok(mut file) = File::open(path.join(consts::LAST_RUN_FILE)) {
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
}

pub fn store(last_run: &Option<DateTime<Local>>, path: &Path) -> Result<(), Error> {
    let mut file = File::create(path.join(consts::LAST_RUN_FILE))?;
    if let Ok(json) = serde_json::to_string(&last_run) {
        write!(file, "{}", json)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Error formatting last run date",
        ))
    }
}
