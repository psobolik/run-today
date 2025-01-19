/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::print_info;
use crate::data::last_run;

pub fn last_run() {
    let last_run = last_run::load();
    print_info!("Last run: {}", last_run::format_last_run(last_run));
}
