# lira

No dependency, fast Rust eDSL for writing HTML

## Usage

Get it from Cargo: [lira](https://crates.io/crates/lira)

```bash
$ cargo add lira
```

```rust
use lira::prelude::*;

fn layout(content: impl Renderable) -> impl Renderable {
    html()
        .lang("en")
        .child(
            head()
                .child(title().text("lira"))
                .child(link().href("/app.css").rel(Rel::Stylesheet)))
        .child(
            body()
                .child(
                    main()
                        .class("container")
                        .child(content))
                .child(
                    script()
                        .src("https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js"))),
}

fn counter_button(value: i32) -> impl Renderable {
    button()
        .class("btn btn-primary")
        .attr("hx-post", format!("/counter/{}", value + 1))
        .attr("hx-swap", "outerHTML")
        .text(format!("+{}", value))
}

fn page() -> impl Renderable {
    div()
        .class("card bg-base-200")
        .child(div()
            .class("card-body")
            .child(h1().class("text-2xl").text("lira"))
            .child(p().text("No dependency fast Rust eDSL for writing HTML"))
            .child(counter_button(1)))
        // wrap content into layout
        .map(|node| layout(node))
}

// we use axum here as an example
use axum::{Router, extract::Path, http::header, response::{IntoResponse, Html}, routing::{get, post}};

async fn css_handler() -> impl IntoResponse {
    let css = include_str!("../assets/app.css");
    ([(header::CONTENT_TYPE, "text/css")], css).into_response()
}

#[tokio::main]
fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/app.css", get(css_handler))
        .route("/", get(Html(page().render())))
        .route("/counter/{value}", post(async |Path(value) : Path<i32>|
            Html(counter_button(value).render())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

## License

MIT
