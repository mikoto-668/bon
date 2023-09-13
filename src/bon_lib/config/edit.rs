use std::path::{Path, PathBuf};

use home_dir::HomeDirExt;
use inquire::{MultiSelect, Text};

use crate::bon_lib::utils::{
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config, funcs::{read_file_to_config_structure, write_string_to_file, check_existence},
};

pub fn edit() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");


    check_existence(config_dir_path, config_file_path.clone());

    let mut config: Config = read_file_to_config_structure(config_file_path.clone());

    let items: Vec<&str> = MultiSelect::new("Edit...:", ["user", "email", "editor"].to_vec())
        .prompt()
        .expect("Something went wrong on choosing item.");

    for i in items {
        match i {
            "user" => config.user = Text::new("user:").prompt().expect("Something went wrong"),
            "email" => config.email = Text::new("email:").prompt().expect("Something went wrong"),
            "editor" => {
                config.editor = Text::new("editor:").prompt().expect("Something went wrong")
            }
            _ => println!("Nothing todo!"),
        }
    }

    let result_content: String =
        serde_json::to_string_pretty(&config).expect("Something went wrong translate to string");

    write_string_to_file(CONFIG_FILE.to_owned(), result_content);
}
