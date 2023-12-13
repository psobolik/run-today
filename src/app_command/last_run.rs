/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::{config, print_info};
use chrono::{DateTime, Local};

pub fn last_run() {
    let last_run = config::load_last_run();
    print_info!("Last run: {}", format_last_run(last_run));
}

pub fn format_last_run(last_run: Option<DateTime<Local>>) -> String {
    if let Some(lr) = last_run {
        lr.format("%F %R").to_string()
    } else {
        String::from("Never")
    }
}
