//! Bootstrap button components.
//!
//! Provides type-safe Bootstrap button generation matching the
//! [Bootstrap button documentation](https://getbootstrap.com/docs/5.3/components/buttons/).
//!
//! ## Example
//!
//! ```rust
//! use html_bootstrap::{buttons::*, Color, Size};
//!
//! // Primary button
//! let primary = btn(Color::Primary, "Click me");
//! assert!(primary.render().contains(r#"class="btn btn-primary"#));
//!
//! // Outline button
//! let outline = btn_outline(Color::Danger, "Delete");
//! assert!(outline.render().contains(r#"class="btn btn-outline-danger"#));
//!
//! // Large button
//! let large = btn_sized(Color::Success, Size::Large, "Submit");
//! assert!(large.render().contains("btn-lg"));
//! ```

use crate::{Color, Size};
use html_builder::typed::Element;
use html_elements::Button;

extern crate alloc;
use alloc::format;

/// Create a Bootstrap button.
///
/// Generates:
/// ```html
/// <button type="button" class="btn btn-{color}">{text}</button>
/// ```
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::{buttons::btn, Color};
///
/// let button = btn(Color::Primary, "Click me");
/// assert_eq!(
///     button.render(),
///     r#"<button type="button" class="btn btn-primary">Click me</button>"#
/// );
/// ```
pub fn btn(color: Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create an outline Bootstrap button.
///
/// Generates:
/// ```html
/// <button type="button" class="btn btn-outline-{color}">{text}</button>
/// ```
pub fn btn_outline(color: Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-outline-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a sized Bootstrap button.
///
/// Generates:
/// ```html
/// <button type="button" class="btn btn-{color} btn-{size}">{text}</button>
/// ```
pub fn btn_sized(color: Color, size: Size, text: &str) -> Element<Button> {
    let size_class = size.as_btn_class();
    let class = if size_class.is_empty() {
        format!("btn btn-{}", color.as_str())
    } else {
        format!("btn btn-{} {}", color.as_str(), size_class)
    };
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a sized outline Bootstrap button.
pub fn btn_outline_sized(color: Color, size: Size, text: &str) -> Element<Button> {
    let size_class = size.as_btn_class();
    let class = if size_class.is_empty() {
        format!("btn btn-outline-{}", color.as_str())
    } else {
        format!("btn btn-outline-{} {}", color.as_str(), size_class)
    };
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a disabled button.
pub fn btn_disabled(color: Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .bool_attr("disabled")
        .text(text)
}

/// Create a link-styled button.
///
/// Generates: `<button type="button" class="btn btn-link">{text}</button>`
pub fn btn_link(text: &str) -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn btn-link")
        .text(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btn_primary() {
        let b = btn(Color::Primary, "Click");
        assert_eq!(
            b.render(),
            r#"<button type="button" class="btn btn-primary">Click</button>"#
        );
    }

    #[test]
    fn test_btn_outline() {
        let b = btn_outline(Color::Danger, "Delete");
        assert!(b.render().contains("btn-outline-danger"));
    }

    #[test]
    fn test_btn_sized() {
        let b = btn_sized(Color::Success, Size::Large, "Submit");
        let html = b.render();
        assert!(html.contains("btn-success"));
        assert!(html.contains("btn-lg"));
    }

    #[test]
    fn test_all_colors() {
        for color in [
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Danger,
            Color::Warning,
            Color::Info,
            Color::Light,
            Color::Dark,
        ] {
            let b = btn(color, "Test");
            assert!(b.render().contains(&format!("btn-{}", color.as_str())));
        }
    }
}
