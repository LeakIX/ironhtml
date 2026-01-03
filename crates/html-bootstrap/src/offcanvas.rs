//! Bootstrap offcanvas components.
//!
//! Provides type-safe Bootstrap offcanvas matching the
//! [Bootstrap offcanvas documentation](https://getbootstrap.com/docs/5.3/components/offcanvas/).

use html_builder::typed::Element;
use html_elements::{Button, Div, A, H5};

extern crate alloc;
use alloc::format;

/// Offcanvas placement options.
#[derive(Clone, Copy, Default)]
pub enum OffcanvasPlacement {
    #[default]
    Start,
    End,
    Top,
    Bottom,
}

impl OffcanvasPlacement {
    fn class(&self) -> &'static str {
        match self {
            OffcanvasPlacement::Start => "offcanvas-start",
            OffcanvasPlacement::End => "offcanvas-end",
            OffcanvasPlacement::Top => "offcanvas-top",
            OffcanvasPlacement::Bottom => "offcanvas-bottom",
        }
    }
}

/// Create a trigger button for an offcanvas.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::offcanvas::offcanvas_button;
/// use html_bootstrap::Color;
///
/// let btn = offcanvas_button("offcanvasExample", Color::Primary, "Toggle sidebar");
/// assert!(btn.render().contains("data-bs-toggle"));
/// ```
pub fn offcanvas_button(target_id: &str, color: crate::Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .class(&class)
        .attr("type", "button")
        .attr("data-bs-toggle", "offcanvas")
        .attr("data-bs-target", format!("#{}", target_id))
        .attr("aria-controls", target_id)
        .text(text)
}

/// Create a trigger link for an offcanvas.
pub fn offcanvas_link(target_id: &str, text: &str) -> Element<A> {
    Element::<A>::new()
        .class("btn btn-primary")
        .attr("data-bs-toggle", "offcanvas")
        .attr("href", format!("#{}", target_id))
        .attr("role", "button")
        .attr("aria-controls", target_id)
        .text(text)
}

/// Create a Bootstrap offcanvas.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::offcanvas::{offcanvas, OffcanvasPlacement};
///
/// let oc = offcanvas("sidebar", OffcanvasPlacement::Start, "Sidebar", |body| {
///     body.text("Sidebar content here")
/// });
/// assert!(oc.render().contains("offcanvas"));
/// ```
pub fn offcanvas<F>(
    id: &str,
    placement: OffcanvasPlacement,
    title: &str,
    body_fn: F,
) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("offcanvas {}", placement.class());

    Element::<Div>::new()
        .class(&class)
        .attr("tabindex", "-1")
        .attr("id", id)
        .attr("aria-labelledby", format!("{}Label", id))
        // Header
        .child::<Div, _>(|header| {
            header
                .class("offcanvas-header")
                .child::<H5, _>(|h| {
                    h.class("offcanvas-title")
                        .attr("id", format!("{}Label", id))
                        .text(title)
                })
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "offcanvas")
                        .attr("aria-label", "Close")
                })
        })
        // Body
        .child::<Div, _>(|body| body_fn(body.class("offcanvas-body")))
}

/// Create an offcanvas with static backdrop (doesn't close on outside click).
pub fn offcanvas_static<F>(
    id: &str,
    placement: OffcanvasPlacement,
    title: &str,
    body_fn: F,
) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("offcanvas {}", placement.class());

    Element::<Div>::new()
        .class(&class)
        .attr("data-bs-backdrop", "static")
        .attr("tabindex", "-1")
        .attr("id", id)
        .attr("aria-labelledby", format!("{}Label", id))
        .child::<Div, _>(|header| {
            header
                .class("offcanvas-header")
                .child::<H5, _>(|h| {
                    h.class("offcanvas-title")
                        .attr("id", format!("{}Label", id))
                        .text(title)
                })
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "offcanvas")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body_fn(body.class("offcanvas-body")))
}

/// Create an offcanvas with body scrolling enabled.
pub fn offcanvas_scroll<F>(
    id: &str,
    placement: OffcanvasPlacement,
    title: &str,
    body_fn: F,
) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("offcanvas {}", placement.class());

    Element::<Div>::new()
        .class(&class)
        .attr("data-bs-scroll", "true")
        .attr("data-bs-backdrop", "false")
        .attr("tabindex", "-1")
        .attr("id", id)
        .attr("aria-labelledby", format!("{}Label", id))
        .child::<Div, _>(|header| {
            header
                .class("offcanvas-header")
                .child::<H5, _>(|h| {
                    h.class("offcanvas-title")
                        .attr("id", format!("{}Label", id))
                        .text(title)
                })
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close")
                        .attr("data-bs-dismiss", "offcanvas")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body_fn(body.class("offcanvas-body")))
}

/// Create a dark-themed offcanvas.
pub fn offcanvas_dark<F>(
    id: &str,
    placement: OffcanvasPlacement,
    title: &str,
    body_fn: F,
) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("offcanvas {} text-bg-dark", placement.class());

    Element::<Div>::new()
        .class(&class)
        .attr("tabindex", "-1")
        .attr("id", id)
        .attr("aria-labelledby", format!("{}Label", id))
        .child::<Div, _>(|header| {
            header
                .class("offcanvas-header")
                .child::<H5, _>(|h| {
                    h.class("offcanvas-title")
                        .attr("id", format!("{}Label", id))
                        .text(title)
                })
                .child::<Button, _>(|b| {
                    b.attr("type", "button")
                        .class("btn-close btn-close-white")
                        .attr("data-bs-dismiss", "offcanvas")
                        .attr("aria-label", "Close")
                })
        })
        .child::<Div, _>(|body| body_fn(body.class("offcanvas-body")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offcanvas() {
        let oc = offcanvas("test", OffcanvasPlacement::Start, "Title", |b| {
            b.text("Content")
        });
        let html = oc.render();
        assert!(html.contains("offcanvas"));
        assert!(html.contains("offcanvas-start"));
        assert!(html.contains("offcanvas-header"));
        assert!(html.contains("offcanvas-body"));
    }

    #[test]
    fn test_offcanvas_placements() {
        let end = offcanvas("e", OffcanvasPlacement::End, "T", |b| b);
        assert!(end.render().contains("offcanvas-end"));

        let top = offcanvas("t", OffcanvasPlacement::Top, "T", |b| b);
        assert!(top.render().contains("offcanvas-top"));

        let bottom = offcanvas("b", OffcanvasPlacement::Bottom, "T", |b| b);
        assert!(bottom.render().contains("offcanvas-bottom"));
    }

    #[test]
    fn test_offcanvas_button() {
        let btn = offcanvas_button("sidebar", crate::Color::Primary, "Open");
        let html = btn.render();
        assert!(html.contains(r#"data-bs-toggle="offcanvas"#));
        assert!(html.contains(r##"data-bs-target="#sidebar""##));
    }
}
