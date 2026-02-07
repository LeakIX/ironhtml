//! Bootstrap modal components.
//!
//! Provides type-safe Bootstrap modals matching the
//! [Bootstrap modal documentation](https://getbootstrap.com/docs/5.3/components/modal/).

use ironhtml::typed::Element;
use ironhtml_elements::{Button, Div, H1, H5};

extern crate alloc;
use alloc::format;
use alloc::string::ToString;

/// Modal size options.
#[derive(Clone, Copy, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Default,
    Large,
    ExtraLarge,
    Fullscreen,
}

impl ModalSize {
    const fn class(self) -> &'static str {
        match self {
            Self::Small => "modal-sm",
            Self::Default => "",
            Self::Large => "modal-lg",
            Self::ExtraLarge => "modal-xl",
            Self::Fullscreen => "modal-fullscreen",
        }
    }
}

/// Create a trigger button for a modal.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::modal::modal_button;
/// use ironhtml_bootstrap::Color;
///
/// let btn = modal_button("myModal", Color::Primary, "Launch modal");
/// assert!(btn.render().contains("data-bs-toggle"));
/// ```
#[must_use]
pub fn modal_button(target_id: &str, color: crate::Color, text: &str) -> Element<Button> {
    let class = format!("btn btn-{}", color.as_str());
    Element::<Button>::new()
        .attr("type", "button")
        .class(&class)
        .attr("data-bs-toggle", "modal")
        .attr("data-bs-target", format!("#{target_id}"))
        .text(text)
}

/// Create a basic modal structure.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::modal::{modal, ModalSize};
///
/// let m = modal("myModal", ModalSize::Default, "Modal Title", |body| {
///     body.text("Modal content goes here.")
/// });
/// assert!(m.render().contains("modal"));
/// ```
#[must_use]
pub fn modal<F>(id: &str, size: ModalSize, title: &str, body_fn: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let dialog_class = if size.class().is_empty() {
        "modal-dialog".to_string()
    } else {
        format!("modal-dialog {}", size.class())
    };

    Element::<Div>::new()
        .class("modal fade")
        .attr("id", id)
        .attr("tabindex", "-1")
        .attr("aria-labelledby", format!("{id}Label"))
        .attr("aria-hidden", "true")
        .child::<Div, _>(|d| {
            d.class(&dialog_class).child::<Div, _>(|content| {
                content
                    .class("modal-content")
                    // Header
                    .child::<Div, _>(|header| {
                        header
                            .class("modal-header")
                            .child::<H1, _>(|h| {
                                h.class("modal-title fs-5")
                                    .attr("id", format!("{id}Label"))
                                    .text(title)
                            })
                            .child::<Button, _>(|b| {
                                b.attr("type", "button")
                                    .class("btn-close")
                                    .attr("data-bs-dismiss", "modal")
                                    .attr("aria-label", "Close")
                            })
                    })
                    // Body
                    .child::<Div, _>(|body| body_fn(body.class("modal-body")))
            })
        })
}

/// Create a modal with footer buttons.
#[must_use]
pub fn modal_with_footer<F>(
    id: &str,
    size: ModalSize,
    title: &str,
    body_fn: F,
    primary_btn_text: &str,
) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let dialog_class = if size.class().is_empty() {
        "modal-dialog".to_string()
    } else {
        format!("modal-dialog {}", size.class())
    };

    Element::<Div>::new()
        .class("modal fade")
        .attr("id", id)
        .attr("tabindex", "-1")
        .attr("aria-labelledby", format!("{id}Label"))
        .attr("aria-hidden", "true")
        .child::<Div, _>(|d| {
            d.class(&dialog_class).child::<Div, _>(|content| {
                content
                    .class("modal-content")
                    // Header
                    .child::<Div, _>(|header| {
                        header
                            .class("modal-header")
                            .child::<H5, _>(|h| {
                                h.class("modal-title")
                                    .attr("id", format!("{id}Label"))
                                    .text(title)
                            })
                            .child::<Button, _>(|b| {
                                b.attr("type", "button")
                                    .class("btn-close")
                                    .attr("data-bs-dismiss", "modal")
                                    .attr("aria-label", "Close")
                            })
                    })
                    // Body
                    .child::<Div, _>(|body| body_fn(body.class("modal-body")))
                    // Footer
                    .child::<Div, _>(|footer| {
                        footer
                            .class("modal-footer")
                            .child::<Button, _>(|b| {
                                b.attr("type", "button")
                                    .class("btn btn-secondary")
                                    .attr("data-bs-dismiss", "modal")
                                    .text("Close")
                            })
                            .child::<Button, _>(|b| {
                                b.attr("type", "button")
                                    .class("btn btn-primary")
                                    .text(primary_btn_text)
                            })
                    })
            })
        })
}

