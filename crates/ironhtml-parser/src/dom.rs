//! DOM types for representing parsed HTML.
//!
//! This module provides types that represent the Document Object Model (DOM)
//! as defined by the WHATWG DOM Standard.
//!
//! ## Reference
//!
//! - [DOM Standard](https://dom.spec.whatwg.org/)
//! - [Node interface](https://dom.spec.whatwg.org/#interface-node)

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

/// The type of a DOM node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// An element node (e.g., `<div>`, `<span>`).
    Element,
    /// A text node containing character data.
    Text,
    /// A comment node (e.g., `<!-- comment -->`).
    Comment,
    /// A document type node (e.g., `<!DOCTYPE html>`).
    DocumentType,
    /// The document root node.
    Document,
}

/// An HTML document.
#[derive(Debug, Clone)]
pub struct Document {
    /// The document type declaration, if present.
    pub doctype: Option<DocumentType>,
    /// The root element (usually `<html>`).
    pub root: Element,
}

impl Document {
    /// Create a new empty document.
    #[must_use]
    pub fn new() -> Self {
        Self {
            doctype: None,
            root: Element::new("html"),
        }
    }

    /// Get the document's title from `<head><title>`.
    #[must_use]
    pub fn title(&self) -> Option<String> {
        self.root
            .find_element("head")
            .and_then(|head| head.find_element("title"))
            .and_then(Element::text_content)
            .map(Cow::into_owned)
    }

    /// Get the body element.
    #[must_use]
    pub fn body(&self) -> Option<&Element> {
        self.root.find_element("body")
    }

    /// Get the head element.
    #[must_use]
    pub fn head(&self) -> Option<&Element> {
        self.root.find_element("head")
    }

    /// Render the document back to an HTML string.
    #[must_use]
    pub fn to_html(&self) -> String {
        let mut output = String::new();
        if let Some(doctype) = &self.doctype {
            output.push_str("<!DOCTYPE ");
            output.push_str(&doctype.name);
            output.push('>');
        }
        self.root.render_to(&mut output);
        output
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// A document type declaration.
#[derive(Debug, Clone)]
pub struct DocumentType {
    /// The document type name (usually "html").
    pub name: String,
    /// The public identifier, if any.
    pub public_id: Option<String>,
    /// The system identifier, if any.
    pub system_id: Option<String>,
}

impl DocumentType {
    /// Create a new HTML5 doctype.
    pub fn html5() -> Self {
        Self {
            name: String::from("html"),
            public_id: None,
            system_id: None,
        }
    }
}

/// A node in the DOM tree.
#[derive(Debug, Clone)]
pub enum Node {
    /// An element node.
    Element(Element),
    /// A text node.
    Text(Text),
    /// A comment node.
    Comment(Comment),
}

impl Node {
    /// Get the type of this node.
    #[must_use]
    pub const fn node_type(&self) -> NodeType {
        match self {
            Self::Element(_) => NodeType::Element,
            Self::Text(_) => NodeType::Text,
            Self::Comment(_) => NodeType::Comment,
        }
    }

    /// Get this node as an element, if it is one.
    #[must_use]
    pub const fn as_element(&self) -> Option<&Element> {
        match self {
            Self::Element(e) => Some(e),
            _ => None,
        }
    }

    /// Get this node as a mutable element, if it is one.
    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Self::Element(e) => Some(e),
            _ => None,
        }
    }

    /// Get this node as a text node, if it is one.
    #[must_use]
    pub const fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Text(t) => Some(t),
            _ => None,
        }
    }

    /// Render this node to an HTML string.
    #[must_use]
    pub fn to_html(&self) -> String {
        let mut output = String::new();
        self.render_to(&mut output);
        output
    }

    /// Render this node to an existing string buffer.
    pub fn render_to(&self, output: &mut String) {
        match self {
            Self::Element(e) => e.render_to(output),
            Self::Text(t) => output.push_str(&t.data),
            Self::Comment(c) => {
                output.push_str("<!--");
                output.push_str(&c.data);
                output.push_str("-->");
            }
        }
    }
}

