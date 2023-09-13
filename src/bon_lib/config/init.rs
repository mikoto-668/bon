use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::path::{CONFIG_DIR, CONFIG_FILE, DEFAULT_CONFIG};

pub fn init() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    let mut flag: [bool; 2] = [false, false];

    if !config_dir_path.is_dir() {
        fs::create_dir(config_dir_path).expect("Something went wrong create directory.")
    } else {
        println!("Warning: Config directory is already exist.");
        flag[0] = true;
    }

    if !config_file_path.is_file() {
        let mut config_file: File = File::create(config_file_path).expect("Something went wrong create file.");
        config_file.write_all(DEFAULT_CONFIG.as_bytes()).expect("Something went wrong writing file.");
    } else {
        println!("Warning: Config file is already exist.");
        flag[1] = true;
    }

    if flag[0] && flag[1] {
        println!("Nothing todo ~/.config/bon/")
    } else {
        println!("Initialized config directory ~/.config/bon/");
    }
}
