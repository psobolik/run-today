/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use super::consts;
use crate::data::program::Program;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::{Path, PathBuf};
use directories::BaseDirs;

fn config_path() -> Option<PathBuf> {
    BaseDirs::new().map(|base_dirs| {
        Path::new(base_dirs.config_local_dir())
            .join(consts::FOLDER_NAME)
    })
}

pub fn load() -> Vec<Program> {
    if let Some(config_path) = config_path() {
        match File::open(config_path.join(consts::PROGRAMS_FILE)) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(programs) => programs,
                Err(error) => panic!("Error loading programs: {}", error),
            },
            Err(_) => vec![],
        }
    } else {
        vec![]
    }
}

pub fn store(programs: &[Program]) -> Result<(), Error> {
    if let Some(config_path) = config_path() {
        std::fs::create_dir_all(&config_path)?;
        let mut file = BufWriter::new(File::create(config_path.join(consts::PROGRAMS_FILE))?);
        Ok(serde_json::to_writer(&mut file, &renumbered(programs))?)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            "Error getting configuration file path",
        ))
    }
}

fn renumbered(programs: &[Program]) -> Vec<Program> {
    programs
        .iter()
        .enumerate()
        .map(|(index, program)| Program::new(index + 1, program.name(), program.args()))
        .collect()
}