/// An HTML element.
#[derive(Debug, Clone)]
pub struct Element {
    /// The tag name (lowercase).
    pub tag_name: String,
    /// The element's attributes.
    pub attributes: Vec<Attribute>,
    /// The element's child nodes.
    pub children: Vec<Node>,
}

impl Element {
    /// Create a new element with the given tag name.
    pub fn new(tag_name: impl Into<String>) -> Self {
        Self {
            tag_name: tag_name.into().to_ascii_lowercase(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Check if this is a void element (self-closing).
    #[must_use]
    pub fn is_void(&self) -> bool {
        matches!(
            self.tag_name.as_str(),
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
        )
    }

    /// Get an attribute value by name.
    #[must_use]
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|a| a.name.eq_ignore_ascii_case(name))
            .map(|a| a.value.as_str())
    }

    /// Check if the element has an attribute.
    #[must_use]
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes
            .iter()
            .any(|a| a.name.eq_ignore_ascii_case(name))
    }

    /// Set an attribute value.
    pub fn set_attribute(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name = name.into();
        let value = value.into();
        if let Some(attr) = self
            .attributes
            .iter_mut()
            .find(|a| a.name.eq_ignore_ascii_case(&name))
        {
            attr.value = value;
        } else {
            self.attributes.push(Attribute { name, value });
        }
    }

