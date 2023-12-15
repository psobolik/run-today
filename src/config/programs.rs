/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use super::consts;
use crate::config::program::Program;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error};
use std::path::Path;

pub fn load(path: &Path) -> Vec<Program> {
    match File::open(path.join(consts::PROGRAMS_FILE)) {
        Ok(file) => {
            match serde_json::from_reader(BufReader::new(file)) {
                Ok(programs) => programs,
                Err(error) => panic!("Error loading programs: {}", error),
            }
        },
        Err(_) => vec![],
    }
}

fn renumbered(programs: &[Program]) -> Vec<Program> {
    programs
        .iter()
        .enumerate()
        .map(|(index, program)| Program::new(index + 1, program.name(), program.args()))
        .collect()
}

pub fn store(programs: &[Program], path: &Path) -> Result<(), Error> {
    let mut file = BufWriter::new(File::create(path.join(consts::PROGRAMS_FILE))?);
    Ok(serde_json::to_writer(&mut file, &renumbered(programs))?)
}
