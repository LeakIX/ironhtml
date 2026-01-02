# html-builder

A minimal, zero-dependency, `no_std` HTML builder for Rust following the
[WHATWG HTML Living Standard](https://html.spec.whatwg.org/).

## Features

- Zero dependencies (only uses `alloc`)
- `no_std` compatible
- Type-safe builder pattern
- Automatic XSS protection via HTML/attribute escaping
- Self-closing tag detection

## Usage

```rust
use html_builder::{Html, Element};

let html = Html::new()
    .raw("<!DOCTYPE html>")
    .elem("html", |e| e
        .attr("lang", "en")
        .child("head", |e| e
            .child("meta", |e| e.attr("charset", "UTF-8"))
            .child("title", |e| e.text("Hello"))
        )
        .child("body", |e| e
            .child("h1", |e| e.text("Hello, World!"))
        )
    )
    .build();
```

## License

MIT
