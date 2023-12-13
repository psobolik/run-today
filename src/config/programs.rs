/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use super::consts;
use crate::config::program::Program;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

pub fn load(path: &Path) -> Vec<Program> {
    // We only panic if the file contains unparseable data.
    if let Ok(mut file) = File::open(path.join(consts::PROGRAMS_FILE)) {
        let mut json = String::new();
        let _ = file.read_to_string(&mut json);
        if let Ok(programs) = serde_json5::from_str(&json) {
            programs
        } else {
            panic!("Error loading programs");
        }
    } else {
        vec![]
    }
}

pub fn store(programs: &[Program], path: &Path) -> Result<(), Error> {
    fn renumbered(programs: &[Program]) -> Vec<Program> {
        programs
            .iter()
            .enumerate()
            .map(|(index, program)| Program::new(index + 1, program.name(), program.args()))
            .collect()
    }
    let mut file = File::create(path.join(consts::PROGRAMS_FILE))?;
    match serde_json5::to_string(&renumbered(programs)) {
        Ok(json) => write!(file, "{json}"),
        Err(error) => Err(Error::new(ErrorKind::InvalidData, error)),
    }
}
