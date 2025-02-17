/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */
use super::consts;
use chrono::{DateTime, Local};
use directories::BaseDirs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct LastRun {
    last_run: Option<DateTime<Local>>,
    data_path: Option<PathBuf>,
}

impl LastRun {
    pub fn last_run(self) -> Option<DateTime<Local>> {
        self.last_run
    }
    pub fn new() -> Self {
        let data_path = BaseDirs::new()
            .map(|base_dirs| Path::new(base_dirs.data_local_dir()).join(consts::FOLDER_NAME));
        let last_run = Self::load(&data_path);
        Self {
            data_path,
            last_run,
        }
    }
    pub fn store(self, last_run: &Option<DateTime<Local>>) -> Result<(), Error> {
        if let Some(data_path) = self.data_path {
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
}
impl LastRun {
    fn load(data_path: &Option<PathBuf>) -> Option<DateTime<Local>> {
        // On load, no folder or not file is okay; we take it to mean the program has never run.
        // We panic if the file contains unparseable data, though.
        if let Some(data_path) = data_path {
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
}

impl std::fmt::Display for LastRun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.last_run {
            None => {
                write!(f, "Never")
            }
            Some(value) => {
                write!(f, "{}", value.format("%F %R"))
            }
        }
    }
}
