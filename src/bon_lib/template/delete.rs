use std::{
    path::{Path, PathBuf},
    process::exit,
};

use home_dir::HomeDirExt;
use inquire::{MultiSelect, Confirm};

use crate::bon_lib::utils::{
    funcs::{check_existence, read_file_to_config_structure, write_string_to_file, get_file_list_from_struct_config},
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
};

pub fn delete() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    check_existence(config_dir_path, config_file_path.clone());

    let mut config: Config = read_file_to_config_structure(config_file_path.clone());

    let file_list: Vec<String> = get_file_list_from_struct_config(config.clone());

    let selection: Option<Vec<String>> = MultiSelect::new("Files:", file_list).prompt_skippable().expect("Something went wrong");

    if selection == None {
        exit(1);
    }

    let confirm = Confirm::new("Delete are you sure want to?:").prompt_skippable().expect("Something went wrong");

    if confirm == None {
        exit(1);
    }

    if confirm.unwrap() {
        for entry in selection.unwrap() {
            config.files.remove(&entry);
        }
        println!("Deleted");
    } else {
        println!("Nothing to do");
    }

    let result_content: String =
        serde_json::to_string_pretty(&config).expect("Something went wrong translate to string");

    write_string_to_file(CONFIG_FILE.to_owned(), result_content);
}
