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
