# BuildWrapper

- 自前のツールを Build した際にすぐに PATH を通すように，コンパイルされたアーティファクトを指定したディレクトリ(PATH がすでに通っているディレクトリ)にコピーする
- cargo build をラップする
- ディレクトリの指定方法は以下例のように環境変数 RUST_BIN_PATH に指定する

```shell
export RUST_BIN_PATH=/usr/local/bin/
```

- どうやって cargo build によって生成されるアーティファクトを取得する？
  - プロジェクト名が分かればいいのかな？
- workspace の時や，bin に沢山のファイルがある場合はどうする？
  - 出力してもらう名前をコマンド引数としてもらう？
