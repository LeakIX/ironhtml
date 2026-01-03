//! Bootstrap alert components.
//!
//! Provides type-safe Bootstrap alerts matching the
//! [Bootstrap alerts documentation](https://getbootstrap.com/docs/5.3/components/alerts/).
//!
//! ## Example
//!
//! ```rust
//! use html_bootstrap::{alerts::*, Color};
//!
//! // Simple alert
//! let warning = alert(Color::Warning, "This is a warning!");
//! assert!(warning.render().contains(r#"class="alert alert-warning"#));
//! assert!(warning.render().contains(r#"role="alert"#));
//!
//! // Dismissible alert
//! let dismissible = alert_dismissible(Color::Danger, "Error occurred!");
//! assert!(dismissible.render().contains("alert-dismissible"));
//! assert!(dismissible.render().contains("btn-close"));
//! ```

use crate::Color;
use html_builder::typed::Element;
use html_elements::{Button, Div};

extern crate alloc;
use alloc::format;

/// Create a simple Bootstrap alert.
///
/// Generates:
/// ```html
/// <div class="alert alert-{color}" role="alert">{text}</div>
/// ```
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::{alerts::alert, Color};
///
/// let a = alert(Color::Primary, "A simple primary alert");
/// assert!(a.render().contains(r#"alert-primary"#));
/// ```
pub fn alert(color: Color, text: &str) -> Element<Div> {
    let class = format!("alert alert-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .attr("role", "alert")
        .text(text)
}

/// Create an alert with custom content.
///
/// Allows adding child elements like links, headings, etc.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::{alerts::alert_with, Color};
/// use html_elements::{A, Strong};
///
/// let a = alert_with(Color::Info, |div| {
///     div.child::<Strong, _>(|s| s.text("Note: "))
///        .text("Check ")
///        .child::<A, _>(|a| a.class("alert-link").attr("href", "#").text("this link"))
/// });
/// assert!(a.render().contains("alert-info"));
/// assert!(a.render().contains("alert-link"));
/// ```
pub fn alert_with<F>(color: Color, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("alert alert-{}", color.as_str());
    f(Element::<Div>::new().class(&class).attr("role", "alert"))
}

/// Create a dismissible Bootstrap alert.
///
/// Generates:
/// ```html
/// <div class="alert alert-{color} alert-dismissible fade show" role="alert">
///   {text}
///   <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
/// </div>
/// ```
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::{alerts::alert_dismissible, Color};
///
/// let a = alert_dismissible(Color::Warning, "Holy guacamole!");
/// let html = a.render();
/// assert!(html.contains("alert-dismissible"));
/// assert!(html.contains("fade show"));
/// assert!(html.contains("btn-close"));
/// assert!(html.contains(r#"data-bs-dismiss="alert"#));
/// ```
pub fn alert_dismissible(color: Color, text: &str) -> Element<Div> {
    let class = format!("alert alert-{} alert-dismissible fade show", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .attr("role", "alert")
        .text(text)
        .child::<Button, _>(|btn| {
            btn.attr("type", "button")
                .class("btn-close")
                .attr("data-bs-dismiss", "alert")
                .attr("aria-label", "Close")
        })
}

/// Create an alert with heading (from Bootstrap docs).
///
/// Generates:
/// ```html
/// <div class="alert alert-{color}" role="alert">
///   <h4 class="alert-heading">{heading}</h4>
///   <p>{text}</p>
///   <hr>
///   <p class="mb-0">{footer}</p>
/// </div>
/// ```
pub fn alert_with_heading(color: Color, heading: &str, text: &str, footer: &str) -> Element<Div> {
    use html_elements::{Hr, H4, P};

    let class = format!("alert alert-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .attr("role", "alert")
        .child::<H4, _>(|h| h.class("alert-heading").text(heading))
        .child::<P, _>(|p| p.text(text))
        .child::<Hr, _>(|hr| hr)
        .child::<P, _>(|p| p.class("mb-0").text(footer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_alert() {
        let a = alert(Color::Success, "Well done!");
        let html = a.render();
        assert!(html.contains(r#"class="alert alert-success"#));
        assert!(html.contains(r#"role="alert"#));
        assert!(html.contains("Well done!"));
    }

    #[test]
    fn test_dismissible_alert() {
        let a = alert_dismissible(Color::Warning, "Warning!");
        let html = a.render();
        assert!(html.contains("alert-dismissible"));
        assert!(html.contains("fade show"));
        assert!(html.contains("btn-close"));
        assert!(html.contains(r#"data-bs-dismiss="alert"#));
    }

    #[test]
    fn test_alert_with_heading() {
        let a = alert_with_heading(Color::Success, "Well done!", "Content here", "Footer");
        let html = a.render();
        assert!(html.contains("alert-heading"));
        assert!(html.contains("Well done!"));
        assert!(html.contains("<hr"));
        assert!(html.contains("mb-0"));
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
            let a = alert(color, "Test");
            assert!(a.render().contains(&format!("alert-{}", color.as_str())));
        }
    }
}
