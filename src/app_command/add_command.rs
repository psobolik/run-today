/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-10
 */

// use crate::data;
use crate::data::programs;
use crate::data::program::Program;
use crate::{print_error, print_info};

pub fn command(string: String) -> u8 {
    match Program::from_string(string) {
        Ok(program) => {
            let mut programs = programs::load();
            if !is_unique(&program, &programs) {
                print_error!(r#"Won't add duplicate program: "{program}""#);
                return 1;
            }
            let message = format!(r#"Added program: "{program}""#);
            programs.push(program);
            match programs::store(&programs) {
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