/// Create a scrollable modal.
#[must_use]
pub fn modal_scrollable<F>(id: &str, title: &str, body_fn: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    Element::<Div>::new()
        .class("modal fade")
        .attr("id", id)
        .attr("tabindex", "-1")
        .attr("aria-labelledby", format!("{id}Label"))
        .attr("aria-hidden", "true")
        .child::<Div, _>(|d| {
            d.class("modal-dialog modal-dialog-scrollable")
                .child::<Div, _>(|content| {
                    content
                        .class("modal-content")
                        .child::<Div, _>(|header| {
                            header
                                .class("modal-header")
                                .child::<H5, _>(|h| {
                                    h.class("modal-title")
                                        .attr("id", format!("{id}Label"))
                                        .text(title)
                                })
                                .child::<Button, _>(|b| {
                                    b.attr("type", "button")
                                        .class("btn-close")
                                        .attr("data-bs-dismiss", "modal")
                                        .attr("aria-label", "Close")
                                })
                        })
                        .child::<Div, _>(|body| body_fn(body.class("modal-body")))
                })
        })
}

/// Create a vertically centered modal.
#[must_use]
pub fn modal_centered<F>(id: &str, title: &str, body_fn: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    Element::<Div>::new()
        .class("modal fade")
        .attr("id", id)
        .attr("tabindex", "-1")
        .attr("aria-labelledby", format!("{id}Label"))
        .attr("aria-hidden", "true")
        .child::<Div, _>(|d| {
            d.class("modal-dialog modal-dialog-centered")
                .child::<Div, _>(|content| {
                    content
                        .class("modal-content")
                        .child::<Div, _>(|header| {
                            header
                                .class("modal-header")
                                .child::<H5, _>(|h| {
                                    h.class("modal-title")
                                        .attr("id", format!("{id}Label"))
                                        .text(title)
                                })
                                .child::<Button, _>(|b| {
                                    b.attr("type", "button")
                                        .class("btn-close")
                                        .attr("data-bs-dismiss", "modal")
                                        .attr("aria-label", "Close")
                                })
                        })
                        .child::<Div, _>(|body| body_fn(body.class("modal-body")))
                })
        })
}

/// Create a static backdrop modal (won't close when clicking outside).
#[must_use]
pub fn modal_static<F>(id: &str, title: &str, body_fn: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    Element::<Div>::new()
        .class("modal fade")
        .attr("id", id)
        .attr("data-bs-backdrop", "static")
        .attr("data-bs-keyboard", "false")
        .attr("tabindex", "-1")
        .attr("aria-labelledby", format!("{id}Label"))
        .attr("aria-hidden", "true")
        .child::<Div, _>(|d| {
            d.class("modal-dialog").child::<Div, _>(|content| {
                content
                    .class("modal-content")
                    .child::<Div, _>(|header| {
                        header
                            .class("modal-header")
                            .child::<H5, _>(|h| {
                                h.class("modal-title")
                                    .attr("id", format!("{id}Label"))
                                    .text(title)
                            })
                            .child::<Button, _>(|b| {
                                b.attr("type", "button")
                                    .class("btn-close")
                                    .attr("data-bs-dismiss", "modal")
                                    .attr("aria-label", "Close")
                            })
                    })
                    .child::<Div, _>(|body| body_fn(body.class("modal-body")))
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal() {
        let m = modal("test", ModalSize::Default, "Title", |body| {
            body.text("Content")
        });
        let html = m.render();
        assert!(html.contains("modal fade"));
        assert!(html.contains("modal-dialog"));
        assert!(html.contains("modal-content"));
        assert!(html.contains("modal-header"));
        assert!(html.contains("modal-body"));
    }

    #[test]
    fn test_modal_sizes() {
        let small = modal("sm", ModalSize::Small, "Small", |b| b);
        assert!(small.render().contains("modal-sm"));

        let large = modal("lg", ModalSize::Large, "Large", |b| b);
        assert!(large.render().contains("modal-lg"));

        let xl = modal("xl", ModalSize::ExtraLarge, "XL", |b| b);
        assert!(xl.render().contains("modal-xl"));
    }

    #[test]
    fn test_modal_button() {
        let btn = modal_button("myModal", crate::Color::Primary, "Open");
        let html = btn.render();
        assert!(html.contains(r#"data-bs-toggle="modal""#));
        assert!(html.contains(r##"data-bs-target="#myModal""##));
    }
}
