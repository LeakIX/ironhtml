# CLAUDE.md

This file provides guidance when working with code in this repository.

## Project Overview

html-builder - a minimal, zero-dependency, `no_std` HTML builder for Rust.
Features type-safe builder pattern, automatic XSS protection, and self-closing
tag detection.

## Build Commands

All commands are in the Makefile. Run `make help` for full list.

```bash
make build            # Build all crates
make test             # Run all tests
make lint             # Lint with clippy
make format           # Format all code
make ci               # Run all CI checks
```

### Detailed commands

```bash
make format-rust      # Format Rust code
make format-toml      # Format TOML files
make format-check     # Check formatting without changes
make doc              # Generate documentation
make doc-open         # Generate and open docs in browser
make clean            # Clean build artifacts
```

## Code Structure

```
html-builder/
├── crates/
│   └── html-builder/     # Main library crate
│       └── src/lib.rs
├── Cargo.toml            # Workspace root
├── Makefile
└── .github/
    └── workflows/ci.yml
```

## Conventions

- Makefile targets have `.PHONY` declaration immediately before each target
- Makefile uses self-documenting help (`## comment` after target)
- Library is `no_std` by default, uses only `alloc`
- Zero external dependencies

## Development Guidelines

### Formatting

- **Always run `make format` before every commit and push**
- Rust: `make format-rust`
- TOML: `make format-toml`

### Pre-Commit Checklist

Before committing and pushing changes:

1. Run `make test` and ensure all tests pass
2. Run `make format` to format all code
3. Run `make lint` to check for linting issues

Or simply run `make ci` to run all checks at once.

### Branching Strategy

- **main/master**: Production branch, protected
- **Never push directly to main/master**. Always create a feature branch and
  submit a PR.
- Feature branches should be named descriptively (e.g., `fix/escape-bug`,
  `feat/new-element`)

### Commit Standards

- No emojis in commit messages
- Never mention AI assistants in commits or as co-authors
- Wrap commit message titles at 72 characters
- Wrap commit message body at 80 characters
- Use conventional commit prefixes: `feat:`, `fix:`, `docs:`, `chore:`,
  `refactor:`, `test:`

### Code Style

- Rust: follow clippy lints with `-D warnings`
- Keep functions focused and small
- Prefer explicit error handling over panics
- Maintain `no_std` compatibility - only use `alloc`, never `std`
