//! Bootstrap toast components.
//!
//! Provides type-safe Bootstrap toasts matching the
//! [Bootstrap toast documentation](https://getbootstrap.com/docs/5.3/components/toasts/).

use html_builder::typed::Element;
use html_elements::{Button, Div, Img, Small, Strong};

extern crate alloc;
use alloc::format;
use alloc::string::ToString;

/// Create a basic Bootstrap toast.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::toast::toast;
///
/// let t = toast("Hello, world!", "11 mins ago");
/// assert!(t.render().contains("toast"));
/// ```
pub fn toast(message: &str, time: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("toast")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|header| {
            header
                .class("toast-header")
                .child::<Strong, _>(|s| s.class("me-auto").text("Bootstrap"))
                .child::<Small, _>(|s| s.text(time))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body.class("toast-body").text(message))
}

/// Create a toast with custom header title.
pub fn toast_titled(title: &str, message: &str, time: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("toast")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|header| {
            header
                .class("toast-header")
                .child::<Strong, _>(|s| s.class("me-auto").text(title))
                .child::<Small, _>(|s| s.class("text-body-secondary").text(time))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body.class("toast-body").text(message))
}

/// Create a toast with image in header.
pub fn toast_with_image(
    img_src: &str,
    img_alt: &str,
    title: &str,
    message: &str,
    time: &str,
) -> Element<Div> {
    Element::<Div>::new()
        .class("toast")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|header| {
            header
                .class("toast-header")
                .child::<Img, _>(|i| {
                    i.attr("src", img_src)
                        .attr("alt", img_alt)
                        .class("rounded me-2")
                        .attr("style", "width: 20px; height: 20px;")
                })
                .child::<Strong, _>(|s| s.class("me-auto").text(title))
                .child::<Small, _>(|s| s.text(time))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body.class("toast-body").text(message))
}

/// Create a simple toast without header.
pub fn toast_simple(message: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("toast align-items-center")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|d| {
            d.class("d-flex")
                .child::<Div, _>(|body| body.class("toast-body").text(message))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close me-2 m-auto")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
}

/// Create a colored toast.
pub fn toast_colored(color: crate::Color, message: &str) -> Element<Div> {
    let class = format!(
        "toast align-items-center text-bg-{} border-0",
        color.as_str()
    );

    Element::<Div>::new()
        .class(&class)
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|d| {
            d.class("d-flex")
                .child::<Div, _>(|body| body.class("toast-body").text(message))
                .child::<Button, _>(|b| {
                    let btn_class = if matches!(color, crate::Color::Light) {
                        "btn-close me-2 m-auto"
                    } else {
                        "btn-close btn-close-white me-2 m-auto"
                    };
                    b.attr("type", "button")
                        .class(btn_class)
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
}

/// Create a toast container for stacking multiple toasts.
///
/// Position classes: top-0/bottom-0, start-0/end-0, translate-middle
pub fn toast_container<F>(position_class: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("toast-container position-fixed p-3 {}", position_class);
    f(Element::<Div>::new().class(&class))
}

/// Create a toast that auto-hides after delay.
pub fn toast_autohide(message: &str, delay_ms: u32) -> Element<Div> {
    Element::<Div>::new()
        .class("toast")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .attr("data-bs-autohide", "true")
        .attr("data-bs-delay", delay_ms.to_string())
        .child::<Div, _>(|header| {
            header
                .class("toast-header")
                .child::<Strong, _>(|s| s.class("me-auto").text("Notification"))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body.class("toast-body").text(message))
}

/// Create a toast that is shown by default.
pub fn toast_show(message: &str, time: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("toast show")
        .attr("role", "alert")
        .attr("aria-live", "assertive")
        .attr("aria-atomic", "true")
        .child::<Div, _>(|header| {
            header
                .class("toast-header")
                .child::<Strong, _>(|s| s.class("me-auto").text("Bootstrap"))
                .child::<Small, _>(|s| s.text(time))
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "toast")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body.class("toast-body").text(message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast() {
        let t = toast("Hello!", "just now");
        let html = t.render();
        assert!(html.contains("toast"));
        assert!(html.contains("toast-header"));
        assert!(html.contains("toast-body"));
        assert!(html.contains("Hello!"));
    }

    #[test]
    fn test_toast_colored() {
        let t = toast_colored(crate::Color::Success, "Success!");
        let html = t.render();
        assert!(html.contains("text-bg-success"));
        assert!(html.contains("btn-close-white"));
    }

    #[test]
    fn test_toast_simple() {
        let t = toast_simple("Simple message");
        let html = t.render();
        assert!(html.contains("toast"));
        assert!(!html.contains("toast-header"));
    }

    #[test]
    fn test_toast_container() {
        let container = toast_container("top-0 end-0", |c| {
            c.child::<Div, _>(|_| toast_show("Message 1", "now"))
                .child::<Div, _>(|_| toast_show("Message 2", "now"))
        });
        let html = container.render();
        assert!(html.contains("toast-container"));
        assert!(html.contains("position-fixed"));
    }
}
