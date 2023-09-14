use std::path::{Path, PathBuf};

use home_dir::HomeDirExt;
use inquire::{MultiSelect, Text};

use crate::bon_lib::utils::{
    funcs::{check_existence, read_file_to_config_structure, write_string_to_file},
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
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

    let items: Option<Vec<&str>> =
        MultiSelect::new("Edit...:", ["user", "email", "editor"].to_vec())
            .prompt_skippable()
            .expect("Something went wrong on choosing item.");

    let mut selected_items: Vec<&str> = [].to_vec();

    if items != None {
        selected_items = items.unwrap()
    }

    for i in selected_items {
        match i {
            "user" => {
                let tmp: Option<String> = Text::new("user:")
                    .with_help_message("Please input user name")
                    .prompt_skippable()
                    .expect("Something went wrong");
                if tmp != None {
                    config.user = tmp.unwrap();
                    println!("Your name has been set to `{}`.", config.user);
                }
            }
            "email" => {
                let tmp: Option<String> = Text::new("email:")
                    .with_help_message("Please input email")
                    .prompt_skippable()
                    .expect("Something went wrong");
                if tmp != None {
                    config.email = tmp.unwrap();
                    println!("Your mail address has been set to `{}`.", config.email);
                }
            }
            "editor" => {
                let tmp: Option<String> = Text::new("editor:")
                    .with_help_message("Please input editor")
                    .prompt_skippable()
                    .expect("Something went wrong");
                if tmp != None {
                    config.editor = tmp.unwrap();
                    println!("Your editor has been set to `{}`.", config.editor);
                }
            }
            _ => println!("Nothing todo!"),
        }
    }

    let result_content: String =
        serde_json::to_string_pretty(&config).expect("Something went wrong translate to string");

    write_string_to_file(CONFIG_FILE.to_owned(), result_content);
}
