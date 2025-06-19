/*
 * Copyright (c) 2025 Paul Sobolik
 * Created 2025-06-06
 */
use crate::data::paths::Paths;
use crate::print_info;

pub fn command() {
    if let Some(data) = Paths::data_path() {
        print_info!("Data path: {}", data.display());
    }
    if let Some(config) = Paths::config_path() {
        print_info!("Config path: {}", config.display());
    }
}
