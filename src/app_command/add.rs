/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-10
 */

use crate::config;
use crate::config::program::Program;
use crate::{print_error, print_info};

pub fn add(string: String) -> u8 {
    match Program::from_string(string) {
        Ok(program) => {
            let mut programs = config::load_programs();
            if !is_unique(&program, &programs) {
                print_error!(r#"Won't add duplicate program: "{program}""#);
                return 1;
            }
            let message = format!(r#"Added program: "{program}""#);
            programs.push(program);
            match config::store_programs(&programs) {
                Ok(_) => {
                    print_info!("{message}");
                    0
                }
                Err(error) => {
                    print_error!(r#"Error adding program: "{error}""#);
                    1
                }
            }
        }
        Err(error) => {
            print_error!(r#"Error parsing program: "{error}""#);
            1
        }
    }
}

fn is_unique(program: &Program, programs: &[Program]) -> bool {
    !programs
        .iter()
        .any(|p| p.to_string() == program.to_string())
}
