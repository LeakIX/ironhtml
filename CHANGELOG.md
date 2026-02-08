# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [1.0.0] - 2026-02-07

### Added

- Initial release as `ironhtml` (renamed from `html-builder`)
- Core HTML builder with type-safe element construction and XSS protection
  (`ironhtml`)
- All 110+ HTML5 elements as zero-sized types with content category traits
  (`ironhtml-elements`)
- Typed attributes (global + element-specific) with validation
  (`ironhtml-attributes`)
- Proc-macro for ergonomic HTML generation (`ironhtml-macro`)
- HTML5 parser and validator following WHATWG spec (`ironhtml-parser`)
- Type-safe Bootstrap 5.3 component library (`ironhtml-bootstrap`)
- Strict clippy lints: `clippy::all`, `clippy::pedantic`, `clippy::nursery`
  enforced at workspace level
- `no_std` support across all crates (only uses `alloc`)
- WebAssembly build target support
- Automatic XSS protection via HTML/attribute escaping
- parse5 integration tests for HTML5 spec compliance

[Unreleased]: https://github.com/LeakIX/ironhtml/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/LeakIX/ironhtml/releases/tag/v1.0.0
