//! Bootstrap progress bar components.
//!
//! Provides type-safe Bootstrap progress bars matching the
//! [Bootstrap progress documentation](https://getbootstrap.com/docs/5.3/components/progress/).

use crate::Color;
use html_builder::typed::Element;
use html_elements::Div;

extern crate alloc;
use alloc::format;
use alloc::string::ToString;

/// Create a Bootstrap progress bar.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::progress::progress;
///
/// let bar = progress(75);
/// assert!(bar.render().contains("progress"));
/// assert!(bar.render().contains("75%"));
/// ```
pub fn progress(percent: u8) -> Element<Div> {
    let percent = percent.min(100);
    let width = format!("width: {}%", percent);
    let label = format!("{}%", percent);

    Element::<Div>::new()
        .class("progress")
        .attr("role", "progressbar")
        .attr("aria-valuenow", percent.to_string())
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .child::<Div, _>(|bar| bar.class("progress-bar").attr("style", &width).text(&label))
}

/// Create a progress bar without label.
pub fn progress_silent(percent: u8) -> Element<Div> {
    let percent = percent.min(100);
    let width = format!("width: {}%", percent);

    Element::<Div>::new()
        .class("progress")
        .attr("role", "progressbar")
        .attr("aria-valuenow", percent.to_string())
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .child::<Div, _>(|bar| bar.class("progress-bar").attr("style", &width))
}

/// Create a colored progress bar.
pub fn progress_colored(percent: u8, color: Color) -> Element<Div> {
    let percent = percent.min(100);
    let width = format!("width: {}%", percent);
    let bar_class = format!("progress-bar bg-{}", color.as_str());

    Element::<Div>::new()
        .class("progress")
        .attr("role", "progressbar")
        .attr("aria-valuenow", percent.to_string())
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .child::<Div, _>(|bar| bar.class(&bar_class).attr("style", &width))
}

/// Create a striped progress bar.
pub fn progress_striped(percent: u8, color: Color) -> Element<Div> {
    let percent = percent.min(100);
    let width = format!("width: {}%", percent);
    let bar_class = format!("progress-bar progress-bar-striped bg-{}", color.as_str());

    Element::<Div>::new()
        .class("progress")
        .attr("role", "progressbar")
        .attr("aria-valuenow", percent.to_string())
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .child::<Div, _>(|bar| bar.class(&bar_class).attr("style", &width))
}

/// Create an animated striped progress bar.
pub fn progress_animated(percent: u8, color: Color) -> Element<Div> {
    let percent = percent.min(100);
    let width = format!("width: {}%", percent);
    let bar_class = format!(
        "progress-bar progress-bar-striped progress-bar-animated bg-{}",
        color.as_str()
    );

    Element::<Div>::new()
        .class("progress")
        .attr("role", "progressbar")
        .attr("aria-valuenow", percent.to_string())
        .attr("aria-valuemin", "0")
        .attr("aria-valuemax", "100")
        .child::<Div, _>(|bar| bar.class(&bar_class).attr("style", &width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress() {
        let bar = progress(50);
        let html = bar.render();
        assert!(html.contains("progress"));
        assert!(html.contains("progress-bar"));
        assert!(html.contains("50%"));
    }

    #[test]
    fn test_progress_colored() {
        let bar = progress_colored(75, Color::Success);
        assert!(bar.render().contains("bg-success"));
    }

    #[test]
    fn test_progress_striped() {
        let bar = progress_striped(60, Color::Info);
        assert!(bar.render().contains("progress-bar-striped"));
    }
}
