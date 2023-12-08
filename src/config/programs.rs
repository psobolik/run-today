/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::config::program::Program;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

pub fn load(path: &Path) -> Vec<Program> {
    // We only panic if the file contains unparseable data.
    if let Ok(mut file) = File::open(path.join(crate::config::PROGRAMS_FILE)) {
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
    let numbered: Vec<Program> = programs
        .iter()
        .enumerate()
        .map(|(index, program)| Program::new(index + 1, program.name(), program.args()))
        .collect();
    let mut file = File::create(path.join(crate::config::PROGRAMS_FILE))?;
    if let Ok(json) = serde_json5::to_string(&numbered) {
        write!(file, "{}", json)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Error formatting programs"))
    }
}
