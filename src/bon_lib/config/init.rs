use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::{
    funcs::{check_existence_dir, check_existence_file, write_string_to_file},
    path::{CONFIG_DIR, CONFIG_FILE, DEFAULT_CONFIG},
};

pub fn init() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    if !check_existence_dir(config_dir_path.clone()) {
        fs::create_dir(config_dir_path).expect("Something went wrong create directory.")
    }

    if !check_existence_file(config_file_path) {
        println!("Initialized config directory ~/.config/bon/");
    } else {
        exit(1);
    }

    write_string_to_file(CONFIG_FILE.to_string(), DEFAULT_CONFIG.to_string())
}
