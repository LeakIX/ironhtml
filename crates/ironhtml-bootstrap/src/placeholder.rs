//! Bootstrap placeholder components.
//!
//! Provides type-safe Bootstrap placeholders matching the
//! [Bootstrap placeholders documentation](https://getbootstrap.com/docs/5.3/components/placeholders/).

use ironhtml::typed::Element;
use ironhtml_elements::{Div, Span, A, P};

extern crate alloc;
use alloc::format;

/// Placeholder width options.
#[derive(Clone, Copy)]
pub enum PlaceholderWidth {
    Col1,
    Col2,
    Col3,
    Col4,
    Col5,
    Col6,
    Col7,
    Col8,
    Col9,
    Col10,
    Col11,
    Col12,
}

impl PlaceholderWidth {
    const fn class(self) -> &'static str {
        match self {
            Self::Col1 => "col-1",
            Self::Col2 => "col-2",
            Self::Col3 => "col-3",
            Self::Col4 => "col-4",
            Self::Col5 => "col-5",
            Self::Col6 => "col-6",
            Self::Col7 => "col-7",
            Self::Col8 => "col-8",
            Self::Col9 => "col-9",
            Self::Col10 => "col-10",
            Self::Col11 => "col-11",
            Self::Col12 => "col-12",
        }
    }
}

/// Placeholder size options.
#[derive(Clone, Copy, Default)]
pub enum PlaceholderSize {
    ExtraSmall,
    Small,
    #[default]
    Default,
    Large,
}

impl PlaceholderSize {
    const fn class(self) -> &'static str {
        match self {
            Self::ExtraSmall => "placeholder-xs",
            Self::Small => "placeholder-sm",
            Self::Default => "",
            Self::Large => "placeholder-lg",
        }
    }
}

/// Create a placeholder span.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::placeholder::{placeholder, PlaceholderWidth};
///
/// let p = placeholder(PlaceholderWidth::Col6);
/// assert!(p.render().contains("placeholder"));
/// ```
#[must_use]
pub fn placeholder(width: PlaceholderWidth) -> Element<Span> {
    let class = format!("placeholder {}", width.class());
    Element::<Span>::new().class(&class)
}

/// Create a sized placeholder.
#[must_use]
pub fn placeholder_sized(width: PlaceholderWidth, size: PlaceholderSize) -> Element<Span> {
    let size_class = size.class();
    let class = if size_class.is_empty() {
        format!("placeholder {}", width.class())
    } else {
        format!("placeholder {} {size_class}", width.class())
    };
    Element::<Span>::new().class(&class)
}

/// Create a colored placeholder.
#[must_use]
pub fn placeholder_colored(width: PlaceholderWidth, color: crate::Color) -> Element<Span> {
    let class = format!("placeholder {} bg-{}", width.class(), color.as_str());
    Element::<Span>::new().class(&class)
}

/// Create a placeholder paragraph (loading text simulation).
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::placeholder::placeholder_paragraph;
///
/// let p = placeholder_paragraph();
/// assert!(p.render().contains("placeholder-glow"));
/// ```
#[must_use]
pub fn placeholder_paragraph() -> Element<P> {
    Element::<P>::new()
        .class("placeholder-glow")
        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col7))
        .text(" ")
        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col4))
        .text(" ")
        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col4))
        .text(" ")
        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col6))
        .text(" ")
        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col8))
}

/// Create a placeholder with glow animation.
#[must_use]
pub fn placeholder_glow<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("placeholder-glow"))
}

/// Create a placeholder with wave animation.
#[must_use]
pub fn placeholder_wave<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("placeholder-wave"))
}

/// Create a placeholder button.
#[must_use]
pub fn placeholder_button(color: crate::Color, width: PlaceholderWidth) -> Element<A> {
    let class = format!(
        "btn btn-{} disabled placeholder {}",
        color.as_str(),
        width.class()
    );
    Element::<A>::new()
        .class(&class)
        .attr("aria-disabled", "true")
}

/// Create a loading card placeholder (matches Bootstrap docs example).
#[must_use]
pub fn placeholder_card() -> Element<Div> {
    use ironhtml_elements::{Img, H5};

    Element::<Div>::new()
        .class("card")
        .attr("aria-hidden", "true")
        .child::<Img, _>(|i| {
            i.attr("src", "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 286 180'%3E%3Crect fill='%23868e96' width='286' height='180'/%3E%3C/svg%3E")
                .class("card-img-top")
                .attr("alt", "")
        })
        .child::<Div, _>(|body| {
            body.class("card-body")
                .child::<H5, _>(|h| {
                    h.class("card-title placeholder-glow")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col6))
                })
                .child::<P, _>(|p| {
                    p.class("card-text placeholder-glow")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col7))
                        .text(" ")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col4))
                        .text(" ")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col4))
                        .text(" ")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col6))
                        .text(" ")
                        .child::<Span, _>(|_| placeholder(PlaceholderWidth::Col8))
                })
                .child::<A, _>(|_| placeholder_button(crate::Color::Primary, PlaceholderWidth::Col6))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        let p = placeholder(PlaceholderWidth::Col6);
        let html = p.render();
        assert!(html.contains("placeholder"));
        assert!(html.contains("col-6"));
    }

    #[test]
    fn test_placeholder_sized() {
        let p = placeholder_sized(PlaceholderWidth::Col4, PlaceholderSize::Large);
        let html = p.render();
        assert!(html.contains("placeholder-lg"));
    }

    #[test]
    fn test_placeholder_colored() {
        let p = placeholder_colored(PlaceholderWidth::Col3, crate::Color::Primary);
        let html = p.render();
        assert!(html.contains("bg-primary"));
    }

    #[test]
    fn test_placeholder_paragraph() {
        let p = placeholder_paragraph();
        let html = p.render();
        assert!(html.contains("placeholder-glow"));
        assert!(html.contains("placeholder"));
    }

    #[test]
    fn test_placeholder_card() {
        let card = placeholder_card();
        let html = card.render();
        assert!(html.contains("card"));
        assert!(html.contains("placeholder-glow"));
    }
}
