use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

use home_dir::HomeDirExt;
use inquire::Text;

use crate::bon_lib::utils::{
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
};

pub fn add() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    if !config_dir_path.is_dir() {
        println!("Config directory is not exist.");
        exit(1);
    }

    if !config_file_path.is_file() {
        eprintln!("Config file is not exist.");
        exit(1);
    }

    let mut config_file: File =
        File::open(&config_file_path).expect("Something went wrong opening file.");

    let mut config_content: String = String::new();

    config_file
        .read_to_string(&mut config_content)
        .expect("Something went wrong reading the file");

    if config_content.is_empty() {
        println!("Error: Cannot read because your config file is empty.");
        exit(1);
    }

    let mut config: Config =
        serde_json::from_str(&config_content).expect("Something went wrong transplant to config.");

    let k = Text::new("File name:").prompt().expect("Something went wrong accepting text");
    let v = "tmpl/".to_string() + &Text::new("File templat path:").prompt().expect("Something went wrong accepting text");

    config.files.insert(k, v);

    let mut config_file: File =
        File::create(&config_file_path).expect("Somethng went wrong opening file.");
    let result_content: String =
        serde_json::to_string_pretty(&config).expect("Something went wrong translate to string");

    config_file
        .write_all(result_content.as_bytes())
        .expect("Something went wrong writing in file.");
}

