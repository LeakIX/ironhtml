//! # ironhtml-parser
//!
//! HTML5 parser following the [WHATWG HTML Living Standard](https://html.spec.whatwg.org/).
//!
//! This crate provides a parser that converts HTML strings into a DOM tree,
//! handling malformed HTML gracefully like browsers do.
//!
//! ## Features
//!
//! - Parse HTML5 documents and fragments
//! - Handles malformed HTML gracefully
//! - `no_std` compatible (with alloc)
//! - Produces a DOM tree that can be traversed and validated
//!
//! ## Example
//!
//! ```rust
//! use ironhtml_parser::{parse, parse_fragment, Node};
//!
//! // Parse a complete document
//! let doc = parse("<!DOCTYPE html><html><body><p>Hello</p></body></html>");
//! assert!(doc.doctype.is_some());
//!
//! // Parse a fragment
//! let nodes = parse_fragment("<div class=\"container\"><span>Text</span></div>");
//! assert_eq!(nodes.len(), 1);
//! ```
//!
//! ## Specification Reference
//!
//! - [HTML Parsing](https://html.spec.whatwg.org/multipage/parsing.html)
//! - [Tokenization](https://html.spec.whatwg.org/multipage/parsing.html#tokenization)
//! - [Tree Construction](https://html.spec.whatwg.org/multipage/parsing.html#tree-construction)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod dom;
mod entities;
mod tokenizer;
mod tree_builder;
mod validator;

pub use dom::{Attribute, Document, Element, Node, NodeType, Text};
pub use tokenizer::{Token, Tokenizer};
pub use tree_builder::TreeBuilder;
pub use validator::{ValidationError, ValidationResult, Validator};

use alloc::vec::Vec;

/// Parse an HTML document string into a Document.
///
/// This handles the full document including doctype, html, head, and body elements.
///
/// ## Example
///
/// ```rust
/// use ironhtml_parser::parse;
///
/// let doc = parse("<!DOCTYPE html><html><head><title>Test</title></head><body><p>Hello</p></body></html>");
/// assert!(doc.doctype.is_some());
/// assert!(!doc.root.children.is_empty());
/// ```
#[must_use]
pub fn parse(html: &str) -> Document {
    let tokenizer = Tokenizer::new(html);
    let mut builder = TreeBuilder::new();

    for token in tokenizer {
        builder.process_token(token);
    }

    builder.finish()
}

/// Parse an HTML fragment string into a list of nodes.
///
/// This is useful for parsing partial HTML content like template snippets.
///
/// ## Example
///
/// ```rust
/// use ironhtml_parser::{parse_fragment, NodeType};
///
/// let nodes = parse_fragment("<div><span>Hello</span></div>");
/// assert_eq!(nodes.len(), 1);
/// assert_eq!(nodes[0].node_type(), NodeType::Element);
/// ```
#[must_use]
pub fn parse_fragment(html: &str) -> Vec<Node> {
    let tokenizer = Tokenizer::new(html);
    let mut builder = TreeBuilder::new();
    builder.set_fragment_mode(true);

    for token in tokenizer {
        builder.process_token(token);
    }

    builder.finish_fragment()
}

/// Validate an HTML document and return any errors.
///
/// ## Example
///
/// ```rust
/// use ironhtml_parser::{parse, validate};
///
/// let doc = parse("<img>");
/// let errors = validate(&doc);
/// // img without alt attribute would be a validation error
/// assert!(!errors.is_empty());
/// ```
#[must_use]
pub fn validate(doc: &Document) -> Vec<ValidationError> {
    let validator = Validator::new();
    validator.validate(doc)
}

/// Validate an HTML fragment and return any errors.
#[must_use]
pub fn validate_fragment(nodes: &[Node]) -> Vec<ValidationError> {
    let validator = Validator::new();
    validator.validate_nodes(nodes)
}
