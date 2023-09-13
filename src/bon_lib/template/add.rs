use std::{
    path::{Path, PathBuf},
    process::exit,
};

use home_dir::HomeDirExt;
use inquire::Text;

use crate::bon_lib::utils::{
    funcs::{check_existence, read_file_to_config_structure, write_string_to_file},
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

    check_existence(config_dir_path, config_file_path.clone());

    let mut config: Config = read_file_to_config_structure(config_file_path.clone());

    let file_name: Option<String> = Text::new("File name:")
        .with_help_message("Please input file name")
        .prompt_skippable()
        .expect("Something went wrong");

    if file_name == None {
        exit(1);
    }

    let file_path: Option<String> = Text::new("File path:")
        .with_help_message("Please input the path to template file")
        .prompt_skippable()
        .expect("Something went wrong");

    if file_path != None {
        config.files.insert(file_name.unwrap(), file_path.unwrap());
    }

    let result_content: String =
        serde_json::to_string_pretty(&config).expect("Something went wrong translate to string");

    write_string_to_file(CONFIG_FILE.to_owned(), result_content);
}
