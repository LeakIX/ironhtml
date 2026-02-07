//! Bootstrap collapse components.
//!
//! Provides type-safe Bootstrap collapse functionality matching the
//! [Bootstrap collapse documentation](https://getbootstrap.com/docs/5.3/components/collapse/).

use ironhtml::typed::Element;
use ironhtml_elements::{Button, Div, A};

extern crate alloc;
use alloc::format;

/// Create a collapse trigger button.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::collapse::collapse_button;
///
/// let btn = collapse_button("collapseExample", "Toggle content");
/// assert!(btn.render().contains("data-bs-toggle"));
/// ```
#[must_use]
pub fn collapse_button(target_id: &str, text: &str) -> Element<Button> {
    let target = format!("#{target_id}");
    Element::<Button>::new()
        .class("btn btn-primary")
        .attr("type", "button")
        .attr("data-bs-toggle", "collapse")
        .attr("data-bs-target", &target)
        .attr("aria-expanded", "false")
        .attr("aria-controls", target_id)
        .text(text)
}

/// Create a collapse trigger link.
#[must_use]
pub fn collapse_link(target_id: &str, text: &str) -> Element<A> {
    let href = format!("#{target_id}");
    Element::<A>::new()
        .class("btn btn-primary")
        .attr("data-bs-toggle", "collapse")
        .attr("href", &href)
        .attr("role", "button")
        .attr("aria-expanded", "false")
        .attr("aria-controls", target_id)
        .text(text)
}

/// Create a collapse content container.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::collapse::{collapse_button, collapse_content};
///
///
/// let button = collapse_button("demo", "Click me");
/// let content = collapse_content("demo", |div| {
///     div.class("card card-body").text("Hidden content here")
/// });
///
/// assert!(content.render().contains("collapse"));
/// ```
#[must_use]
pub fn collapse_content<F>(id: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("collapse").attr("id", id))
}

/// Create a collapse content container that starts open.
#[must_use]
pub fn collapse_content_show<F>(id: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("collapse show").attr("id", id))
}

/// Create a horizontal collapse content container.
#[must_use]
pub fn collapse_horizontal<F>(id: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new()
        .class("collapse collapse-horizontal")
        .attr("id", id))
}

/// Create a button that triggers multiple collapse targets.
#[must_use]
pub fn collapse_multi_button(target_class: &str, text: &str) -> Element<Button> {
    let target = format!(".{target_class}");
    Element::<Button>::new()
        .class("btn btn-primary")
        .attr("type", "button")
        .attr("data-bs-toggle", "collapse")
        .attr("data-bs-target", &target)
        .attr("aria-expanded", "false")
        .text(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_button() {
        let btn = collapse_button("demo", "Toggle");
        let html = btn.render();
        assert!(html.contains("data-bs-toggle=\"collapse\""));
        assert!(html.contains("data-bs-target=\"#demo\""));
        assert!(html.contains("aria-controls=\"demo\""));
    }

    #[test]
    fn test_collapse_content() {
        let content = collapse_content("demo", |div| div.text("Content"));
        let html = content.render();
        assert!(html.contains("collapse"));
        assert!(html.contains("id=\"demo\""));
    }

    #[test]
    fn test_collapse_horizontal() {
        let content = collapse_horizontal("demo", |div| div.text("Content"));
        assert!(content.render().contains("collapse-horizontal"));
    }
}
