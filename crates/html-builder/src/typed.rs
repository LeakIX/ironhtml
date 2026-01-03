//! # Typed HTML Builder
//!
//! Type-safe HTML construction with compile-time validation of element
//! nesting following the [WHATWG HTML Living Standard](https://html.spec.whatwg.org/).
//!
//! ## Design
//!
//! The typed builder uses Rust's type system to enforce valid HTML structure:
//!
//! - **Generic elements**: `Element<E>` is parameterized by the element type
//! - **Compile-time validation**: The `CanContain` trait ensures valid parent-child
//!   relationships at compile time
//! - **Zero runtime overhead**: Element types are zero-sized, adding no cost
//!
//! ## Example
//!
//! ```rust
//! use html_builder::typed::{Element, TypedNode};
//! use html_elements::{Div, Span, P, Ul, Li, A, Text};
//!
//! // Build a navigation list
//! let nav = Element::<Ul>::new()
//!     .class("nav")
//!     .child::<Li, _>(|li| {
//!         li.child::<A, _>(|a| {
//!             a.attr("href", "/").text("Home")
//!         })
//!     })
//!     .child::<Li, _>(|li| {
//!         li.child::<A, _>(|a| {
//!             a.attr("href", "/about").text("About")
//!         })
//!     });
//!
//! let html = nav.render();
//! assert!(html.contains(r#"<ul class="nav">"#));
//! assert!(html.contains(r#"<a href="/">Home</a>"#));
//! ```
//!
//! ## Compile-Time Safety
//!
//! Invalid nesting produces a compilation error:
//!
//! ```rust,compile_fail
//! use html_builder::typed::Element;
//! use html_elements::{Ul, Div};
//!
//! // This fails to compile: Ul cannot contain Div
//! let invalid = Element::<Ul>::new()
//!     .child::<Div, _>(|d| d);
//! ```

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::marker::PhantomData;
use html_attributes::AttributeValue;
use html_elements::{CanContain, HtmlElement, Text};

use crate::{escape_attr, escape_html};

/// A node in the typed HTML tree.
#[derive(Debug, Clone)]
pub enum TypedNode {
    /// An element with tag, attributes, and children.
    Element {
        tag: &'static str,
        is_void: bool,
        attrs: Vec<(Cow<'static, str>, String)>,
        children: Vec<TypedNode>,
    },
    /// Escaped text content.
    Text(String),
    /// Raw HTML (not escaped).
    Raw(String),
}

impl TypedNode {
    /// Render this node to a string.
    pub fn render(&self) -> String {
        let mut output = String::new();
        self.render_to(&mut output);
        output
    }

