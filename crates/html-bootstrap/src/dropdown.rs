//! Bootstrap dropdown components.
//!
//! Provides type-safe Bootstrap dropdowns matching the
//! [Bootstrap dropdown documentation](https://getbootstrap.com/docs/5.3/components/dropdowns/).

use crate::Color;
use html_builder::typed::Element;
use html_elements::{Button, Div, Hr, Li, Span, Ul, A};

extern crate alloc;
use alloc::format;
use alloc::string::String;

/// A dropdown menu item.
pub enum DropdownItem {
    /// A clickable link item.
    Link { text: String, href: String },
    /// An active (highlighted) link item.
    Active { text: String, href: String },
    /// A disabled link item.
    Disabled { text: String, href: String },
    /// A divider line.
    Divider,
    /// A non-interactive header.
    Header(String),
    /// Plain text (non-interactive).
    Text(String),
}

impl DropdownItem {
    /// Create a link item.
    pub fn link(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self::Link {
            text: text.into(),
            href: href.into(),
        }
    }

    /// Create an active link item.
    pub fn active(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self::Active {
            text: text.into(),
            href: href.into(),
        }
    }

    /// Create a disabled link item.
    pub fn disabled(text: impl Into<String>, href: impl Into<String>) -> Self {
        Self::Disabled {
            text: text.into(),
            href: href.into(),
        }
    }

    /// Create a divider.
    pub fn divider() -> Self {
        Self::Divider
    }

    /// Create a header.
    pub fn header(text: impl Into<String>) -> Self {
        Self::Header(text.into())
    }

    /// Create plain text.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }
}

/// Create a Bootstrap dropdown.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::dropdown::{dropdown, DropdownItem};
/// use html_bootstrap::Color;
///
/// let items = vec![
///     DropdownItem::link("Action", "#"),
///     DropdownItem::link("Another action", "#"),
///     DropdownItem::divider(),
///     DropdownItem::link("Separated link", "#"),
/// ];
///
/// let dd = dropdown(Color::Primary, "Dropdown button", &items);
/// assert!(dd.render().contains("dropdown"));
/// ```
pub fn dropdown(color: Color, label: &str, items: &[DropdownItem]) -> Element<Div> {
    let btn_class = format!("btn btn-{} dropdown-toggle", color.as_str());

    Element::<Div>::new()
        .class("dropdown")
        .child::<Button, _>(|b| {
            b.class(&btn_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .text(label)
        })
        .child::<Ul, _>(|ul| dropdown_menu(ul, items))
}

/// Create a dropdown with a split button.
pub fn dropdown_split(
    color: Color,
    label: &str,
    href: &str,
    items: &[DropdownItem],
) -> Element<Div> {
    let btn_class = format!("btn btn-{}", color.as_str());
    let toggle_class = format!(
        "btn btn-{} dropdown-toggle dropdown-toggle-split",
        color.as_str()
    );

    Element::<Div>::new()
        .class("btn-group")
        .child::<A, _>(|a| a.class(&btn_class).attr("href", href).text(label))
        .child::<Button, _>(|b| {
            b.class(&toggle_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .child::<Span, _>(|s| s.class("visually-hidden").text("Toggle Dropdown"))
        })
        .child::<Ul, _>(|ul| dropdown_menu(ul, items))
}

/// Create a dropup (opens upward).
pub fn dropup(color: Color, label: &str, items: &[DropdownItem]) -> Element<Div> {
    let btn_class = format!("btn btn-{} dropdown-toggle", color.as_str());

    Element::<Div>::new()
        .class("btn-group dropup")
        .child::<Button, _>(|b| {
            b.class(&btn_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .text(label)
        })
        .child::<Ul, _>(|ul| dropdown_menu(ul, items))
}

/// Create a dropstart (opens to the left).
pub fn dropstart(color: Color, label: &str, items: &[DropdownItem]) -> Element<Div> {
    let btn_class = format!("btn btn-{} dropdown-toggle", color.as_str());

    Element::<Div>::new()
        .class("btn-group dropstart")
        .child::<Button, _>(|b| {
            b.class(&btn_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .text(label)
        })
        .child::<Ul, _>(|ul| dropdown_menu(ul, items))
}

/// Create a dropend (opens to the right).
pub fn dropend(color: Color, label: &str, items: &[DropdownItem]) -> Element<Div> {
    let btn_class = format!("btn btn-{} dropdown-toggle", color.as_str());

    Element::<Div>::new()
        .class("btn-group dropend")
        .child::<Button, _>(|b| {
            b.class(&btn_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .text(label)
        })
        .child::<Ul, _>(|ul| dropdown_menu(ul, items))
}

/// Create a dark dropdown menu.
pub fn dropdown_dark(color: Color, label: &str, items: &[DropdownItem]) -> Element<Div> {
    let btn_class = format!("btn btn-{} dropdown-toggle", color.as_str());

    Element::<Div>::new()
        .class("dropdown")
        .child::<Button, _>(|b| {
            b.class(&btn_class)
                .attr("type", "button")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-expanded", "false")
                .text(label)
        })
        .child::<Ul, _>(|ul| dropdown_menu_dark(ul, items))
}

/// Build a dropdown menu element.
fn dropdown_menu(ul: Element<Ul>, items: &[DropdownItem]) -> Element<Ul> {
    items
        .iter()
        .fold(ul.class("dropdown-menu"), |ul, item| match item {
            DropdownItem::Link { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| a.class("dropdown-item").attr("href", href).text(text))
            }),
            DropdownItem::Active { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| {
                    a.class("dropdown-item active")
                        .attr("href", href)
                        .attr("aria-current", "true")
                        .text(text)
                })
            }),
            DropdownItem::Disabled { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| {
                    a.class("dropdown-item disabled")
                        .attr("href", href)
                        .attr("aria-disabled", "true")
                        .text(text)
                })
            }),
            DropdownItem::Divider => {
                ul.child::<Li, _>(|li| li.child::<Hr, _>(|hr| hr.class("dropdown-divider")))
            }
            DropdownItem::Header(text) => {
                ul.child::<Li, _>(|li| li.child::<H6, _>(|h| h.class("dropdown-header").text(text)))
            }
            DropdownItem::Text(text) => ul.child::<Li, _>(|li| {
                li.child::<Span, _>(|s| s.class("dropdown-item-text").text(text))
            }),
        })
}

