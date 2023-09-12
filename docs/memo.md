# コマンド設計

## 全体図

```
bon
 ├ config
 │  ├ init
 │  ├ edit
 │  ├ add
 │  ├ update
 │  └ delete
 └ voyage
```

## 各コマンド概要

### bon config init

`~/.config/bon/bonconfig.json`を作成する

### bon config edit

`~/.config/bon/bonconfig.json`のユーザ情報を編集する

### bon config add

`~/.config/bon/bonconfig.json`のファイル情報を一つ追加する

### bon config update FILE

`~/.config/bon/bonconfig.json`のファイル情報からFILEを選択してその情報を編集する

### bon voyage

実行するディレクトリに、`~/.config/bon/bonconfig.json`から読みとったファイル群を、選択して生成する
