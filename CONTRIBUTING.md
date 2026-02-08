# Contributing to ironhtml

## Getting Started

```bash
git clone https://github.com/LeakIX/ironhtml.git
cd ironhtml
make build
make test
```

## Development Workflow

1. Create a branch from `master`
2. Make your changes
3. Run `make format` before committing
4. Run `make lint` (clippy with `all`, `pedantic`, `nursery` lints)
5. Run `make test` and `make test-doc`
6. Open a pull request against `master`

## Available Make Targets

```
make build          Build all crates
make test           Run all tests
make test-doc       Run documentation tests
make lint           Run clippy
make format         Format Rust, TOML, and Markdown
make bench          Run benchmarks
make doc            Generate documentation
make ci             Run all CI checks locally
```

## MSRV Policy

The minimum supported Rust version is declared in `Cargo.toml`
(`rust-version`) and tested in CI. Bumping the MSRV is a minor version
change.

## Commit Messages

- Use imperative mood (`fix`, `add`, `update`, not `fixed`, `added`)
- Wrap title at 80 characters
- Reference issues with `Closes #N` when applicable
- CHANGELOG updates must be in their own commit

## License

By contributing, you agree that your contributions will be licensed under
the MIT License.