    /// Get the element's id attribute.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.get_attribute("id")
    }

    /// Get the element's class attribute.
    #[must_use]
    pub fn class(&self) -> Option<&str> {
        self.get_attribute("class")
    }

    /// Get the text content of this element and its descendants.
    #[must_use]
    pub fn text_content(&self) -> Option<Cow<'_, str>> {
        let mut text = String::new();
        self.collect_text(&mut text);
        if text.is_empty() {
            None
        } else {
            Some(Cow::Owned(text))
        }
    }

    fn collect_text(&self, output: &mut String) {
        for child in &self.children {
            match child {
                Node::Text(t) => output.push_str(&t.data),
                Node::Element(e) => e.collect_text(output),
                Node::Comment(_) => {}
            }
        }
    }

    /// Find the first descendant element with the given tag name.
    #[must_use]
    pub fn find_element(&self, tag_name: &str) -> Option<&Self> {
        for child in &self.children {
            if let Node::Element(e) = child {
                if e.tag_name.eq_ignore_ascii_case(tag_name) {
                    return Some(e);
                }
                if let Some(found) = e.find_element(tag_name) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find all descendant elements with the given tag name.
    #[must_use]
    pub fn find_all_elements(&self, tag_name: &str) -> Vec<&Self> {
        let mut results = Vec::new();
        self.collect_elements(tag_name, &mut results);
        results
    }

    fn collect_elements<'a>(&'a self, tag_name: &str, results: &mut Vec<&'a Self>) {
        for child in &self.children {
            if let Node::Element(e) = child {
                if e.tag_name.eq_ignore_ascii_case(tag_name) {
                    results.push(e);
                }
                e.collect_elements(tag_name, results);
            }
        }
    }

    /// Find elements by class name.
    #[must_use]
    pub fn find_by_class(&self, class_name: &str) -> Vec<&Self> {
        let mut results = Vec::new();
        self.collect_by_class(class_name, &mut results);
        results
    }

    fn collect_by_class<'a>(&'a self, class_name: &str, results: &mut Vec<&'a Self>) {
        if let Some(class) = self.class() {
            if class.split_whitespace().any(|c| c == class_name) {
                results.push(self);
            }
        }
        for child in &self.children {
            if let Node::Element(e) = child {
                e.collect_by_class(class_name, results);
            }
        }
    }

    /// Find element by id.
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&Self> {
        if self.id() == Some(id) {
            return Some(self);
        }
        for child in &self.children {
            if let Node::Element(e) = child {
                if let Some(found) = e.find_by_id(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Render this element to an HTML string.
    #[must_use]
    pub fn to_html(&self) -> String {
        let mut output = String::new();
        self.render_to(&mut output);
        output
    }

    /// Render this element to an existing string buffer.
    pub fn render_to(&self, output: &mut String) {
        output.push('<');
        output.push_str(&self.tag_name);

        for attr in &self.attributes {
            output.push(' ');
            output.push_str(&attr.name);
            if !attr.value.is_empty() {
                output.push_str("=\"");
                // Escape attribute value
                for c in attr.value.chars() {
                    match c {
                        '&' => output.push_str("&amp;"),
                        '"' => output.push_str("&quot;"),
                        '\'' => output.push_str("&#x27;"),
                        '<' => output.push_str("&lt;"),
                        '>' => output.push_str("&gt;"),
                        _ => output.push(c),
                    }
                }
                output.push('"');
            }
        }

        if self.is_void() {
            output.push_str(" />");
        } else {
            output.push('>');
            for child in &self.children {
                child.render_to(output);
            }
            output.push_str("</");
            output.push_str(&self.tag_name);
            output.push('>');
        }
    }
}

/// An attribute on an element.
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The attribute name.
    pub name: String,
    /// The attribute value.
    pub value: String,
}

impl Attribute {
    /// Create a new attribute.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// A text node.
#[derive(Debug, Clone)]
pub struct Text {
    /// The text content.
    pub data: String,
}

impl Text {
    /// Create a new text node.
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

/// A comment node.
#[derive(Debug, Clone)]
pub struct Comment {
    /// The comment content.
    pub data: String,
}

impl Comment {
    /// Create a new comment node.
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_new() {
        let elem = Element::new("DIV");
        assert_eq!(elem.tag_name, "div");
    }

    #[test]
    fn test_element_attributes() {
        let mut elem = Element::new("div");
        elem.set_attribute("class", "container");
        elem.set_attribute("id", "main");

        assert_eq!(elem.get_attribute("class"), Some("container"));
        assert_eq!(elem.get_attribute("id"), Some("main"));
        assert_eq!(elem.class(), Some("container"));
        assert_eq!(elem.id(), Some("main"));
    }

    #[test]
    fn test_element_is_void() {
        assert!(Element::new("br").is_void());
        assert!(Element::new("img").is_void());
        assert!(Element::new("input").is_void());
        assert!(!Element::new("div").is_void());
        assert!(!Element::new("span").is_void());
    }

    #[test]
    fn test_element_text_content() {
        let mut elem = Element::new("p");
        elem.children.push(Node::Text(Text::new("Hello, ")));
        elem.children.push(Node::Text(Text::new("World!")));

        assert_eq!(
            elem.text_content(),
            Some(Cow::Owned("Hello, World!".into()))
        );
    }

    #[test]
    fn test_element_render() {
        let mut elem = Element::new("div");
        elem.set_attribute("class", "test");
        elem.children.push(Node::Text(Text::new("Hello")));

        assert_eq!(elem.to_html(), r#"<div class="test">Hello</div>"#);
    }

    #[test]
    fn test_void_element_render() {
        let mut elem = Element::new("img");
        elem.set_attribute("src", "test.jpg");

        assert_eq!(elem.to_html(), r#"<img src="test.jpg" />"#);
    }

    #[test]
    fn test_document() {
        let mut doc = Document::new();
        doc.doctype = Some(DocumentType::html5());

        let html = doc.to_html();
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn test_escape_single_quotes_in_attributes() {
        let mut elem = Element::new("div");
        elem.set_attribute("data-msg", "it's a test");

        assert_eq!(elem.to_html(), r#"<div data-msg="it&#x27;s a test"></div>"#);
    }

    #[test]
    fn test_escape_all_special_chars_in_attributes() {
        let mut elem = Element::new("div");
        elem.set_attribute("data-val", r#"a&b<c>d"e'f"#);

        assert_eq!(
            elem.to_html(),
            r#"<div data-val="a&amp;b&lt;c&gt;d&quot;e&#x27;f"></div>"#
        );
    }
}
