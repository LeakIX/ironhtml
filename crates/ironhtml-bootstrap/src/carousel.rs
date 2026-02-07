//! Bootstrap carousel components.
//!
//! Provides type-safe Bootstrap carousels matching the
//! [Bootstrap carousel documentation](https://getbootstrap.com/docs/5.3/components/carousel/).

use ironhtml::typed::Element;
use ironhtml_elements::{Button, Div, Img, Span};

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

/// A carousel slide item.
pub struct CarouselItem {
    pub image_src: String,
    pub image_alt: String,
    pub caption_title: Option<String>,
    pub caption_text: Option<String>,
    pub active: bool,
}

impl CarouselItem {
    /// Create a new carousel item.
    #[must_use]
    pub fn new(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self {
            image_src: src.into(),
            image_alt: alt.into(),
            caption_title: None,
            caption_text: None,
            active: false,
        }
    }

    /// Mark this slide as active.
    #[must_use]
    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Add a caption to this slide.
    #[must_use]
    pub fn caption(mut self, title: impl Into<String>, text: impl Into<String>) -> Self {
        self.caption_title = Some(title.into());
        self.caption_text = Some(text.into());
        self
    }
}

/// Create a Bootstrap carousel.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::carousel::{carousel, CarouselItem};
///
/// let items = vec![
///     CarouselItem::new("/img/1.jpg", "First slide").active(),
///     CarouselItem::new("/img/2.jpg", "Second slide"),
///     CarouselItem::new("/img/3.jpg", "Third slide"),
/// ];
///
/// let c = carousel("myCarousel", &items);
/// assert!(c.render().contains("carousel"));
/// ```
#[must_use]
pub fn carousel(id: &str, items: &[CarouselItem]) -> Element<Div> {
    let target = format!("#{id}");

    Element::<Div>::new()
        .attr("id", id)
        .class("carousel slide")
        .child::<Div, _>(|inner| {
            items
                .iter()
                .fold(inner.class("carousel-inner"), |inner, item| {
                    inner.child::<Div, _>(|_| carousel_item(item))
                })
        })
        .child::<Button, _>(|b| {
            b.class("carousel-control-prev")
                .attr("type", "button")
                .attr("data-bs-target", &target)
                .attr("data-bs-slide", "prev")
                .child::<Span, _>(|s| {
                    s.class("carousel-control-prev-icon")
                        .attr("aria-hidden", "true")
                })
                .child::<Span, _>(|s| s.class("visually-hidden").text("Previous"))
        })
        .child::<Button, _>(|b| {
            b.class("carousel-control-next")
                .attr("type", "button")
                .attr("data-bs-target", &target)
                .attr("data-bs-slide", "next")
                .child::<Span, _>(|s| {
                    s.class("carousel-control-next-icon")
                        .attr("aria-hidden", "true")
                })
                .child::<Span, _>(|s| s.class("visually-hidden").text("Next"))
        })
}

/// Create a carousel with indicators.
#[must_use]
pub fn carousel_with_indicators(id: &str, items: &[CarouselItem]) -> Element<Div> {
    let target = format!("#{id}");

    Element::<Div>::new()
        .attr("id", id)
        .class("carousel slide")
        // Indicators
        .child::<Div, _>(|indicators| {
            items.iter().enumerate().fold(
                indicators.class("carousel-indicators"),
                |indicators, (i, item)| {
                    indicators.child::<Button, _>(|b| {
                        let mut btn = b
                            .attr("type", "button")
                            .attr("data-bs-target", &target)
                            .attr("data-bs-slide-to", i.to_string())
                            .attr("aria-label", format!("Slide {}", i + 1));
                        if item.active {
                            btn = btn.class("active").attr("aria-current", "true");
                        }
                        btn
                    })
                },
            )
        })
        // Slides
        .child::<Div, _>(|inner| {
            items
                .iter()
                .fold(inner.class("carousel-inner"), |inner, item| {
                    inner.child::<Div, _>(|_| carousel_item(item))
                })
        })
        // Controls
        .child::<Button, _>(|b| {
            b.class("carousel-control-prev")
                .attr("type", "button")
                .attr("data-bs-target", &target)
                .attr("data-bs-slide", "prev")
                .child::<Span, _>(|s| {
                    s.class("carousel-control-prev-icon")
                        .attr("aria-hidden", "true")
                })
                .child::<Span, _>(|s| s.class("visually-hidden").text("Previous"))
        })
        .child::<Button, _>(|b| {
            b.class("carousel-control-next")
                .attr("type", "button")
                .attr("data-bs-target", &target)
                .attr("data-bs-slide", "next")
                .child::<Span, _>(|s| {
                    s.class("carousel-control-next-icon")
                        .attr("aria-hidden", "true")
                })
                .child::<Span, _>(|s| s.class("visually-hidden").text("Next"))
        })
}

/// Create a carousel that autoplays.
#[must_use]
pub fn carousel_autoplay(id: &str, items: &[CarouselItem]) -> Element<Div> {
    carousel_with_indicators(id, items).attr("data-bs-ride", "carousel")
}

/// Create a single carousel item.
fn carousel_item(item: &CarouselItem) -> Element<Div> {
    use ironhtml_elements::{H5, P};

    let class = if item.active {
        "carousel-item active"
    } else {
        "carousel-item"
    };

    let mut elem = Element::<Div>::new().class(class).child::<Img, _>(|i| {
        i.class("d-block w-100")
            .attr("src", &item.image_src)
            .attr("alt", &item.image_alt)
    });

    if item.caption_title.is_some() || item.caption_text.is_some() {
        elem = elem.child::<Div, _>(|d| {
            let mut caption = d.class("carousel-caption d-none d-md-block");
            if let Some(ref title) = item.caption_title {
                caption = caption.child::<H5, _>(|h| h.text(title));
            }
            if let Some(ref text) = item.caption_text {
                caption = caption.child::<P, _>(|p| p.text(text));
            }
            caption
        });
    }

    elem
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_carousel() {
        let items = vec![
            CarouselItem::new("/1.jpg", "First").active(),
            CarouselItem::new("/2.jpg", "Second"),
        ];
        let c = carousel("test", &items);
        let html = c.render();
        assert!(html.contains("carousel"));
        assert!(html.contains("carousel-item active"));
        assert!(html.contains("carousel-control-prev"));
    }

    #[test]
    fn test_carousel_with_captions() {
        let items = vec![CarouselItem::new("/1.jpg", "First")
            .active()
            .caption("Title", "Description")];
        let c = carousel("test", &items);
        let html = c.render();
        assert!(html.contains("carousel-caption"));
        assert!(html.contains("Title"));
    }
}
