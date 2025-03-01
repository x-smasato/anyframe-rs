# カスタマイズガイド

anyframe-rsは高度にカスタマイズ可能で、独自のソース、セレクタ、アクション、ウィジェットを作成できます。

## 独自のソースを作成する

新しいソースを作成するには、`Source`トレイトを実装します：

```rust
use anyframe_rs::{error, Result, sources::Source};

pub struct MyCustomSource;

impl Source for MyCustomSource {
    fn get_data(&self) -> Result<String> {
        // カスタムデータを取得するロジックを実装
        let data = "item1\nitem2\nitem3".to_string();
        Ok(data)
    }

    fn name(&self) -> &str {
        "my-custom-source"
    }
}
```

## 独自のセレクタを作成する

新しいセレクタを作成するには、`Selector`トレイトを実装します：

```rust
use anyframe_rs::{Result, selectors::Selector};
use std::process::Command;

pub struct MyCustomSelector {
    path: String,
}

impl MyCustomSelector {
    pub fn new(path: Option<String>) -> Self {
        Self {
            path: path.unwrap_or_else(|| "my-selector".to_string()),
        }
    }
}

impl Selector for MyCustomSelector {
    fn select(&self, input: &str, query: Option<&str>) -> Result<String> {
        // カスタムセレクタのロジックを実装
        // 例：外部コマンドを実行して選択を処理
        let mut cmd = Command::new(&self.path);
        // コマンドの設定...
        
        // 選択結果を返す
        Ok("Selected item".to_string())
    }

    fn name(&self) -> &str {
        "my-custom-selector"
    }
}
```

## 独自のアクションを作成する

新しいアクションを作成するには、`Action`トレイトを実装します：

```rust
use anyframe_rs::{Result, actions::Action};

pub struct MyCustomAction;

impl Action for MyCustomAction {
    fn perform(&self, item: &str) -> Result<()> {
        // 選択されたアイテムに対する操作を実装
        println!("Selected: {}", item);
        Ok(())
    }

    fn name(&self) -> &str {
        "my-custom-action"
    }
}
```

## 独自のウィジェットを作成する

新しいウィジェットを作成するには、`Widget`トレイトを実装します：

```rust
use anyframe_rs::{
    actions::Action,
    selectors::Selector,
    sources::Source,
    widgets::Widget,
    Result,
};

pub struct MyCustomWidget<S: Source, F: Selector, A: Action> {
    source: S,
    selector: F,
    action: A,
}

impl<S: Source, F: Selector, A: Action> MyCustomWidget<S, F, A> {
    pub fn new(source: S, selector: F, action: A) -> Self {
        Self {
            source,
            selector,
            action,
        }
    }
}

impl<S: Source, F: Selector, A: Action> Widget for MyCustomWidget<S, F, A> {
    fn run(&self) -> Result<()> {
        let data = self.source.get_data()?;
        let selected = self.selector.select(&data, None)?;
        self.action.perform(&selected)?;
        Ok(())
    }

    fn name(&self) -> &str {
        "my-custom-widget"
    }
}
```

## Zshでの使用

カスタムウィジェットをZshで使用するには、以下のようにanyframe.plugin.zshファイルに関数を追加します：

```zsh
# カスタムウィジェットの定義
function anyframe-widget-my-custom-widget {
    anyframe-rs my-custom-widget
}
```

そして、キーバインディングを設定します：

```zsh
bindkey '^x^m' anyframe-widget-my-custom-widget
```
