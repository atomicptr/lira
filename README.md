# lira

No dependency, fast Rust eDSL for writing HTML

## Usage

```bash
$ cargo add lira
```

```rust
use lira::prelude::*;

fn view() -> String {
    div()
        .class("card bg-base-200")
        .child(
            div()
                .class("card-body")
                .child(
                    h1().class("text-2xl").text("lira")
                )
                .child(
                    p().text("No dependency fast Rust eDSL for writing HTML")
                )
        )
        .render()
}
```

## License

MIT
