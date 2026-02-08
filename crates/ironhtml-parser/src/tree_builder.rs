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
    /// Stack of open element indices (used by `navigate_to_element`).
    open_elements: Vec<usize>,
    /// Parallel stack of open element tag names (for `pop_until`).
    open_element_names: Vec<String>,
    /// Whether we're in fragment mode.
    fragment_mode: bool,
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

/// Navigate from the document root to the element described by `path`.
///
/// `path[0]` is the root sentinel (skipped); subsequent entries are
/// child indices at each nesting level.
fn navigate_to_element<'a>(root: &'a mut Element, path: &[usize]) -> &'a mut Element {
    let mut current = root;

    for &idx in path.iter().skip(1) {
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

impl TreeBuilder {
    /// Create a new tree builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            open_elements: Vec::new(),
            open_element_names: Vec::new(),
            fragment_mode: false,
            insertion_mode: InsertionMode::Initial,
            pending_text: String::new(),
        }
    }

    /// Set fragment mode (for parsing HTML fragments).
    ///
    /// In fragment mode the tree builder skips implicit `<html>`,
    /// `<head>`, and `<body>` creation and inserts directly into
    /// `document.root` which acts as a virtual container.
    pub fn set_fragment_mode(&mut self, fragment: bool) {
        self.fragment_mode = fragment;
        if fragment {
            self.insertion_mode = InsertionMode::InBody;
            // Push a sentinel so navigate_to_element starts from root.
            self.open_elements.push(0);
            self.open_element_names.push(String::new());
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
        self.insert_into_current(comment);
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
        self.open_element_names.push(String::from("html"));
    }

    fn insert_element(&mut self, tag_name: &str, attributes: Vec<(String, String)>) {
        let mut element = Element::new(tag_name);
        for (key, value) in attributes {
            element.attributes.push(Attribute::new(key, value));
        }

        let node = Node::Element(element);
        let idx = self.insert_into_current(node);
        self.open_elements.push(idx);
        self.open_element_names.push(String::from(tag_name));
    }

    fn insert_into_current(&mut self, node: Node) -> usize {
        let parent = navigate_to_element(&mut self.document.root, &self.open_elements);
        let idx = parent.children.len();
        parent.children.push(node);
        idx
    }

    fn insert_text(&mut self, text: String) {
        let text_node = Node::Text(Text::new(text));
        self.insert_into_current(text_node);
    }

    fn pop_element(&mut self) {
        self.open_elements.pop();
        self.open_element_names.pop();
    }

    /// Pop elements from the stack until one matching `tag_name` is found
    /// and popped. Never pops the root sentinel.
    fn pop_until(&mut self, tag_name: &str) {
        while self.open_element_names.len() > 1 {
            if self.open_element_names.last().map(String::as_str) == Some(tag_name) {
                self.open_elements.pop();
                self.open_element_names.pop();
                return;
            }
            self.open_elements.pop();
            self.open_element_names.pop();
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
        self.document.root.children
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
            "<!DOCTYPE html><html><head><title>Test</title>\
             </head><body><p>Hello</p></body></html>",
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

    #[test]
    fn test_pop_until_nested() {
        let nodes = parse_fragment("<div><span>Hello</span> World</div>");
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            assert_eq!(div.tag_name, "div");
            assert_eq!(div.children.len(), 2);
            if let Some(Node::Element(span)) = div.children.first() {
                assert_eq!(span.tag_name, "span");
                assert_eq!(span.text_content(), Some("Hello".into()));
            }
        }
    }

    #[test]
    fn test_deeply_nested_fragment() {
        let nodes = parse_fragment("<div><ul><li><span>Deep</span></li></ul></div>");
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            let ul = div.find_element("ul").unwrap();
            let li = ul.find_element("li").unwrap();
            let span = li.find_element("span").unwrap();
            assert_eq!(span.text_content(), Some("Deep".into()));
        }
    }

    #[test]
    fn test_fragment_void_elements() {
        let nodes = parse_fragment("<div><br><span>After</span></div>");
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            // br is void, should not nest span inside it
            assert_eq!(div.children.len(), 2);
            if let Some(Node::Element(br)) = div.children.first() {
                assert_eq!(br.tag_name, "br");
                assert!(br.children.is_empty());
            }
        }
    }

    #[test]
    fn test_fragment_multiple_top_level() {
        let nodes = parse_fragment("<p>One</p><p>Two</p><p>Three</p>");
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_many_children_fragment() {
        use core::fmt::Write;
        let mut html = String::from("<div>");
        for i in 0..1100 {
            let _ = write!(html, "<span>{i}</span>");
        }
        html.push_str("</div>");
        let nodes = parse_fragment(&html);
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            assert_eq!(div.children.len(), 1100);
        }
    }

    #[test]
    fn test_unmatched_end_tag() {
        // Unmatched </span> should not crash or empty the stack
        let nodes = parse_fragment("<div>Hello</span></div>");
        assert_eq!(nodes.len(), 1);
        if let Some(Node::Element(div)) = nodes.first() {
            assert_eq!(div.tag_name, "div");
        }
    }
}
