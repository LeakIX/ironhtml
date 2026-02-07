//! Bootstrap spinner components.
//!
//! Provides type-safe Bootstrap spinners matching the
//! [Bootstrap spinner documentation](https://getbootstrap.com/docs/5.3/components/spinners/).

use crate::Color;
use ironhtml::typed::Element;
use ironhtml_elements::{Div, Span};

extern crate alloc;
use alloc::format;

/// Create a Bootstrap border spinner.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::spinner::spinner;
///
/// let s = spinner();
/// assert!(s.render().contains("spinner-border"));
/// ```
#[must_use]
pub fn spinner() -> Element<Div> {
    Element::<Div>::new()
        .class("spinner-border")
        .attr("role", "status")
        .child::<Span, _>(|s| s.class("visually-hidden").text("Loading..."))
}

/// Create a colored border spinner.
#[must_use]
pub fn spinner_colored(color: Color) -> Element<Div> {
    let class = format!("spinner-border text-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .attr("role", "status")
        .child::<Span, _>(|s| s.class("visually-hidden").text("Loading..."))
}

/// Create a growing spinner.
#[must_use]
pub fn spinner_grow() -> Element<Div> {
    Element::<Div>::new()
        .class("spinner-grow")
        .attr("role", "status")
        .child::<Span, _>(|s| s.class("visually-hidden").text("Loading..."))
}

/// Create a colored growing spinner.
#[must_use]
pub fn spinner_grow_colored(color: Color) -> Element<Div> {
    let class = format!("spinner-grow text-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .attr("role", "status")
        .child::<Span, _>(|s| s.class("visually-hidden").text("Loading..."))
}

/// Create a small border spinner.
#[must_use]
pub fn spinner_sm() -> Element<Span> {
    Element::<Span>::new()
        .class("spinner-border spinner-border-sm")
        .attr("role", "status")
        .attr("aria-hidden", "true")
}

/// Create a small growing spinner.
#[must_use]
pub fn spinner_grow_sm() -> Element<Span> {
    Element::<Span>::new()
        .class("spinner-grow spinner-grow-sm")
        .attr("role", "status")
        .attr("aria-hidden", "true")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner() {
        let s = spinner();
        let html = s.render();
        assert!(html.contains("spinner-border"));
        assert!(html.contains("Loading..."));
    }

    #[test]
    fn test_spinner_colored() {
        let s = spinner_colored(Color::Primary);
        assert!(s.render().contains("text-primary"));
    }

    #[test]
    fn test_spinner_grow() {
        let s = spinner_grow();
        assert!(s.render().contains("spinner-grow"));
    }
}
