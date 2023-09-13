use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::path::{CONFIG_DIR, CONFIG_FILE, DEFAULT_CONFIG};

pub fn init() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR).expand_home().unwrap();
    let config_file_path: PathBuf = Path::new(CONFIG_FILE).expand_home().unwrap();

    if !config_dir_path.is_dir() {
        fs::create_dir(config_dir_path).unwrap()
    } else {
        println!("Config directory is already exist.")
    }

    if !config_file_path.is_file() {
        let mut config_file: File = File::create(config_file_path).unwrap();
        config_file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
    } else {
        println!("Config file is already exist.")
    }
}