    /// Render this node to an existing string buffer.
    pub fn render_to(&self, output: &mut String) {
        match self {
            TypedNode::Element {
                tag,
                is_void,
                attrs,
                children,
            } => {
                output.push('<');
                output.push_str(tag);

                for (name, value) in attrs {
                    output.push(' ');
                    output.push_str(name);
                    if !value.is_empty() {
                        output.push_str("=\"");
                        output.push_str(&escape_attr(value));
                        output.push('"');
                    }
                }

                if *is_void && children.is_empty() {
                    output.push_str(" />");
                } else {
                    output.push('>');

                    for child in children {
                        child.render_to(output);
                    }

                    output.push_str("</");
                    output.push_str(tag);
                    output.push('>');
                }
            }
            TypedNode::Text(text) => output.push_str(&escape_html(text)),
            TypedNode::Raw(html) => output.push_str(html),
        }
    }
}

/// A type-safe HTML element builder.
///
/// The type parameter `E` must implement [`HtmlElement`] and determines:
/// - The tag name (via `E::TAG`)
/// - Whether it's a void element (via `E::VOID`)
/// - Which children are allowed (via `CanContain<Child>` implementations)
#[derive(Debug, Clone)]
pub struct Element<E: HtmlElement> {
    attrs: Vec<(Cow<'static, str>, String)>,
    children: Vec<TypedNode>,
    _marker: PhantomData<E>,
}

impl<E: HtmlElement> Default for Element<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: HtmlElement> Element<E> {
    /// Create a new empty element.
    pub fn new() -> Self {
        Element {
            attrs: Vec::new(),
            children: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Add an attribute with a string value.
    pub fn attr(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    /// Add an attribute with a type-safe value.
    pub fn attr_value<V: AttributeValue>(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: V,
    ) -> Self {
        self.attrs.push((name.into(), value.to_attr_value().into()));
        self
    }

    /// Add a boolean attribute (no value, e.g., `disabled`, `checked`).
    pub fn bool_attr(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.attrs.push((name.into(), String::new()));
        self
    }

    /// Add a class. Multiple calls append to the class list.
    pub fn class(mut self, class: impl Into<String>) -> Self {
        let class = class.into();
        if let Some(pos) = self.attrs.iter().position(|(k, _)| k == "class") {
            self.attrs[pos].1.push(' ');
            self.attrs[pos].1.push_str(&class);
        } else {
            self.attrs.push((Cow::Borrowed("class"), class));
        }
        self
    }

    /// Add an id attribute.
    pub fn id(self, id: impl Into<String>) -> Self {
        self.attr("id", id)
    }

    /// Add a data-* attribute.
    pub fn data(self, name: &str, value: impl Into<String>) -> Self {
        let attr_name = alloc::format!("data-{}", name);
        self.attr(attr_name, value)
    }

    /// Add a child element.
    ///
    /// The child type must be allowed by the parent's content model.
    /// This is enforced at compile time via the `CanContain` trait.
    pub fn child<C, F>(mut self, f: F) -> Self
    where
        E: CanContain<C>,
        C: HtmlElement,
        F: FnOnce(Element<C>) -> Element<C>,
    {
        let child = f(Element::<C>::new());
        self.children.push(child.into_node());
        self
    }

    /// Add text content.
    ///
    /// Only available for elements that can contain text (via `CanContain<Text>`).
    pub fn text(mut self, content: impl Into<String>) -> Self
    where
        E: CanContain<Text>,
    {
        self.children.push(TypedNode::Text(content.into()));
        self
    }

    /// Add raw HTML content (not escaped).
    ///
    /// Use with caution - this bypasses XSS protection.
    pub fn raw(mut self, html: impl Into<String>) -> Self
    where
        E: CanContain<Text>,
    {
        self.children.push(TypedNode::Raw(html.into()));
        self
    }

    /// Add multiple children from an iterator.
    pub fn children<C, I, F>(mut self, items: I, f: F) -> Self
    where
        E: CanContain<C>,
        C: HtmlElement,
        I: IntoIterator,
        F: Fn(I::Item, Element<C>) -> Element<C>,
    {
        for item in items {
            let child = f(item, Element::<C>::new());
            self.children.push(child.into_node());
        }
        self
    }

    /// Conditionally add content.
    pub fn when<F>(self, condition: bool, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        if condition {
            f(self)
        } else {
            self
        }
    }

    /// Conditionally add content with else branch.
    pub fn when_else<F, G>(self, condition: bool, if_true: F, if_false: G) -> Self
    where
        F: FnOnce(Self) -> Self,
        G: FnOnce(Self) -> Self,
    {
        if condition {
            if_true(self)
        } else {
            if_false(self)
        }
    }

    /// Convert this element into a renderable node.
    pub fn into_node(self) -> TypedNode {
        TypedNode::Element {
            tag: E::TAG,
            is_void: E::VOID,
            attrs: self.attrs,
            children: self.children,
        }
    }

    /// Render this element to a string.
    pub fn render(&self) -> String {
        let mut output = String::new();
        self.render_to(&mut output);
        output
    }

    /// Render this element to an existing string buffer.
    pub fn render_to(&self, output: &mut String) {
        output.push('<');
        output.push_str(E::TAG);

        for (name, value) in &self.attrs {
            output.push(' ');
            output.push_str(name);
            if !value.is_empty() {
                output.push_str("=\"");
                output.push_str(&escape_attr(value));
                output.push('"');
            }
        }

        if E::VOID && self.children.is_empty() {
            output.push_str(" />");
        } else {
            output.push('>');

            for child in &self.children {
                child.render_to(output);
            }

            output.push_str("</");
            output.push_str(E::TAG);
            output.push('>');
        }
    }
}

/// A typed HTML document builder.
#[derive(Debug, Clone, Default)]
pub struct Document {
    nodes: Vec<TypedNode>,
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Document { nodes: Vec::new() }
    }

    /// Add the HTML5 doctype declaration.
    pub fn doctype(mut self) -> Self {
        self.nodes
            .push(TypedNode::Raw("<!DOCTYPE html>".to_string()));
        self
    }

    /// Add a root element.
    pub fn root<E, F>(mut self, f: F) -> Self
    where
        E: HtmlElement,
        F: FnOnce(Element<E>) -> Element<E>,
    {
        let elem = f(Element::<E>::new());
        self.nodes.push(elem.into_node());
        self
    }

    /// Add raw HTML at the document level.
    pub fn raw(mut self, html: impl Into<String>) -> Self {
        self.nodes.push(TypedNode::Raw(html.into()));
        self
    }

    /// Build the final HTML string.
    pub fn build(&self) -> String {
        let mut output = String::new();
        for node in &self.nodes {
            node.render_to(&mut output);
        }
        output
    }

    /// Render the document to a string (alias for `build`).
    pub fn render(&self) -> String {
        self.build()
    }

    /// Write the document to a file.
    ///
    /// Requires the `std` feature.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use html_builder::typed::Document;
    /// use html_elements::{Html, Head, Body, Title, P};
    ///
    /// let doc = Document::new()
    ///     .doctype()
    ///     .root::<Html, _>(|html| {
    ///         html.child::<Head, _>(|h| h.child::<Title, _>(|t| t.text("Hello")))
    ///             .child::<Body, _>(|b| b.child::<P, _>(|p| p.text("Hello, World!")))
    ///     });
    ///
    /// // Write to a temp file
    /// let temp_path = std::env::temp_dir().join("html_builder_doctest.html");
    /// doc.write_to_file(&temp_path).expect("Failed to write file");
    ///
    /// // Verify the file was written correctly
    /// let content = std::fs::read_to_string(&temp_path).unwrap();
    /// assert!(content.contains("<!DOCTYPE html>"));
    /// assert!(content.contains("<title>Hello</title>"));
    ///
    /// // Clean up
    /// std::fs::remove_file(&temp_path).ok();
    /// ```
    #[cfg(feature = "std")]
    pub fn write_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        std::fs::write(path, self.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_elements::*;

    #[test]
    fn test_simple_element() {
        let html = Element::<Div>::new()
            .class("container")
            .text("Hello")
            .render();
        assert_eq!(html, r#"<div class="container">Hello</div>"#);
    }

    #[test]
    fn test_nested_elements() {
        let html = Element::<Table>::new()
            .class("table")
            .child::<Tr, _>(|tr| {
                tr.child::<Td, _>(|td| td.text("Cell 1"))
                    .child::<Td, _>(|td| td.text("Cell 2"))
            })
            .render();

        assert_eq!(
            html,
            r#"<table class="table"><tr><td>Cell 1</td><td>Cell 2</td></tr></table>"#
        );
    }

    #[test]
    fn test_list() {
        let items = ["Apple", "Banana", "Cherry"];
        let html = Element::<Ul>::new()
            .children(items, |item, li: Element<Li>| li.text(item))
            .render();

        assert_eq!(
            html,
            r#"<ul><li>Apple</li><li>Banana</li><li>Cherry</li></ul>"#
        );
    }

    #[test]
    fn test_void_element() {
        let html = Element::<Img>::new()
            .attr("src", "image.jpg")
            .attr("alt", "An image")
            .render();

        assert_eq!(html, r#"<img src="image.jpg" alt="An image" />"#);
    }

    #[test]
    fn test_document() {
        let html = Document::new()
            .doctype()
            .root::<Html, _>(|html| {
                html.attr("lang", "en")
                    .child::<Head, _>(|head| {
                        head.child::<Meta, _>(|meta| meta.attr("charset", "UTF-8"))
                            .child::<Title, _>(|title| title.text("Hello"))
                    })
                    .child::<Body, _>(|body| body.child::<H1, _>(|h1| h1.text("Hello, World!")))
            })
            .build();

        assert_eq!(
            html,
            r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8" /><title>Hello</title></head><body><h1>Hello, World!</h1></body></html>"#
        );
    }

    #[test]
    fn test_class_chaining() {
        let html = Element::<Div>::new()
            .class("btn")
            .class("btn-primary")
            .class("active")
            .render();

        assert_eq!(html, r#"<div class="btn btn-primary active"></div>"#);
    }

    #[test]
    fn test_data_attributes() {
        let html = Element::<Div>::new()
            .data("id", "123")
            .data("action", "submit")
            .render();

        assert_eq!(html, r#"<div data-id="123" data-action="submit"></div>"#);
    }

    #[test]
    fn test_conditional() {
        let show = true;
        let html = Element::<Div>::new()
            .when(show, |e| e.child::<Span, _>(|s| s.text("Visible")))
            .render();

        assert_eq!(html, r#"<div><span>Visible</span></div>"#);

        let hide = false;
        let html = Element::<Div>::new()
            .when(hide, |e| e.child::<Span, _>(|s| s.text("Hidden")))
            .render();

        assert_eq!(html, r#"<div></div>"#);
    }

    #[test]
    fn test_escape_text() {
        let html = Element::<Div>::new()
            .text("<script>alert('xss')</script>")
            .render();

        assert_eq!(
            html,
            r#"<div>&lt;script&gt;alert('xss')&lt;/script&gt;</div>"#
        );
    }

    #[test]
    fn test_escape_attr() {
        let html = Element::<Div>::new()
            .attr("data-value", "say \"hello\"")
            .render();

        assert_eq!(html, r#"<div data-value="say &quot;hello&quot;"></div>"#);
    }

    #[test]
    fn test_type_safe_attribute_value() {
        use html_attributes::{InputType, Loading};

        let html = Element::<Input>::new()
            .attr_value(html_attributes::input::TYPE, InputType::Email)
            .attr("name", "email")
            .render();

        assert_eq!(html, r#"<input type="email" name="email" />"#);

        let html = Element::<Img>::new()
            .attr("src", "large.jpg")
            .attr_value(html_attributes::img::LOADING, Loading::Lazy)
            .render();

        assert_eq!(html, r#"<img src="large.jpg" loading="lazy" />"#);
    }

    #[test]
    fn test_form() {
        use html_attributes::Method;

        let html = Element::<Form>::new()
            .attr("action", "/submit")
            .attr_value(html_attributes::form::METHOD, Method::Post)
            .child::<Input, _>(|i| {
                i.attr("type", "text")
                    .attr("name", "username")
                    .attr("placeholder", "Username")
            })
            .child::<Input, _>(|i| i.attr("type", "password").attr("name", "password"))
            .child::<Button, _>(|b| b.attr("type", "submit").text("Login"))
            .render();

        assert!(html.contains(r#"<form action="/submit" method="post">"#));
        assert!(html.contains(r#"<input type="text" name="username""#));
        assert!(html.contains(r#"<button type="submit">Login</button>"#));
    }

    #[test]
    fn test_anchor_link() {
        use html_attributes::Target;

        let html = Element::<A>::new()
            .attr("href", "https://example.com")
            .attr_value(html_attributes::anchor::TARGET, Target::Blank)
            .attr("rel", "noopener noreferrer")
            .text("External Link")
            .render();

        assert_eq!(
            html,
            r#"<a href="https://example.com" target="_blank" rel="noopener noreferrer">External Link</a>"#
        );
    }

    #[test]
    fn test_select_options() {
        let html = Element::<Select>::new()
            .attr("name", "country")
            .child::<Option_, _>(|o| o.attr("value", "us").text("United States"))
            .child::<Option_, _>(|o| o.attr("value", "uk").text("United Kingdom"))
            .child::<Option_, _>(|o| o.attr("value", "ca").text("Canada"))
            .render();

        assert!(html.contains(r#"<select name="country">"#));
        assert!(html.contains(r#"<option value="us">United States</option>"#));
        assert!(html.contains(r#"<option value="uk">United Kingdom</option>"#));
    }

    #[test]
    fn test_definition_list() {
        let html = Element::<Dl>::new()
            .child::<Dt, _>(|dt| dt.text("HTML"))
            .child::<Dd, _>(|dd| dd.text("HyperText Markup Language"))
            .child::<Dt, _>(|dt| dt.text("CSS"))
            .child::<Dd, _>(|dd| dd.text("Cascading Style Sheets"))
            .render();

        assert!(html.contains("<dl>"));
        assert!(html.contains("<dt>HTML</dt>"));
        assert!(html.contains("<dd>HyperText Markup Language</dd>"));
    }

    #[test]
    fn test_complex_table() {
        let html = Element::<Table>::new()
            .class("table")
            .child::<Thead, _>(|thead| {
                thead.child::<Tr, _>(|tr| {
                    tr.child::<Th, _>(|th| th.text("Name"))
                        .child::<Th, _>(|th| th.text("Age"))
                })
            })
            .child::<Tbody, _>(|tbody| {
                tbody
                    .child::<Tr, _>(|tr| {
                        tr.child::<Td, _>(|td| td.text("Alice"))
                            .child::<Td, _>(|td| td.text("30"))
                    })
                    .child::<Tr, _>(|tr| {
                        tr.child::<Td, _>(|td| td.text("Bob"))
                            .child::<Td, _>(|td| td.text("25"))
                    })
            })
            .render();

        assert!(html.contains("<thead>"));
        assert!(html.contains("<th>Name</th>"));
        assert!(html.contains("<tbody>"));
        assert!(html.contains("<td>Alice</td>"));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_write_to_file() {
        use std::fs;
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_html_builder_output.html");

        let doc = Document::new().doctype().root::<Html, _>(|html| {
            html.child::<Head, _>(|h| h.child::<Title, _>(|t| t.text("Test")))
                .child::<Body, _>(|b| b.child::<P, _>(|p| p.text("Hello")))
        });

        doc.write_to_file(&file_path).expect("Failed to write file");

        let content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("<title>Test</title>"));
        assert!(content.contains("<p>Hello</p>"));

        // Clean up
        fs::remove_file(file_path).ok();
    }
}
