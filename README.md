# usage

```sh
# makefileのテンプレートが生成される
# 単一ファイルの場合のみ実行できる
templatehoshii dump make > Makefile

# templateを既定のファイル名で展開
# 既に存在してたらエラー
templatehoshii dump make --to-file

# list current all templates
templatehoshii list

# 特定のファイルを登録
templatehoshii add circleci .circleci/config.yml

# 特定のディレクトリを登録
templatehoshii add hoge ./src/

```
