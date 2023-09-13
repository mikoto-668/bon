# ライブラリ

## ファイル分け

```
src/
 ├ config/
 │  ├ init.rs     - コンフィグ生成
 │  ├ edit.rs     - ユーザ情報編集
 │  ├ add.rs      - ファイルプロファイル追加
 │  ├ update.rs   - ファイルプロファイル変更
 │  ├ delete.rs   - ファイルプロファイル削除
 │  └ template.rs - テンプレートファイル編集
 ├ utils.rs
 │  ├ 構造体
 │  └ static変数
 └ voyage.rs
```

## 必要なもの

### 関数

- initialize_config
- edit_user_infomation
- add_file_profile
- update_file_profile
- delete_file_profile
- voyage

### 構造体

- config
  - user
  - email
  - editor
  - files
    - filename: filepath

### static変数

- CONFIG_DIR = ~/.config/bon
- CONFIG_FILE = ~/.config/bonconfig.json
- DEFAULT_CONFIG
