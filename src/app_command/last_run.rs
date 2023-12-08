/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-09
 */

use crate::{config, format_last_run};

pub fn do_it() {
    let last_run = config::load_last_run();
    crate::print_info!("Last run: {}", format_last_run(last_run));
}
