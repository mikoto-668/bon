use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use home_dir::HomeDirExt;

use crate::bon_lib::utils::structure::Config;

pub fn check_existence_dir(dir_path: PathBuf) -> bool {
    if dir_path.is_dir() {
        println!("> Dir `{:?}` is already exists.", dir_path.file_name());
        return true;
    } else {
        println!("> Dir `{:?}` is not found.", dir_path.file_name());
        return false;
    }
}

pub fn check_existence_file(file_path: PathBuf) -> bool {
    if file_path.is_file() {
        println!("> File `{:?}` is already exists.", file_path.file_name());
        return true;
    } else {
        println!("> File `{:?}` is not found.", file_path.file_name());
        return false;
    }
}

pub fn get_file_list_from_struct_config(config: Config) -> Vec<String> {
    let profiles: BTreeMap<String, String> = config.files;
    let files: Vec<&String> = profiles.keys().clone().collect::<Vec<&String>>();
    let mut result: Vec<String> = [].to_vec();
    for f in files {
        result.push(f.to_owned());
    }
    return result;
}

pub fn read_config_to_string(path: PathBuf) -> Config {
    let mut config_file: File = File::open(path).unwrap();
    let mut contents: String = String::new();

    config_file.read_to_string(&mut contents).unwrap();

    let config: Config = serde_json::from_str(&contents).unwrap();

    return config;
}

pub fn read_file_to_string(path: PathBuf) -> String {
    let mut file: File = File::open(path).expect("Something went wrong");
    let mut result_string: String = String::new();
    file.read_to_string(&mut result_string).unwrap();
    return result_string;
}

pub fn write_string_to_file(path_string: String, contents_string: String) {
    // ファイル作成
    let mut file: File = File::create(Path::new(&path_string).expand_home().unwrap()).unwrap();
    // 書き出し
    file.write_all(contents_string.as_bytes()).unwrap();
}