use html_elements::H6;

/// Build a dark dropdown menu element.
fn dropdown_menu_dark(ul: Element<Ul>, items: &[DropdownItem]) -> Element<Ul> {
    items.iter().fold(
        ul.class("dropdown-menu dropdown-menu-dark"),
        |ul, item| match item {
            DropdownItem::Link { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| a.class("dropdown-item").attr("href", href).text(text))
            }),
            DropdownItem::Active { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| {
                    a.class("dropdown-item active")
                        .attr("href", href)
                        .attr("aria-current", "true")
                        .text(text)
                })
            }),
            DropdownItem::Disabled { text, href } => ul.child::<Li, _>(|li| {
                li.child::<A, _>(|a| {
                    a.class("dropdown-item disabled")
                        .attr("href", href)
                        .attr("aria-disabled", "true")
                        .text(text)
                })
            }),
            DropdownItem::Divider => {
                ul.child::<Li, _>(|li| li.child::<Hr, _>(|hr| hr.class("dropdown-divider")))
            }
            DropdownItem::Header(text) => {
                ul.child::<Li, _>(|li| li.child::<H6, _>(|h| h.class("dropdown-header").text(text)))
            }
            DropdownItem::Text(text) => ul.child::<Li, _>(|li| {
                li.child::<Span, _>(|s| s.class("dropdown-item-text").text(text))
            }),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_dropdown() {
        let items = vec![
            DropdownItem::link("Action", "#"),
            DropdownItem::divider(),
            DropdownItem::link("Another", "#"),
        ];
        let dd = dropdown(Color::Primary, "Menu", &items);
        let html = dd.render();
        assert!(html.contains("dropdown"));
        assert!(html.contains("dropdown-toggle"));
        assert!(html.contains("dropdown-menu"));
        assert!(html.contains("dropdown-item"));
    }

    #[test]
    fn test_dropdown_split() {
        let items = vec![DropdownItem::link("Action", "#")];
        let dd = dropdown_split(Color::Success, "Action", "#", &items);
        let html = dd.render();
        assert!(html.contains("btn-group"));
        assert!(html.contains("dropdown-toggle-split"));
    }

    #[test]
    fn test_dropup() {
        let items = vec![DropdownItem::link("Action", "#")];
        let dd = dropup(Color::Primary, "Dropup", &items);
        assert!(dd.render().contains("dropup"));
    }

    #[test]
    fn test_dropdown_with_header() {
        let items = vec![
            DropdownItem::header("Dropdown header"),
            DropdownItem::link("Action", "#"),
        ];
        let dd = dropdown(Color::Secondary, "Menu", &items);
        assert!(dd.render().contains("dropdown-header"));
    }
}
