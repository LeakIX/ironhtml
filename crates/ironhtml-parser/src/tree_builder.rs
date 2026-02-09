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

    // ── pop_until tests ──────────────────────────────────────────────

    #[test]
    fn test_pop_until_skips_intermediate() {
        // </div> should pop both <em> and <span>, closing at <div>
        let nodes = parse_fragment("<div><span><em>Text</div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.tag_name, "div");
        // span was opened, em was opened inside span, then </div>
        // pops em, span, div
        let span = div.find_element("span").unwrap();
        let em = span.find_element("em").unwrap();
        assert_eq!(em.text_content(), Some("Text".into()));
    }

    #[test]
    fn test_pop_until_no_match_preserves_root() {
        // </nonexistent> pops elements but stops at root sentinel
        let nodes = parse_fragment("<div><p>Hello</p></nonexistent></div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.tag_name, "div");
    }

    #[test]
    fn test_pop_until_closes_correct_level() {
        // Nested <div>s: inner </div> should only close the inner one
        let nodes = parse_fragment("<div><div><span>Inner</span></div><span>Outer</span></div>");
        assert_eq!(nodes.len(), 1);
        let outer = nodes[0].as_element().unwrap();
        assert_eq!(outer.tag_name, "div");
        assert_eq!(outer.children.len(), 2);
        // First child: inner div
        let inner = outer.children[0].as_element().unwrap();
        assert_eq!(inner.tag_name, "div");
        assert_eq!(
            inner.find_element("span").unwrap().text_content(),
            Some("Inner".into())
        );
        // Second child: outer span (after inner div was closed)
        let outer_span = outer.children[1].as_element().unwrap();
        assert_eq!(outer_span.tag_name, "span");
        assert_eq!(outer_span.text_content(), Some("Outer".into()));
    }

    #[test]
    fn test_pop_until_multiple_same_tag() {
        // Three nested <span>s, one </span> closes only the innermost
        let nodes = parse_fragment("<div><span><span><span>Deep</span>Mid</span>Top</span></div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        let s1 = div.find_element("span").unwrap();
        let s2 = s1.find_element("span").unwrap();
        let s3 = s2.find_element("span").unwrap();
        assert_eq!(s3.text_content(), Some("Deep".into()));
        // "Mid" is text after inner span closes, inside middle span
        assert!(s2.children.len() >= 2);
        // "Top" is text after middle span closes, inside outer span
        assert!(s1.children.len() >= 2);
    }

    // ── fragment nesting tests ───────────────────────────────────────

    #[test]
    fn test_fragment_five_levels_deep() {
        let nodes = parse_fragment(
            "<div><section><article><header><h1>Title</h1>\
             </header></article></section></div>",
        );
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        let section = div.find_element("section").unwrap();
        let article = section.find_element("article").unwrap();
        let header = article.find_element("header").unwrap();
        let h1 = header.find_element("h1").unwrap();
        assert_eq!(h1.text_content(), Some("Title".into()));
    }

    #[test]
    fn test_fragment_text_at_every_level() {
        let nodes = parse_fragment("<div>A<span>B<em>C</em>D</span>E</div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        // div has: text("A"), span, text("E")
        assert_eq!(div.children.len(), 3);
        assert_eq!(div.children[0].as_text().unwrap().data, "A");
        let span = div.children[1].as_element().unwrap();
        // span has: text("B"), em, text("D")
        assert_eq!(span.children.len(), 3);
        assert_eq!(span.children[0].as_text().unwrap().data, "B");
        let em = span.children[1].as_element().unwrap();
        assert_eq!(em.text_content(), Some("C".into()));
        assert_eq!(span.children[2].as_text().unwrap().data, "D");
        assert_eq!(div.children[2].as_text().unwrap().data, "E");
    }

    #[test]
    fn test_fragment_siblings_with_children() {
        let nodes = parse_fragment("<ul><li>One<em>!</em></li><li>Two</li><li>Three</li></ul>");
        assert_eq!(nodes.len(), 1);
        let ul = nodes[0].as_element().unwrap();
        assert_eq!(ul.children.len(), 3);
        // First li has text + em
        let li1 = ul.children[0].as_element().unwrap();
        assert_eq!(li1.children.len(), 2);
        assert_eq!(li1.children[0].as_text().unwrap().data, "One");
        assert_eq!(
            li1.children[1].as_element().unwrap().text_content(),
            Some("!".into())
        );
        // Second and third are simple
        let li2 = ul.children[1].as_element().unwrap();
        assert_eq!(li2.text_content(), Some("Two".into()));
        let li3 = ul.children[2].as_element().unwrap();
        assert_eq!(li3.text_content(), Some("Three".into()));
    }

    // ── fragment void element tests ──────────────────────────────────

    #[test]
    fn test_fragment_multiple_void_elements() {
        let nodes = parse_fragment("<div><br><hr><img><input></div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.children.len(), 4);
        assert_eq!(div.children[0].as_element().unwrap().tag_name, "br");
        assert_eq!(div.children[1].as_element().unwrap().tag_name, "hr");
        assert_eq!(div.children[2].as_element().unwrap().tag_name, "img");
        assert_eq!(div.children[3].as_element().unwrap().tag_name, "input");
        // None should have children
        for child in &div.children {
            assert!(child.as_element().unwrap().children.is_empty());
        }
    }

    #[test]
    fn test_fragment_void_between_text() {
        let nodes = parse_fragment("<p>Before<br>After</p>");
        assert_eq!(nodes.len(), 1);
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.children.len(), 3);
        assert_eq!(p.children[0].as_text().unwrap().data, "Before");
        assert_eq!(p.children[1].as_element().unwrap().tag_name, "br");
        assert_eq!(p.children[2].as_text().unwrap().data, "After");
    }

    #[test]
    fn test_fragment_void_with_attributes() {
        let nodes = parse_fragment(r#"<div><img src="a.png" alt="test"><br></div>"#);
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        let img = div.children[0].as_element().unwrap();
        assert_eq!(img.get_attribute("src"), Some("a.png"));
        assert_eq!(img.get_attribute("alt"), Some("test"));
        assert!(img.children.is_empty());
    }

    #[test]
    fn test_fragment_void_nested_inside() {
        // Void element inside a nested structure
        let nodes = parse_fragment("<table><tr><td><input></td></tr></table>");
        assert_eq!(nodes.len(), 1);
        let table = nodes[0].as_element().unwrap();
        let tr = table.find_element("tr").unwrap();
        let td = tr.find_element("td").unwrap();
        let input = td.find_element("input").unwrap();
        assert!(input.children.is_empty());
    }

    // ── fragment comment tests ───────────────────────────────────────

    #[test]
    fn test_fragment_comment_top_level() {
        let nodes = parse_fragment("<!-- top --><div>Hi</div>");
        assert_eq!(nodes.len(), 2);
        assert!(matches!(nodes[0], Node::Comment(_)));
        if let Node::Comment(c) = &nodes[0] {
            assert_eq!(c.data, " top ");
        }
        assert_eq!(nodes[1].as_element().unwrap().tag_name, "div");
    }

    #[test]
    fn test_fragment_comment_inside_element() {
        let nodes = parse_fragment("<div><!-- inside --></div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.children.len(), 1);
        assert!(matches!(div.children[0], Node::Comment(_)));
    }

    #[test]
    fn test_fragment_comment_between_elements() {
        let nodes = parse_fragment("<ul><li>A</li><!-- sep --><li>B</li></ul>");
        assert_eq!(nodes.len(), 1);
        let ul = nodes[0].as_element().unwrap();
        assert_eq!(ul.children.len(), 3);
        assert_eq!(ul.children[0].as_element().unwrap().tag_name, "li");
        assert!(matches!(ul.children[1], Node::Comment(_)));
        assert_eq!(ul.children[2].as_element().unwrap().tag_name, "li");
    }

    // ── fragment top-level variety tests ─────────────────────────────

    #[test]
    fn test_fragment_text_only() {
        let nodes = parse_fragment("Just text");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].as_text().unwrap().data, "Just text");
    }

    #[test]
    fn test_fragment_mixed_top_level() {
        let nodes = parse_fragment("Hello <em>world</em> and <strong>more</strong>!");
        // text, em, text, strong, text
        assert_eq!(nodes.len(), 5);
        assert_eq!(nodes[0].as_text().unwrap().data, "Hello ");
        assert_eq!(nodes[1].as_element().unwrap().tag_name, "em");
        assert_eq!(nodes[2].as_text().unwrap().data, " and ");
        assert_eq!(nodes[3].as_element().unwrap().tag_name, "strong");
        assert_eq!(nodes[4].as_text().unwrap().data, "!");
    }

    #[test]
    fn test_fragment_empty() {
        let nodes = parse_fragment("");
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_fragment_whitespace_only() {
        // Whitespace in InBody mode is NOT skipped
        let nodes = parse_fragment("   ");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].as_text().unwrap().data, "   ");
    }

    // ── malformed input tests ────────────────────────────────────────

    #[test]
    fn test_malformed_only_end_tags() {
        let nodes = parse_fragment("</div></span></p>");
        // No start tags to match, nothing produced
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_malformed_extra_end_tags() {
        let nodes = parse_fragment("<div>Hello</div></div></div></div>");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.text_content(), Some("Hello".into()));
    }

    #[test]
    fn test_malformed_unclosed_tags() {
        // Tags that are never closed
        let nodes = parse_fragment("<div><span><em>Text");
        assert_eq!(nodes.len(), 1);
        let div = nodes[0].as_element().unwrap();
        let span = div.find_element("span").unwrap();
        let em = span.find_element("em").unwrap();
        assert_eq!(em.text_content(), Some("Text".into()));
    }

    #[test]
    fn test_malformed_interleaved_tags() {
        // <b><i></b></i> - interleaved close order
        let nodes = parse_fragment("<b><i>Text</b>After</i>");
        // After </b> pops both i and b (pop_until finds b).
        // "After" goes to root since both are closed.
        // </i> is unmatched, ignored.
        assert!(!nodes.is_empty());
        let b = nodes[0].as_element().unwrap();
        assert_eq!(b.tag_name, "b");
    }

    #[test]
    fn test_malformed_deeply_mismatched() {
        let nodes = parse_fragment("<a><b><c><d><e>Text</a>");
        // </a> pops e, d, c, b, a
        assert_eq!(nodes.len(), 1);
        let a = nodes[0].as_element().unwrap();
        assert_eq!(a.tag_name, "a");
        assert!(a.find_element("e").is_some());
    }

    // ── full document tests ──────────────────────────────────────────

    #[test]
    fn test_document_head_elements() {
        let doc = parse(
            r#"<!DOCTYPE html><html><head>
            <title>Test</title>
            <meta charset="utf-8">
            <link rel="stylesheet" href="style.css">
            </head><body></body></html>"#,
        );
        let head = doc.head().unwrap();
        assert!(head.find_element("title").is_some());
        assert!(head.find_element("meta").is_some());
        assert!(head.find_element("link").is_some());
    }

    #[test]
    fn test_document_implicit_body() {
        // No explicit <body> tag, elements go into implicit body
        let doc = parse("<html><head></head><div>Content</div></html>");
        let body = doc.body().unwrap();
        let div = body.find_element("div").unwrap();
        assert_eq!(div.text_content(), Some("Content".into()));
    }

    #[test]
    fn test_document_implicit_head_and_body() {
        // No head or body, just content
        let doc = parse("<div>Content</div>");
        assert_eq!(doc.root.tag_name, "html");
        assert!(doc.head().is_some());
        assert!(doc.body().is_some());
        let body = doc.body().unwrap();
        let div = body.find_element("div").unwrap();
        assert_eq!(div.text_content(), Some("Content".into()));
    }

    #[test]
    fn test_document_title() {
        let doc = parse(
            "<!DOCTYPE html><html><head><title>Hello World</title></head>\
             <body></body></html>",
        );
        assert_eq!(doc.title(), Some(String::from("Hello World")));
    }

    #[test]
    fn test_document_round_trip() {
        let html = "<!DOCTYPE html><html><head><title>Test</title></head>\
                     <body><p>Hello</p></body></html>";
        let doc = parse(html);
        let output = doc.to_html();
        // Re-parse the output and verify structure
        let doc2 = parse(&output);
        assert_eq!(doc2.title(), Some(String::from("Test")));
        let body = doc2.body().unwrap();
        let p = body.find_element("p").unwrap();
        assert_eq!(p.text_content(), Some("Hello".into()));
    }

    // ── entity decoding integration tests ────────────────────────────

    #[test]
    fn test_entity_in_text_content() {
        let nodes = parse_fragment("<p>&amp; &lt; &gt;</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("& < >".into()));
    }

    #[test]
    fn test_entity_in_attribute_value() {
        let nodes = parse_fragment(r#"<a href="?a=1&amp;b=2">link</a>"#);
        let a = nodes[0].as_element().unwrap();
        assert_eq!(a.get_attribute("href"), Some("?a=1&b=2"));
    }

    #[test]
    fn test_numeric_entity_in_text() {
        let nodes = parse_fragment("<p>&#65;&#66;&#67;</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("ABC".into()));
    }

    #[test]
    fn test_hex_entity_in_text() {
        let nodes = parse_fragment("<p>&#x41;&#x42;&#x43;</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("ABC".into()));
    }

    #[test]
    fn test_nbsp_entity_in_text() {
        let nodes = parse_fragment("<p>hello&nbsp;world</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("hello\u{00A0}world".into()));
    }

    #[test]
    fn test_entity_mixed_with_tags() {
        let nodes = parse_fragment("<div>&lt;script&gt;alert(1)&lt;/script&gt;</div>");
        let div = nodes[0].as_element().unwrap();
        assert_eq!(div.text_content(), Some("<script>alert(1)</script>".into()));
    }

    #[test]
    fn test_unknown_entity_passthrough() {
        let nodes = parse_fragment("<p>&unknown;</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("&unknown;".into()));
    }

    #[test]
    fn test_bare_ampersand_passthrough() {
        let nodes = parse_fragment("<p>A & B</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("A & B".into()));
    }

    #[test]
    fn test_entity_copyright_symbol() {
        let nodes = parse_fragment("<p>&copy; 2024</p>");
        let p = nodes[0].as_element().unwrap();
        assert_eq!(p.text_content(), Some("\u{00A9} 2024".into()));
    }

    #[test]
    fn test_entity_in_title() {
        let doc = parse(
            "<!DOCTYPE html><html><head>\
             <title>A &amp; B</title>\
             </head><body></body></html>",
        );
        assert_eq!(doc.title(), Some(String::from("A & B")));
    }
}
