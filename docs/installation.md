# インストール方法

anyframe-rsをインストールするには複数の方法があります。

## 前提条件

anyframe-rsを使用するには、以下のフィルタリングツールのいずれかが必要です：

- [peco](https://github.com/peco/peco)
- [percol](https://github.com/mooz/percol)
- [fzf](https://github.com/junegunn/fzf)
- fzf-tmux

また、Zshシェルが必要です。

## ソースからのインストール

```sh
git clone https://github.com/x-smasato/anyframe-rs
cd anyframe-rs
cargo build --release
cp target/release/anyframe-rs ~/.local/bin/
```

## Cargoからのインストール（将来的に対応予定）

```sh
cargo install anyframe-rs
```

## Zshプラグインとしてのセットアップ

anyframe-rsをインストールした後、以下を`.zshrc`に追加します：

```zsh
# anyframe-rsプラグインを読み込む
source /path/to/anyframe-rs/anyframe.plugin.zsh
```

## キーバインディングの設定

anyframe-rsウィジェットを好きなキーにマップできます：

```zsh
bindkey '^xr' anyframe-widget-execute-history
bindkey '^xd' anyframe-widget-cd-directory
# 必要に応じて他のキーバインディングを追加
```
