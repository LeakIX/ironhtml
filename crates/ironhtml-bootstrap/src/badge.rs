//! Bootstrap badge components.
//!
//! Provides type-safe Bootstrap badges matching the
//! [Bootstrap badge documentation](https://getbootstrap.com/docs/5.3/components/badge/).

use crate::Color;
use ironhtml::typed::Element;
use ironhtml_elements::Span;

extern crate alloc;
use alloc::format;

/// Create a Bootstrap badge.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{badge::badge, Color};
///
/// let b = badge(Color::Primary, "New");
/// assert!(b.render().contains("text-bg-primary"));
/// ```
#[must_use]
pub fn badge(color: Color, text: &str) -> Element<Span> {
    let class = format!("badge text-bg-{}", color.as_str());
    Element::<Span>::new().class(&class).text(text)
}

/// Create a pill badge (rounded).
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{badge::badge_pill, Color};
///
/// let b = badge_pill(Color::Danger, "99+");
/// assert!(b.render().contains("rounded-pill"));
/// ```
#[must_use]
pub fn badge_pill(color: Color, text: &str) -> Element<Span> {
    let class = format!("badge rounded-pill text-bg-{}", color.as_str());
    Element::<Span>::new().class(&class).text(text)
}

/// Create a positioned badge (for notifications).
///
/// Use this inside a position-relative button.
#[must_use]
pub fn badge_positioned(color: Color, text: &str) -> Element<Span> {
    let class = format!(
        "position-absolute top-0 start-100 translate-middle badge rounded-pill bg-{}",
        color.as_str()
    );
    Element::<Span>::new()
        .class(&class)
        .text(text)
        .child::<Span, _>(|s| s.class("visually-hidden").text("notifications"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge() {
        let b = badge(Color::Primary, "New");
        assert!(b.render().contains("text-bg-primary"));
        assert!(b.render().contains("New"));
    }

    #[test]
    fn test_badge_pill() {
        let b = badge_pill(Color::Success, "OK");
        assert!(b.render().contains("rounded-pill"));
    }

    #[test]
    fn test_all_colors() {
        for color in [
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Danger,
        ] {
            let b = badge(color, "Test");
            assert!(b.render().contains(&format!("text-bg-{}", color.as_str())));
        }
    }
}
