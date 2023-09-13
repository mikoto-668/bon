use std::{
    collections::BTreeMap,
    path::{Path, PathBuf}, process::exit,
};

use home_dir::HomeDirExt;
use inquire::MultiSelect;

use crate::bon_lib::utils::{
    funcs::{
        get_file_list_from_struct_config, read_file_to_string,
        write_string_to_file, read_file_to_config_structure, check_existence,
    },
    path::{CONFIG_DIR, CONFIG_FILE},
    structure::Config,
};

pub fn voyage() {
    // コンフィグファイルへのパスを定義
    let config_file_path: PathBuf = Path::new(CONFIG_FILE).expand_home().unwrap();
    let config_dir_path: PathBuf = Path::new(CONFIG_DIR).expand_home().unwrap();

    check_existence(config_dir_path.clone(), config_file_path.clone());

    // コンフィグファイルを読み込んでstructにぶち込む
    let config: Config = read_file_to_config_structure(config_file_path.clone());

    // コンフィグ構造体からプロファイル一覧を取得
    let profiles: BTreeMap<String, String> = config.clone().files;
    // プロファイル一覧からファイル一覧を配列として抽出
    let file_list: Vec<String> = get_file_list_from_struct_config(config.clone());

    if file_list.is_empty() {
        println!("> File list is empty.");
        exit(1);
    }

    // ファイル一覧を使って生成するファイルを選択する
    let selected_files: Vec<String> = MultiSelect::new("File:", file_list).prompt().unwrap();

    for entry in selected_files {
        // ファイル名で検索してパス文字列を呼び出し
        let path_string: String = profiles.get(&entry).unwrap().to_owned();
        // パス文字列をコンフィグディレクトリに接続してパスにする
        let path: PathBuf = config_dir_path.join(path_string);

        let template: String = read_file_to_string(path);

        write_string_to_file(entry, template);
    }

    println!("Bon Voyage!");
}
