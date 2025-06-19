/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::data::programs::Programs;
use crate::print_info;

pub fn command() {
    let programs = Programs::load();
    if programs.is_empty() {
        print_info!("No programs to list")
    } else {
        for program in &programs {
            println!("[{}] {program}", program.id());
        }
    }
}
