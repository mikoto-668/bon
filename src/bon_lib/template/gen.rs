use std::{
    path::{Path, PathBuf}, fs::File,
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::{
    funcs::{check_existence, read_file_to_config_structure},
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
};

pub fn gen() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    check_existence(config_dir_path.clone(), config_file_path.clone());

    let config: Config = read_file_to_config_structure(config_file_path.clone());

    for entry in config.files.values() {
        let str: String = entry.to_owned();
        let path: &Path = Path::new(&str);
        let all_path: PathBuf = config_dir_path.clone().join(path);

        File::create(all_path).expect("Something went wrong");
    }

    println!("Generated")
}
