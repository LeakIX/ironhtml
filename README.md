# html-builder

A minimal, zero-dependency, `no_std` HTML builder for Rust.

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

## API

| Method                | Description                |
| --------------------- | -------------------------- |
| `Element::new(tag)`   | Create element             |
| `.attr(name, value)`  | Add attribute              |
| `.bool_attr(name)`    | Add boolean attribute      |
| `.class(name)`        | Add/append class           |
| `.id(id)`             | Set id                     |
| `.text(content)`      | Add escaped text           |
| `.raw(html)`          | Add raw HTML               |
| `.child(tag, fn)`     | Add child element          |
| `.children(iter, fn)` | Add children from iterator |
| `.when(cond, fn)`     | Conditional rendering      |

## License

MIT
