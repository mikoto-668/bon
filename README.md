<p align="center">
  <samp>
    <b>
      <h1 align="center">
        BON VOYAGE!
      </h1>
    </b>
  </samp>
</p>

## これは何？

予め定義しておいたプロファイルから、ファイルを生成するコマンド。

## 使い方

### サブコマンド一覧

`bon`は以下の3つのサブコマンドを持っている。

1. `config`
1. `template`
1. `voyage`

それぞれのサブコマンドの使い方について以下で説明する。

### config

`config`はコンフィグを生成したり、編集したりする機能がある。

- `config init`: コンフィグを新しく生成する。既に存在している場合、実行されずに終了する。
- `config edit`: コンフィグを編集する。どの項目を編集するか選択し、値を入力していく。

### template

`template`はテンプレートを生成する機能がある。

- `template add`: 新しいプロファイルを作成する。
- `template update`: プロファイルに記載されたパスを書き換える。
- `template delete`: プロファイルを削除する。
- `template gen`: プロファイルに記載されたパスに空ファイルを作成する。
- `template edit`: プロファイルに記載されたテンプレートを編集する。

### voyage

`voyage`は、コンフィグファイルを読み取って、テンプレートからファイルを生成する。

## インストール方法

### 必要なもの

Rustをインストールしておいてください。[こちら](https://www.rust-lang.org)

### インストール

```
cargo install --git https://github.com/yuni-hutsuka/bon.git
```

## Contributers

- [Yuni Hutsuka](mailto:yuni.wille999@gmail.com)
