/*
 * Copyright (c) 2025 Paul Sobolik
 * Created 2025-06-06
 */
use crate::data::consts;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

pub struct Paths;

impl Paths {
    pub fn data_path() -> Option<PathBuf> {
        BaseDirs::new()
            .map(|base_dirs| Path::new(base_dirs.data_local_dir()).join(consts::FOLDER_NAME))
    }
    pub fn config_path() -> Option<PathBuf> {
        BaseDirs::new()
            .map(|base_dirs| Path::new(base_dirs.config_local_dir()).join(consts::FOLDER_NAME))
    }
}
