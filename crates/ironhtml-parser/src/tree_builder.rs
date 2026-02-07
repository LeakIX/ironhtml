//! HTML5 tree builder.
//!
//! This module implements the tree construction stage of HTML parsing.
//!
//! ## Reference
//!
//! - [Tree Construction](https://html.spec.whatwg.org/multipage/parsing.html#tree-construction)

use alloc::string::String;
use alloc::vec::Vec;

use crate::dom::{Attribute, Comment, Document, DocumentType, Element, Node, Text};
use crate::tokenizer::Token;

/// HTML5 tree builder.
///
/// Constructs a DOM tree from a stream of tokens.
pub struct TreeBuilder {
    /// The document being built.
    document: Document,
    /// Stack of open elements.
    open_elements: Vec<usize>,
    /// Whether we're in fragment mode.
    fragment_mode: bool,
    /// Fragment nodes (when in fragment mode).
    fragment_nodes: Vec<Node>,
    /// Current insertion mode.
    insertion_mode: InsertionMode,
    /// Pending text to be inserted.
    pending_text: String,
}

/// Insertion mode for the tree builder.
#[derive(Debug, Clone, Copy, PartialEq)]
enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    AfterBody,
    AfterAfterBody,
}

impl TreeBuilder {
    /// Create a new tree builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            open_elements: Vec::new(),
            fragment_mode: false,
            fragment_nodes: Vec::new(),
            insertion_mode: InsertionMode::Initial,
            pending_text: String::new(),
        }
    }

    /// Set fragment mode (for parsing HTML fragments).
    pub fn set_fragment_mode(&mut self, fragment: bool) {
        self.fragment_mode = fragment;
        if fragment {
            self.insertion_mode = InsertionMode::InBody;
        }
    }

    /// Process a token.
    pub fn process_token(&mut self, token: Token) {
        // Flush pending text before processing non-character tokens
        match &token {
            Token::Character(_) => {}
            _ => self.flush_pending_text(),
        }

        match token {
            Token::Doctype {
                name,
                public_id,
                system_id,
            } => {
                self.process_doctype(name, public_id, system_id);
            }
            Token::StartTag {
                name,
                attributes,
                self_closing,
            } => {
                self.process_start_tag(&name, attributes, self_closing);
            }
            Token::EndTag { name } => {
                self.process_end_tag(&name);
            }
            Token::Comment(data) => {
                self.process_comment(data);
            }
            Token::Character(c) => {
                self.pending_text.push(c);
            }
            Token::Eof => {
                self.flush_pending_text();
            }
        }
    }

    fn flush_pending_text(&mut self) {
        if self.pending_text.is_empty() {
            return;
        }

        let text = core::mem::take(&mut self.pending_text);

        // Skip whitespace-only text in certain modes
        if text.chars().all(|c| c.is_ascii_whitespace()) {
            match self.insertion_mode {
                InsertionMode::Initial
                | InsertionMode::BeforeHtml
                | InsertionMode::BeforeHead
                | InsertionMode::AfterHead
                | InsertionMode::AfterBody
                | InsertionMode::AfterAfterBody => return,
                _ => {}
            }
        }

        self.insert_text(text);
    }

    fn process_doctype(
        &mut self,
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
    ) {
        if self.insertion_mode == InsertionMode::Initial {
            self.document.doctype = Some(DocumentType {
                name: name.unwrap_or_default(),
                public_id,
                system_id,
            });
            self.insertion_mode = InsertionMode::BeforeHtml;
        }
    }

    #[allow(clippy::too_many_lines, clippy::only_used_in_recursion)]
    fn process_start_tag(
        &mut self,
        name: &str,
        attributes: Vec<(String, String)>,
        self_closing: bool,
    ) {
        let name_lower = name.to_ascii_lowercase();

        match self.insertion_mode {
            InsertionMode::Initial => {
                // Implicitly create doctype and html
                self.insertion_mode = InsertionMode::BeforeHtml;
                self.process_start_tag(name, attributes, self_closing);
            }

            InsertionMode::BeforeHtml => {
                if name_lower == "html" {
                    self.create_html_element(attributes);
                    self.insertion_mode = InsertionMode::BeforeHead;
                } else {
                    // Implicitly create html element
                    self.create_html_element(Vec::new());
                    self.insertion_mode = InsertionMode::BeforeHead;
                    self.process_start_tag(name, attributes, self_closing);
                }
            }

            InsertionMode::BeforeHead => {
                if name_lower == "head" {
                    self.insert_element(&name_lower, attributes);
                    self.insertion_mode = InsertionMode::InHead;
                } else if name_lower == "html" {
                    // Merge attributes with existing html element
                    for (key, value) in attributes {
                        self.document.root.set_attribute(key, value);
                    }
                } else {
                    // Implicitly create head element
                    self.insert_element("head", Vec::new());
                    self.insertion_mode = InsertionMode::InHead;
                    self.process_start_tag(name, attributes, self_closing);
                }
            }

            InsertionMode::InHead => {
                match name_lower.as_str() {
                    "meta" | "link" | "base" => {
                        self.insert_element(&name_lower, attributes);
                        self.pop_element();
                    }
                    "title" | "style" | "script" | "noscript" => {
                        self.insert_element(&name_lower, attributes);
                    }
                    "head" => {
                        // Ignore duplicate head
                    }
                    "body" => {
                        self.pop_element(); // pop head
                        self.insertion_mode = InsertionMode::AfterHead;
                        self.process_start_tag(name, attributes, self_closing);
                    }
                    _ => {
                        // Implicitly close head and switch to after head
                        self.pop_element();
                        self.insertion_mode = InsertionMode::AfterHead;
                        self.process_start_tag(name, attributes, self_closing);
                    }
                }
            }

            InsertionMode::AfterHead => {
                if name_lower == "body" {
                    self.insert_element(&name_lower, attributes);
                    self.insertion_mode = InsertionMode::InBody;
                } else if name_lower == "html" {
                    // Merge attributes
                    for (key, value) in attributes {
                        self.document.root.set_attribute(key, value);
                    }
                } else {
                    // Implicitly create body
                    self.insert_element("body", Vec::new());
                    self.insertion_mode = InsertionMode::InBody;
                    self.process_start_tag(name, attributes, self_closing);
                }
            }

            InsertionMode::InBody => {
                if self.fragment_mode && self.open_elements.is_empty() {
                    // In fragment mode, just add to fragment nodes
                    let mut element = Element::new(&name_lower);
                    for (key, value) in attributes {
                        element.attributes.push(Attribute::new(key, value));
                    }
                    self.fragment_nodes.push(Node::Element(element));
                    self.open_elements.push(self.fragment_nodes.len() - 1);
                } else {
                    // Check for void elements
                    let is_void = matches!(
                        name_lower.as_str(),
                        "area"
                            | "base"
                            | "br"
                            | "col"
                            | "embed"
                            | "hr"
                            | "img"
                            | "input"
                            | "link"
                            | "meta"
                            | "source"
                            | "track"
                            | "wbr"
                    );

                    self.insert_element(&name_lower, attributes);

                    if is_void {
                        self.pop_element();
                    }
                }
            }

            InsertionMode::AfterBody => {
                if name_lower == "html" {
                    // Merge attributes
                    for (key, value) in attributes {
                        self.document.root.set_attribute(key, value);
                    }
                } else {
                    self.insertion_mode = InsertionMode::InBody;
                    self.process_start_tag(name, attributes, self_closing);
                }
            }

            InsertionMode::AfterAfterBody => {
                self.insertion_mode = InsertionMode::InBody;
                self.process_start_tag(name, attributes, self_closing);
            }
        }
    }

    fn process_end_tag(&mut self, name: &str) {
        let name_lower = name.to_ascii_lowercase();

        match self.insertion_mode {
            InsertionMode::InHead => {
                if name_lower == "head" {
                    self.pop_element();
                    self.insertion_mode = InsertionMode::AfterHead;
                }
            }

            InsertionMode::InBody => {
                if name_lower == "body" || name_lower == "html" {
                    self.insertion_mode = InsertionMode::AfterBody;
                } else {
                    // Pop elements until we find the matching start tag
                    self.pop_until(&name_lower);
                }
            }

            InsertionMode::AfterBody => {
                if name_lower == "html" {
                    self.insertion_mode = InsertionMode::AfterAfterBody;
                }
            }

            _ => {}
        }
    }

    fn process_comment(&mut self, data: String) {
        let comment = Node::Comment(Comment::new(data));

        if self.fragment_mode && self.open_elements.is_empty() {
            self.fragment_nodes.push(comment);
        } else if let Some(&idx) = self.open_elements.last() {
            if self.fragment_mode {
                if let Some(Node::Element(elem)) = self.fragment_nodes.get_mut(idx) {
                    elem.children.push(comment);
                }
            } else {
                self.insert_into_current(comment);
            }
        }
    }

    fn create_html_element(&mut self, attributes: Vec<(String, String)>) {
        self.document.root = Element::new("html");
        for (key, value) in attributes {
            self.document
                .root
                .attributes
                .push(Attribute::new(key, value));
        }
        self.open_elements.push(0); // html is always index 0
    }

    fn insert_element(&mut self, tag_name: &str, attributes: Vec<(String, String)>) {
        let mut element = Element::new(tag_name);
        for (key, value) in attributes {
            element.attributes.push(Attribute::new(key, value));
        }

        if self.fragment_mode {
            if self.open_elements.is_empty() {
                let idx = self.fragment_nodes.len();
                self.fragment_nodes.push(Node::Element(element));
                self.open_elements.push(idx);
            } else {
                let parent_idx = *self.open_elements.last().unwrap();
                if let Some(Node::Element(parent)) = self.fragment_nodes.get_mut(parent_idx) {
                    let child_idx = parent.children.len();
                    parent.children.push(Node::Element(element));
                    // Track using a simple index scheme
                    self.open_elements.push(parent_idx * 1000 + child_idx);
                }
            }
        } else {
            let node = Node::Element(element);
            let idx = self.insert_into_current(node);
            self.open_elements.push(idx);
        }
    }

    fn insert_into_current(&mut self, node: Node) -> usize {
        // Get the indices for navigation
        let path: Vec<usize> = self.open_elements.clone();

        // Navigate to the correct parent element using indices
        let parent = self.navigate_to_element(&path);
        let idx = parent.children.len();
        parent.children.push(node);
        idx
    }

    fn navigate_to_element(&mut self, path: &[usize]) -> &mut Element {
        let indices: Vec<usize> = path.iter().skip(1).copied().collect();
        let mut current = &mut self.document.root;

        for idx in indices {
            if idx < current.children.len() && matches!(current.children[idx], Node::Element(_)) {
                current = match &mut current.children[idx] {
                    Node::Element(elem) => elem,
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }

        current
    }

    fn insert_text(&mut self, text: String) {
        let text_node = Node::Text(Text::new(text));

        if self.fragment_mode {
            if self.open_elements.is_empty() {
                self.fragment_nodes.push(text_node);
            } else {
                let parent_idx = *self.open_elements.last().unwrap();
                if parent_idx < self.fragment_nodes.len() {
                    if let Some(Node::Element(parent)) = self.fragment_nodes.get_mut(parent_idx) {
                        parent.children.push(text_node);
                    }
                }
            }
        } else {
            self.insert_into_current(text_node);
        }
    }

    fn pop_element(&mut self) {
        self.open_elements.pop();
    }

    fn pop_until(&mut self, _tag_name: &str) {
        // In a full implementation, we'd pop until we find an element matching tag_name.
        // For simplicity, we just pop the current element once.
        if self.open_elements.last().is_some() {
            self.open_elements.pop();
        }
    }

    /// Finish building and return the document.
    #[must_use]
    pub fn finish(mut self) -> Document {
        self.flush_pending_text();
        self.document
    }

    /// Finish building and return fragment nodes.
    #[must_use]
    pub fn finish_fragment(mut self) -> Vec<Node> {
        self.flush_pending_text();
        self.fragment_nodes
    }
}

impl Default for TreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    fn parse(html: &str) -> Document {
        let tokenizer = Tokenizer::new(html);
        let mut builder = TreeBuilder::new();
        for token in tokenizer {
            builder.process_token(token);
        }
        builder.finish()
    }

    fn parse_fragment(html: &str) -> Vec<Node> {
        let tokenizer = Tokenizer::new(html);
        let mut builder = TreeBuilder::new();
        builder.set_fragment_mode(true);
        for token in tokenizer {
            builder.process_token(token);
        }
        builder.finish_fragment()
    }

    #[test]
    fn test_simple_document() {
        let doc = parse(
            "<!DOCTYPE html><html><head><title>Test</title></head><body><p>Hello</p></body></html>",
        );
        assert!(doc.doctype.is_some());
        assert_eq!(doc.doctype.as_ref().unwrap().name, "html");
        assert_eq!(doc.root.tag_name, "html");
    }

    #[test]
    fn test_implicit_html() {
        let doc = parse("<p>Hello</p>");
        assert_eq!(doc.root.tag_name, "html");
        assert!(doc.body().is_some());
    }

    #[test]
    fn test_fragment() {
        let nodes = parse_fragment("<div><span>Hello</span></div>");
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            assert_eq!(div.tag_name, "div");
        }
    }

    #[test]
    fn test_text_content() {
        let doc = parse("<p>Hello World</p>");
        if let Some(body) = doc.body() {
            if let Some(p) = body.find_element("p") {
                assert_eq!(p.text_content(), Some("Hello World".into()));
            }
        }
    }

    #[test]
    fn test_attributes() {
        let nodes = parse_fragment(r#"<div class="container" id="main"></div>"#);
        if let Some(Node::Element(div)) = nodes.first() {
            assert_eq!(div.get_attribute("class"), Some("container"));
            assert_eq!(div.get_attribute("id"), Some("main"));
        }
    }
}
