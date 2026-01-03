//! Bootstrap list group components.
//!
//! Provides type-safe Bootstrap list groups matching the
//! [Bootstrap list group documentation](https://getbootstrap.com/docs/5.3/components/list-group/).

use crate::Color;
use html_builder::typed::Element;
use html_elements::{Button, Div, Li, Ul, A};

extern crate alloc;
use alloc::format;
use alloc::string::String;

/// Create a basic list group.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::list_group::list_group;
///
/// let items = vec!["Item 1", "Item 2", "Item 3"];
/// let list = list_group(&items);
/// assert!(list.render().contains("list-group"));
/// ```
pub fn list_group(items: &[&str]) -> Element<Ul> {
    items
        .iter()
        .fold(Element::<Ul>::new().class("list-group"), |ul, item| {
            ul.child::<Li, _>(|li| li.class("list-group-item").text(*item))
        })
}

/// Create a flush list group (no borders, square corners).
pub fn list_group_flush(items: &[&str]) -> Element<Ul> {
    items.iter().fold(
        Element::<Ul>::new().class("list-group list-group-flush"),
        |ul, item| ul.child::<Li, _>(|li| li.class("list-group-item").text(*item)),
    )
}

/// A list group link item.
pub struct ListGroupLink {
    pub text: String,
    pub href: String,
    pub active: bool,
    pub disabled: bool,
    pub color: Option<Color>,
}

impl ListGroupLink {
    /// Create a new list group link.
    pub fn new(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            href: href.into(),
            active: false,
            disabled: false,
            color: None,
        }
    }

    /// Set this link as active.
    pub fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Set this link as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Set a contextual color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

/// Create a list group with links.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::list_group::{list_group_links, ListGroupLink};
///
/// let items = vec![
///     ListGroupLink::new("Home", "#").active(),
///     ListGroupLink::new("Profile", "#"),
///     ListGroupLink::new("Settings", "#"),
///     ListGroupLink::new("Disabled", "#").disabled(),
/// ];
///
/// let list = list_group_links(&items);
/// assert!(list.render().contains("list-group-item-action"));
/// ```
pub fn list_group_links(items: &[ListGroupLink]) -> Element<Div> {
    items
        .iter()
        .fold(Element::<Div>::new().class("list-group"), |div, item| {
            div.child::<A, _>(|_| list_group_link_item(item))
        })
}

/// Create a single list group link item.
fn list_group_link_item(item: &ListGroupLink) -> Element<A> {
    let mut class = String::from("list-group-item list-group-item-action");

    if item.active {
        class.push_str(" active");
    }
    if item.disabled {
        class.push_str(" disabled");
    }
    if let Some(color) = item.color {
        class.push_str(&format!(" list-group-item-{}", color.as_str()));
    }

    let mut elem = Element::<A>::new()
        .attr("href", &item.href)
        .class(&class)
        .text(&item.text);

    if item.active {
        elem = elem.attr("aria-current", "true");
    }
    if item.disabled {
        elem = elem.attr("aria-disabled", "true");
    }

    elem
}

/// Create a list group with buttons.
pub fn list_group_buttons(items: &[(String, bool, bool)]) -> Element<Div> {
    items.iter().fold(
        Element::<Div>::new().class("list-group"),
        |div, (text, active, disabled)| {
            div.child::<Button, _>(|btn| {
                let mut class = String::from("list-group-item list-group-item-action");
                if *active {
                    class.push_str(" active");
                }

                let mut btn = btn.attr("type", "button").class(&class).text(text);

                if *active {
                    btn = btn.attr("aria-current", "true");
                }
                if *disabled {
                    btn = btn.bool_attr("disabled");
                }
                btn
            })
        },
    )
}

/// Create a numbered list group.
pub fn list_group_numbered(items: &[&str]) -> Element<Ol> {
    items.iter().fold(
        Element::<Ol>::new().class("list-group list-group-numbered"),
        |ol, item| ol.child::<Li, _>(|li| li.class("list-group-item").text(*item)),
    )
}

/// Create a horizontal list group.
pub fn list_group_horizontal(items: &[&str]) -> Element<Ul> {
    items.iter().fold(
        Element::<Ul>::new().class("list-group list-group-horizontal"),
        |ul, item| ul.child::<Li, _>(|li| li.class("list-group-item").text(*item)),
    )
}

use html_elements::Ol;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_list_group() {
        let items = vec!["One", "Two", "Three"];
        let list = list_group(&items);
        let html = list.render();
        assert!(html.contains("list-group"));
        assert!(html.contains("list-group-item"));
    }

    #[test]
    fn test_list_group_links() {
        let items = vec![
            ListGroupLink::new("Active", "#").active(),
            ListGroupLink::new("Normal", "#"),
            ListGroupLink::new("Disabled", "#").disabled(),
        ];
        let list = list_group_links(&items);
        let html = list.render();
        assert!(html.contains("list-group-item-action"));
        assert!(html.contains("active"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_list_group_colored() {
        let items = vec![ListGroupLink::new("Success", "#").color(Color::Success)];
        let list = list_group_links(&items);
        assert!(list.render().contains("list-group-item-success"));
    }
}
