/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::data::last_run::LastRun;
use crate::print_info;

pub fn command() {
    print_info!("Last run: {}", LastRun::display());
}
