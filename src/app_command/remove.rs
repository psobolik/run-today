/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-10
 */

use crate::config;
use crate::config::program::Program;

pub fn do_it(program_id: usize) -> u8 {
    let programs = config::load_programs();
    match program_with_id(program_id, &programs) {
        Some(program) => {
            let filtered: Vec<Program> = programs
                .iter()
                .filter(|program| program.id() != program_id)
                .map(|program| Program::new(0, program.name(), program.args()))
                .collect();
            match config::store_programs(&filtered) {
                Ok(_) => {
                    crate::print_info!(r#"Removed program: "{program}""#);
                    0
                }
                Err(error) => {
                    crate::print_error!(r#"Error removing program: "{error}""#);
                    1
                }
            }
        }
        None => {
            crate::print_error!("Invalid program ID: {program_id}");
            1
        }
    }
}

fn program_with_id(program_id: usize, programs: &[Program]) -> Option<&Program> {
    programs.iter().find(|program| program.id() == program_id)
}
