//! Bootstrap tooltip and popover components.
//!
//! Provides type-safe Bootstrap tooltips and popovers matching the
//! [Bootstrap tooltips documentation](https://getbootstrap.com/docs/5.3/components/tooltips/) and
//! [Bootstrap popovers documentation](https://getbootstrap.com/docs/5.3/components/popovers/).

use html_builder::typed::Element;
use html_elements::{Button, A};

extern crate alloc;
use alloc::format;

/// Tooltip/popover placement options.
#[derive(Clone, Copy, Default)]
pub enum Placement {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl Placement {
    fn as_str(&self) -> &'static str {
        match self {
            Placement::Top => "top",
            Placement::Right => "right",
            Placement::Bottom => "bottom",
            Placement::Left => "left",
        }
    }
}

/// Create a button with a tooltip.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::tooltip::{tooltip_button, Placement};
/// use html_bootstrap::Color;
///
/// let btn = tooltip_button(Color::Primary, "Hover me", "Tooltip text", Placement::Top);
/// assert!(btn.render().contains("data-bs-toggle=\"tooltip\""));
/// ```
pub fn tooltip_button(
    color: crate::Color,
    text: &str,
    tooltip: &str,
    placement: Placement,
) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .attr("data-bs-toggle", "tooltip")
        .attr("data-bs-placement", placement.as_str())
        .attr("data-bs-title", tooltip)
        .text(text)
}

/// Create a link with a tooltip.
pub fn tooltip_link(href: &str, text: &str, tooltip: &str, placement: Placement) -> Element<A> {
    Element::<A>::new()
        .attr("href", href)
        .attr("data-bs-toggle", "tooltip")
        .attr("data-bs-placement", placement.as_str())
        .attr("data-bs-title", tooltip)
        .text(text)
}

/// Create a button with HTML tooltip content.
pub fn tooltip_html_button(
    color: crate::Color,
    text: &str,
    tooltip_html: &str,
    placement: Placement,
) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .attr("data-bs-toggle", "tooltip")
        .attr("data-bs-placement", placement.as_str())
        .attr("data-bs-html", "true")
        .attr("data-bs-title", tooltip_html)
        .text(text)
}

/// Create a button with a popover.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::tooltip::{popover_button, Placement};
/// use html_bootstrap::Color;
///
/// let btn = popover_button(
///     Color::Danger,
///     "Click me",
///     "Popover title",
///     "And here's some content.",
///     Placement::Right,
/// );
/// assert!(btn.render().contains("data-bs-toggle=\"popover\""));
/// ```
pub fn popover_button(
    color: crate::Color,
    text: &str,
    title: &str,
    content: &str,
    placement: Placement,
) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .attr("data-bs-toggle", "popover")
        .attr("data-bs-placement", placement.as_str())
        .attr("data-bs-title", title)
        .attr("data-bs-content", content)
        .text(text)
}

/// Create a dismissible popover button (click to open, click again to close).
pub fn popover_dismissible(
    color: crate::Color,
    text: &str,
    title: &str,
    content: &str,
) -> Element<A> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<A>::new()
        .attr("tabindex", "0")
        .class(&class)
        .attr("role", "button")
        .attr("data-bs-toggle", "popover")
        .attr("data-bs-trigger", "focus")
        .attr("data-bs-title", title)
        .attr("data-bs-content", content)
        .text(text)
}

/// Create a popover with HTML content.
pub fn popover_html(
    color: crate::Color,
    text: &str,
    title: &str,
    content_html: &str,
    placement: Placement,
) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .attr("data-bs-toggle", "popover")
        .attr("data-bs-placement", placement.as_str())
        .attr("data-bs-html", "true")
        .attr("data-bs-title", title)
        .attr("data-bs-content", content_html)
        .text(text)
}

/// Create a link that triggers a popover on hover.
pub fn popover_hover(href: &str, text: &str, title: &str, content: &str) -> Element<A> {
    Element::<A>::new()
        .attr("href", href)
        .attr("data-bs-toggle", "popover")
        .attr("data-bs-trigger", "hover focus")
        .attr("data-bs-title", title)
        .attr("data-bs-content", content)
        .text(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_button() {
        let btn = tooltip_button(
            crate::Color::Primary,
            "Hover",
            "Tooltip text",
            Placement::Top,
        );
        let html = btn.render();
        assert!(html.contains(r#"data-bs-toggle="tooltip"#));
        assert!(html.contains(r#"data-bs-placement="top"#));
        assert!(html.contains("data-bs-title"));
    }

    #[test]
    fn test_popover_button() {
        let btn = popover_button(
            crate::Color::Secondary,
            "Click",
            "Title",
            "Content",
            Placement::Right,
        );
        let html = btn.render();
        assert!(html.contains(r#"data-bs-toggle="popover"#));
        assert!(html.contains(r#"data-bs-placement="right"#));
        assert!(html.contains("data-bs-title"));
        assert!(html.contains("data-bs-content"));
    }

    #[test]
    fn test_tooltip_placements() {
        for placement in [
            Placement::Top,
            Placement::Right,
            Placement::Bottom,
            Placement::Left,
        ] {
            let btn = tooltip_button(crate::Color::Info, "Test", "Tip", placement);
            assert!(btn
                .render()
                .contains(&format!(r#"data-bs-placement="{}""#, placement.as_str())));
        }
    }
}
