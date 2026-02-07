//! Bootstrap navbar components.
//!
//! Provides type-safe Bootstrap navbars matching the
//! [Bootstrap navbar documentation](https://getbootstrap.com/docs/5.3/components/navbar/).
//!
//! ## Example
//!
//! ```rust
//! use ironhtml_bootstrap::navbar::*;
//! use ironhtml_bootstrap::NavbarExpand;
//!
//! // Create a navbar with brand and nav items
//! let nav = navbar("MyApp", NavbarExpand::Lg, "main-nav", |nav| {
//!     nav.child(nav_item("/", "Home", true))
//!        .child(nav_item("/about", "About", false))
//!        .child(nav_item("/contact", "Contact", false))
//! });
//!
//! let html = nav.render();
//! assert!(html.contains("navbar"));
//! assert!(html.contains("navbar-brand"));
//! assert!(html.contains("nav-item"));
//! ```

use crate::NavbarExpand;
use ironhtml::typed::Element;
use ironhtml_elements::{Button, Div, Li, Nav, Span, Ul, A};

extern crate alloc;
use alloc::format;

/// Create a Bootstrap navbar.
///
/// Generates the standard responsive navbar structure from Bootstrap docs.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{navbar::navbar, NavbarExpand};
///
/// let nav = navbar("Brand", NavbarExpand::Lg, "navbarNav", |nav| {
///     nav // add nav items here
/// });
/// assert!(nav.render().contains("navbar-brand"));
/// ```
#[must_use]
pub fn navbar<F>(brand: &str, expand: NavbarExpand, id: &str, f: F) -> Element<Nav>
where
    F: FnOnce(Element<Ul>) -> Element<Ul>,
{
    let class = format!("navbar {} bg-body-tertiary", expand.as_class());
    let target = format!("#{id}");

    Element::<Nav>::new()
        .class(&class)
        .child::<Div, _>(|container| {
            container
                .class("container-fluid")
                .child::<A, _>(|a| a.class("navbar-brand").attr("href", "#").text(brand))
                .child::<Button, _>(|btn| {
                    btn.class("navbar-toggler")
                        .attr("type", "button")
                        .attr("data-bs-toggle", "collapse")
                        .attr("data-bs-target", &target)
                        .attr("aria-controls", id)
                        .attr("aria-expanded", "false")
                        .attr("aria-label", "Toggle navigation")
                        .child::<Span, _>(|s| s.class("navbar-toggler-icon"))
                })
                .child::<Div, _>(|collapse| {
                    collapse
                        .class("collapse navbar-collapse")
                        .id(id)
                        .child::<Ul, _>(|ul| f(ul.class("navbar-nav me-auto mb-2 mb-lg-0")))
                })
        })
}

/// Create a navbar with dark theme.
#[must_use]
pub fn navbar_dark<F>(brand: &str, expand: NavbarExpand, id: &str, f: F) -> Element<Nav>
where
    F: FnOnce(Element<Ul>) -> Element<Ul>,
{
    let class = format!("navbar {} bg-dark", expand.as_class());
    let target = format!("#{id}");

    Element::<Nav>::new()
        .class(&class)
        .attr("data-bs-theme", "dark")
        .child::<Div, _>(|container| {
            container
                .class("container-fluid")
                .child::<A, _>(|a| a.class("navbar-brand").attr("href", "#").text(brand))
                .child::<Button, _>(|btn| {
                    btn.class("navbar-toggler")
                        .attr("type", "button")
                        .attr("data-bs-toggle", "collapse")
                        .attr("data-bs-target", &target)
                        .attr("aria-controls", id)
                        .attr("aria-expanded", "false")
                        .attr("aria-label", "Toggle navigation")
                        .child::<Span, _>(|s| s.class("navbar-toggler-icon"))
                })
                .child::<Div, _>(|collapse| {
                    collapse
                        .class("collapse navbar-collapse")
                        .id(id)
                        .child::<Ul, _>(|ul| f(ul.class("navbar-nav me-auto mb-2 mb-lg-0")))
                })
        })
}

/// Create a nav item.
///
/// Generates: `<li class="nav-item"><a class="nav-link" href="...">{text}</a></li>`
#[must_use]
pub fn nav_item(href: &str, text: &str, active: bool) -> Element<Li> {
    let link_class = if active {
        "nav-link active"
    } else {
        "nav-link"
    };

    if active {
        Element::<Li>::new().class("nav-item").child::<A, _>(|a| {
            a.class(link_class)
                .attr("aria-current", "page")
                .attr("href", href)
                .text(text)
        })
    } else {
        Element::<Li>::new()
            .class("nav-item")
            .child::<A, _>(|a| a.class(link_class).attr("href", href).text(text))
    }
}

/// Create a disabled nav item.
#[must_use]
pub fn nav_item_disabled(text: &str) -> Element<Li> {
    Element::<Li>::new().class("nav-item").child::<A, _>(|a| {
        a.class("nav-link disabled")
            .attr("aria-disabled", "true")
            .text(text)
    })
}

/// Wrapper to add nav items to a navbar.
///
/// This trait allows adding nav items to a ul element.
pub trait NavItemExt {
    /// Add a nav item to this navbar.
    #[must_use]
    fn child(self, item: Element<Li>) -> Self;
}

impl NavItemExt for Element<Ul> {
    fn child(self, item: Element<Li>) -> Self {
        self.child::<Li, _>(|_| item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navbar() {
        let nav = navbar("Brand", NavbarExpand::Lg, "nav", |ul| ul);
        let html = nav.render();
        assert!(html.contains("navbar"));
        assert!(html.contains("navbar-expand-lg"));
        assert!(html.contains("navbar-brand"));
        assert!(html.contains("navbar-toggler"));
        assert!(html.contains("collapse navbar-collapse"));
    }

    #[test]
    fn test_nav_item() {
        let item = nav_item("/home", "Home", true);
        let html = item.render();
        assert!(html.contains("nav-item"));
        assert!(html.contains("nav-link active"));
        assert!(html.contains(r#"aria-current="page"#));
    }

    #[test]
    fn test_nav_item_inactive() {
        let item = nav_item("/about", "About", false);
        let html = item.render();
        assert!(html.contains("nav-item"));
        assert!(html.contains("nav-link"));
        assert!(!html.contains("active"));
    }
}
