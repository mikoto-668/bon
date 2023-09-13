use std::{
    fs::{self, File},
    io::{Result, Write},
    path::{Path, PathBuf},
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::path::{CONFIG_DIR, CONFIG_FILE, DEFAULT_CONFIG};

pub fn init() -> Result<()> {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    if !config_dir_path.is_dir() {
        fs::create_dir(config_dir_path)?
    } else {
        println!("Warning: Config directory is already exist.")
    }

    if !config_file_path.is_file() {
        let mut config_file: File = File::create(config_file_path)?;
        config_file.write_all(DEFAULT_CONFIG.as_bytes())?;
    } else {
        println!("Warning: Config file is already exist.")
    }

    println!("Initialized config directory ~/.config/bon/");

    Ok(())
}
