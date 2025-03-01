# 使用方法

anyframe-rsは、コマンド履歴、ディレクトリ、プロセスなどのデータソースに対して対話的なフィルタリングを提供します。

## 基本的な使い方

### コマンド履歴からコマンドを実行する

```zsh
anyframe-widget-execute-history
```

このコマンドは、コマンド履歴を表示し、選択したコマンドを実行します。

### ディレクトリに移動する

```zsh
anyframe-widget-cd-directory
```

このコマンドは、ディレクトリのリストを表示し、選択したディレクトリに移動します。

## キーバインディング

anyframe-rsウィジェットを好きなキーにマップできます：

```zsh
bindkey '^xr' anyframe-widget-execute-history
bindkey '^xd' anyframe-widget-cd-directory
# 必要に応じて他のキーバインディングを追加
```

## 設定

anyframe-rsの動作は、zstyleコマンドを使用して設定できます：

```zsh
# 使用するセレクタを指定
zstyle ":anyframe:selector:" use peco
# または
zstyle ":anyframe:selector:" use fzf

# セレクタのパスとオプションを指定
zstyle ":anyframe:selector:peco:" command 'peco --no-ignore-case'
zstyle ":anyframe:selector:fzf:" command 'fzf --extended'
```

## 利用可能なウィジェット

anyframe-rsは以下のウィジェットを提供します：

- `anyframe-widget-execute-history`: コマンド履歴からコマンドを選択して実行
- `anyframe-widget-cd-directory`: ディレクトリを選択して移動
- `anyframe-widget-checkout-git-branch`: Gitブランチを選択してチェックアウト
- `anyframe-widget-insert-git-branch`: Gitブランチ名を選択して挿入
- `anyframe-widget-git-add`: Gitステータスから変更ファイルを選択して追加

## カスタムウィジェットの作成

anyframe-rsを使用して独自のウィジェットを作成することもできます。詳細は[カスタマイズガイド](customization.md)を参照してください。
