/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::print_info;
use crate::data::last_run::LastRun;

pub fn command() {
    let last_run = LastRun::new();
    print_info!("Last run: {}", last_run);
}
