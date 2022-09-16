# rust-beginner

Udemy Rustプログラミング入門 (最高峰・最難解言語)

## 設定

### settings.json

補完が効かない場合は以下を`settings.json`に追加する。

```json
  "[rust]":{
    "editor.defaultFormatter": "rust-lang.rust",
    "editor.formatOnSave":true
  }
```

なお、rust-analyzer を使う場合は以下のように定義する。

```json
  "[rust]":{
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave":true
  }
```

## 最初に知っておくべき知識

### crate.io

外部ライブラリは[crate.io](https://crates.io/)を使うと良い。

### 基本コマンド

プロジェクトを新規作成する。

```bash
cargo new hello_world
```

ライブラリとしてプロジェクトを新規に作成する。

```bash
cargo new --lib hello_world
```

構文をチェックする。

```bash
cargo check
```

プロジェクトをビルドする。

```bash
cargo build
```

プロジェクトを実行する。

```bash
cargo run
```

### バインドとは

変数に値を代入する動作のこと。

### ドロップとは

メモリ解放のこと

### アロケーターとは

メモリを解放してくれる仕組みのこと

### 所有権とは

データを所有する変数が持つ権利(所有権)であり、メモリ解放時には解放の責任を追う。
静的領域に格納される場合はデータを解放する必要がないので所有権を持たない。
とりわけ、ヒープ領域でメモリを確保する際に発生する権利である。

※つまり、文字列スライスには所有権がない。

String型やVector型には所有権が存在する。

### 借用とは

所有権を移動することなく変数を参照する権限のみを貸し出すこと

### 所有権という仕組みがある理由

メモリの二重解放エラーを防ぐ役割がある。

### メモリの二重解放エラーとは

データの参照を持つポインタ変数がデータを解放した後、同じデータを参照している変数が  
再度、メモリの解放を実行してしまうことによって発生するエラー

所有権を持つ変数だけがメモリ解放の責任を負うことで所有権を持たない変数による解放を防ぐことができる。
