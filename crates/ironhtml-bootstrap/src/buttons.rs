//! Bootstrap button components.
//!
//! Provides type-safe Bootstrap button generation matching the
//! [Bootstrap button documentation](https://getbootstrap.com/docs/5.3/components/buttons/).
//!
//! ## Example
//!
//! ```rust
//! use ironhtml_bootstrap::{buttons::*, Color, Size};
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
use ironhtml::typed::Element;
use ironhtml_elements::Button;

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
/// use ironhtml_bootstrap::{buttons::btn, Color};
///
/// let button = btn(Color::Primary, "Click me");
/// assert_eq!(
///     button.render(),
///     r#"<button type="button" class="btn btn-primary">Click me</button>"#
/// );
/// ```
#[must_use]
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
#[must_use]
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
#[must_use]
pub fn btn_sized(color: Color, size: Size, text: &str) -> Element<Button> {
    let size_class = size.as_btn_class();
    let class = if size_class.is_empty() {
        format!("btn btn-{}", color.as_str())
    } else {
        format!("btn btn-{} {size_class}", color.as_str())
    };
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a sized outline Bootstrap button.
#[must_use]
pub fn btn_outline_sized(color: Color, size: Size, text: &str) -> Element<Button> {
    let size_class = size.as_btn_class();
    let class = if size_class.is_empty() {
        format!("btn btn-outline-{}", color.as_str())
    } else {
        format!("btn btn-outline-{} {size_class}", color.as_str())
    };
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a disabled button.
#[must_use]
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
#[must_use]
pub fn btn_link(text: &str) -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn btn-link")
        .text(text)
}

/// Create a button with a loading spinner (border style).
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{buttons::btn_loading, Color};
///
/// let btn = btn_loading(Color::Primary, "Loading...");
/// assert!(btn.render().contains("spinner-border"));
/// ```
#[must_use]
pub fn btn_loading(color: Color, text: &str) -> Element<Button> {
    use ironhtml_elements::Span;

    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .bool_attr("disabled")
        .child::<Span, _>(|s| {
            s.class("spinner-border spinner-border-sm")
                .attr("role", "status")
                .attr("aria-hidden", "true")
        })
        .text(format!(" {text}"))
}

/// Create a button with a growing loading spinner.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{buttons::btn_loading_grow, Color};
///
/// let btn = btn_loading_grow(Color::Primary, "Loading...");
/// assert!(btn.render().contains("spinner-grow"));
/// ```
#[must_use]
pub fn btn_loading_grow(color: Color, text: &str) -> Element<Button> {
    use ironhtml_elements::Span;

    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .bool_attr("disabled")
        .child::<Span, _>(|s| {
            s.class("spinner-grow spinner-grow-sm")
                .attr("role", "status")
                .attr("aria-hidden", "true")
        })
        .text(format!(" {text}"))
}

/// Create a block-level button (full width).
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{buttons::btn_block, Color};
///
/// let btn = btn_block(Color::Primary, "Full Width");
/// assert!(btn.render().contains("w-100"));
/// ```
#[must_use]
pub fn btn_block(color: Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-{} w-100", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .text(text)
}

/// Create a button with an icon.
#[must_use]
pub fn btn_icon(color: Color, icon_class: &str, text: &str) -> Element<Button> {
    use ironhtml_elements::I;

    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .child::<I, _>(|i| i.class(icon_class))
        .text(format!(" {text}"))
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
