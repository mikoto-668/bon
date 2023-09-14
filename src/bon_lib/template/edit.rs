use std::{path::{Path, PathBuf}, process::{exit, Command}, os::unix::process::CommandExt};

use home_dir::HomeDirExt;
use inquire::Select;

use crate::bon_lib::utils::{
    funcs::{check_existence, read_file_to_config_structure, get_file_list_from_struct_config},
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
};

pub fn edit_template() {
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR)
        .expand_home()
        .expect("Something went wrong on expand tilde.");
    let config_file_path: PathBuf = Path::new(CONFIG_FILE)
        .expand_home()
        .expect("Something went wrong on expand tilde.");

    check_existence(config_dir_path.clone(), config_file_path.clone());

    let config: Config = read_file_to_config_structure(config_file_path.clone());

    let file_list: Vec<String> = get_file_list_from_struct_config(config.clone());

    let selection: Option<String> = Select::new("File:", file_list).prompt_skippable().expect("Something went wrong");

    if selection == None {
        exit(1);
    }

    let editor: String = config.editor;
    let template: String = config.files.get(&selection.unwrap()).unwrap().to_owned();

    let all_path: PathBuf = config_dir_path.join(template);

    Command::new(editor).arg(all_path).exec();
}
