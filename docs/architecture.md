# アーキテクチャ

anyframe-rsは、モジュール化された設計を採用しており、以下の主要コンポーネントで構成されています：

## コンポーネント

### ソース (Sources)

ソースは、フィルタリングするデータを提供します。例えば：

- コマンド履歴
- ディレクトリ
- プロセス
- その他

各ソースは`Source`トレイトを実装し、`get_data()`メソッドを通じてデータを提供します。

```rust
pub trait Source {
    /// ソースからデータを取得
    fn get_data(&self) -> Result<String>;

    /// ソースの名前を取得
    fn name(&self) -> &str;
}
```

### セレクタ (Selectors)

セレクタは、対話的なフィルタリングツールです：

- peco
- percol
- fzf
- fzf-tmux

各セレクタは`Selector`トレイトを実装し、`select()`メソッドを通じてユーザーに選択肢を提示します。

```rust
pub trait Selector {
    /// 与えられた入力でセレクタを実行
    fn select(&self, input: &str, query: Option<&str>) -> Result<String>;

    /// セレクタの名前を取得
    fn name(&self) -> &str;
}
```

### アクション (Actions)

アクションは、選択されたアイテムに対して操作を実行します：

- 実行
- 挿入
- 配置

各アクションは`Action`トレイトを実装し、`perform()`メソッドを通じて選択されたアイテムに対して操作を実行します。

```rust
pub trait Action {
    /// 選択されたアイテムに対してアクションを実行
    fn perform(&self, item: &str) -> Result<()>;

    /// アクションの名前を取得
    fn name(&self) -> &str;
}
```

### ウィジェット (Widgets)

ウィジェットは、ソース、セレクタ、アクションを組み合わせて有用な機能を作成します。各ウィジェットは`Widget`トレイトを実装し、`run()`メソッドを通じて機能を実行します。

```rust
pub trait Widget {
    /// ウィジェットを実行
    fn run(&self) -> Result<()>;

    /// ウィジェットの名前を取得
    fn name(&self) -> &str;
}
```

## データフロー

anyframe-rsのデータフローは以下の通りです：

1. ウィジェットが呼び出される
2. ウィジェットがソースからデータを取得
3. ウィジェットがセレクタを使用してユーザーにデータを提示
4. ユーザーがアイテムを選択
5. ウィジェットがアクションを使用して選択されたアイテムに対して操作を実行

## 拡張性

anyframe-rsは拡張性を考慮して設計されており、新しいソース、セレクタ、アクション、ウィジェットを簡単に追加できます。詳細は[カスタマイズガイド](customization.md)を参照してください。
