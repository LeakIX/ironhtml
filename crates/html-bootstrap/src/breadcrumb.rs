//! Bootstrap breadcrumb components.
//!
//! Provides type-safe Bootstrap breadcrumbs matching the
//! [Bootstrap breadcrumb documentation](https://getbootstrap.com/docs/5.3/components/breadcrumb/).

use html_builder::typed::Element;
use html_elements::{Li, Nav, Ol, A};

extern crate alloc;
use alloc::string::String;

/// A breadcrumb item.
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub active: bool,
}

impl BreadcrumbItem {
    /// Create a link breadcrumb item.
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            active: false,
        }
    }

    /// Create the active (current) breadcrumb item.
    pub fn active(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            active: true,
        }
    }
}

/// Create a Bootstrap breadcrumb navigation.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::breadcrumb::{breadcrumb, BreadcrumbItem};
///
/// let items = vec![
///     BreadcrumbItem::link("Home", "/"),
///     BreadcrumbItem::link("Library", "/library"),
///     BreadcrumbItem::active("Data"),
/// ];
///
/// let nav = breadcrumb(&items);
/// assert!(nav.render().contains("breadcrumb"));
/// assert!(nav.render().contains(r#"aria-current="page"#));
/// ```
pub fn breadcrumb(items: &[BreadcrumbItem]) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "breadcrumb")
        .child::<Ol, _>(|ol| {
            items.iter().fold(ol.class("breadcrumb"), |ol, item| {
                ol.child::<Li, _>(|_| breadcrumb_item(item))
            })
        })
}

/// Create a single breadcrumb item.
fn breadcrumb_item(item: &BreadcrumbItem) -> Element<Li> {
    if item.active {
        Element::<Li>::new()
            .class("breadcrumb-item active")
            .attr("aria-current", "page")
            .text(&item.label)
    } else if let Some(ref href) = item.href {
        Element::<Li>::new()
            .class("breadcrumb-item")
            .child::<A, _>(|a| a.attr("href", href).text(&item.label))
    } else {
        Element::<Li>::new()
            .class("breadcrumb-item")
            .text(&item.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_breadcrumb() {
        let items = vec![
            BreadcrumbItem::link("Home", "/"),
            BreadcrumbItem::link("Products", "/products"),
            BreadcrumbItem::active("Widget"),
        ];

        let nav = breadcrumb(&items);
        let html = nav.render();
        assert!(html.contains("breadcrumb"));
        assert!(html.contains(r#"aria-label="breadcrumb"#));
        assert!(html.contains(r#"href="/"#));
        assert!(html.contains(r#"aria-current="page"#));
        assert!(html.contains("active"));
    }
}
