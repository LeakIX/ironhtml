//! Bootstrap close button component.
//!
//! Provides type-safe Bootstrap close buttons matching the
//! [Bootstrap close button documentation](https://getbootstrap.com/docs/5.3/components/close-button/).

use html_builder::typed::Element;
use html_elements::Button;

/// Create a Bootstrap close button.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::close_button::close_button;
///
/// let btn = close_button();
/// assert!(btn.render().contains("btn-close"));
/// ```
pub fn close_button() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("aria-label", "Close")
}

/// Create a disabled close button.
pub fn close_button_disabled() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("aria-label", "Close")
        .bool_attr("disabled")
}

/// Create a white (inverted) close button for dark backgrounds.
pub fn close_button_white() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close btn-close-white")
        .attr("aria-label", "Close")
}

/// Create a close button that dismisses an alert.
pub fn close_button_dismiss_alert() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("data-bs-dismiss", "alert")
        .attr("aria-label", "Close")
}

/// Create a close button that dismisses a modal.
pub fn close_button_dismiss_modal() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("data-bs-dismiss", "modal")
        .attr("aria-label", "Close")
}

/// Create a close button that dismisses an offcanvas.
pub fn close_button_dismiss_offcanvas() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("data-bs-dismiss", "offcanvas")
        .attr("aria-label", "Close")
}

/// Create a close button that dismisses a toast.
pub fn close_button_dismiss_toast() -> Element<Button> {
    Element::<Button>::new()
        .attr("type", "button")
        .class("btn-close")
        .attr("data-bs-dismiss", "toast")
        .attr("aria-label", "Close")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_button() {
        let btn = close_button();
        let html = btn.render();
        assert!(html.contains("btn-close"));
        assert!(html.contains(r#"aria-label="Close"#));
    }

    #[test]
    fn test_close_button_disabled() {
        let btn = close_button_disabled();
        assert!(btn.render().contains("disabled"));
    }

    #[test]
    fn test_close_button_white() {
        let btn = close_button_white();
        assert!(btn.render().contains("btn-close-white"));
    }

    #[test]
    fn test_close_button_dismiss() {
        let btn = close_button_dismiss_modal();
        assert!(btn.render().contains(r#"data-bs-dismiss="modal"#));
    }
}
