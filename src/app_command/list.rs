/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::config;

pub fn do_it() {
    let programs = config::load_programs();
    if programs.is_empty() {
        crate::print_info!("No programs to list")
    } else {
        for program in programs {
            println!("[{}] {program}", program.id());
        }
    }
}
