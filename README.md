# ironhtml

A minimal, zero-dependency, `no_std` HTML5 library for Rust following the
[WHATWG HTML Living Standard](https://html.spec.whatwg.org/).

## Crates

| Crate                                             | Description                                                              |
| ------------------------------------------------- | ------------------------------------------------------------------------ |
| [ironhtml](crates/ironhtml)                       | Core HTML builder with type-safe element construction and XSS protection |
| [ironhtml-elements](crates/ironhtml-elements)     | All 110+ HTML5 elements as zero-sized types with content category traits |
| [ironhtml-attributes](crates/ironhtml-attributes) | Typed attributes (global + element-specific) with validation             |
| [ironhtml-macro](crates/ironhtml-macro)           | Proc-macro for ergonomic HTML generation with Rust-like syntax           |
| [ironhtml-parser](crates/ironhtml-parser)         | HTML5 parser and validator following WHATWG spec                         |
| [ironhtml-bootstrap](crates/ironhtml-bootstrap)   | Type-safe Bootstrap 5.3 components                                       |

## Features

- Zero dependencies (only uses `alloc`)
- `no_std` compatible, builds to WebAssembly
- Type-safe builder pattern with compile-time validation
- Automatic XSS protection via HTML/attribute escaping
- Complete HTML5 element and attribute coverage
- Strict clippy lints: `clippy::all`, `clippy::pedantic`, `clippy::nursery`

## Documentation

See the [API documentation](https://leakix.github.io/ironhtml/ironhtml/) for
usage examples and detailed reference.

## License

MIT
