//! Bootstrap card components.
//!
//! Provides type-safe Bootstrap cards matching the
//! [Bootstrap cards documentation](https://getbootstrap.com/docs/5.3/components/card/).
//!
//! ## Example
//!
//! ```rust
//! use html_bootstrap::cards::*;
//!
//! // Simple card
//! let simple = card(|c| c.text("Card content"));
//! assert!(simple.render().contains(r#"class="card"#));
//!
//! // Card with title and text (from Bootstrap docs)
//! let doc_card = card_simple("Card title", "Some text", "Go", "#");
//! assert!(doc_card.render().contains("card-title"));
//! assert!(doc_card.render().contains("card-text"));
//! ```

use crate::Color;
use html_builder::typed::Element;
use html_elements::{Div, Img, A, H5, P};

extern crate alloc;
use alloc::format;

/// Create a basic Bootstrap card.
///
/// Generates:
/// ```html
/// <div class="card">
///   <div class="card-body">...</div>
/// </div>
/// ```
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::cards::card;
///
/// let c = card(|body| body.text("Card content"));
/// assert!(c.render().contains(r#"class="card"#));
/// assert!(c.render().contains(r#"class="card-body"#));
/// ```
pub fn card<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    Element::<Div>::new()
        .class("card")
        .child::<Div, _>(|body| f(body.class("card-body")))
}

/// Create a card with specific width.
pub fn card_width<F>(width: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let style = format!("width: {};", width);
    Element::<Div>::new()
        .class("card")
        .attr("style", &style)
        .child::<Div, _>(|body| f(body.class("card-body")))
}

/// Create a card with title and text (from Bootstrap docs example).
///
/// Generates:
/// ```html
/// <div class="card" style="width: 18rem;">
///   <div class="card-body">
///     <h5 class="card-title">Card title</h5>
///     <p class="card-text">Some quick example text...</p>
///     <a href="#" class="btn btn-primary">Go somewhere</a>
///   </div>
/// </div>
/// ```
pub fn card_simple(title: &str, text: &str, link_text: &str, link_href: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("card")
        .attr("style", "width: 18rem;")
        .child::<Div, _>(|body| {
            body.class("card-body")
                .child::<H5, _>(|h| h.class("card-title").text(title))
                .child::<P, _>(|p| p.class("card-text").text(text))
                .child::<A, _>(|a| {
                    a.attr("href", link_href)
                        .class("btn btn-primary")
                        .text(link_text)
                })
        })
}

/// Create a card with image on top (from Bootstrap docs).
///
/// Generates:
/// ```html
/// <div class="card" style="width: 18rem;">
///   <img src="..." class="card-img-top" alt="...">
///   <div class="card-body">
///     <h5 class="card-title">Card title</h5>
///     <p class="card-text">Some text...</p>
///   </div>
/// </div>
/// ```
pub fn card_with_image(img_src: &str, img_alt: &str, title: &str, text: &str) -> Element<Div> {
    Element::<Div>::new()
        .class("card")
        .attr("style", "width: 18rem;")
        .child::<Img, _>(|img| {
            img.attr("src", img_src)
                .class("card-img-top")
                .attr("alt", img_alt)
        })
        .child::<Div, _>(|body| {
            body.class("card-body")
                .child::<H5, _>(|h| h.class("card-title").text(title))
                .child::<P, _>(|p| p.class("card-text").text(text))
        })
}

/// Create a card with header and footer.
///
/// Generates:
/// ```html
/// <div class="card">
///   <div class="card-header">{header}</div>
///   <div class="card-body">...</div>
///   <div class="card-footer text-body-secondary">{footer}</div>
/// </div>
/// ```
pub fn card_with_header_footer<F>(header: &str, footer: &str, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    Element::<Div>::new()
        .class("card")
        .child::<Div, _>(|h| h.class("card-header").text(header))
        .child::<Div, _>(|body| f(body.class("card-body")))
        .child::<Div, _>(|foot| foot.class("card-footer text-body-secondary").text(footer))
}

/// Create a colored card (text-bg-{color}).
pub fn card_colored<F>(color: Color, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("card text-bg-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .child::<Div, _>(|body| f(body.class("card-body")))
}

/// Create a card with border color.
pub fn card_border<F>(color: Color, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = format!("card border-{}", color.as_str());
    Element::<Div>::new()
        .class(&class)
        .child::<Div, _>(|body| f(body.class("card-body")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_card() {
        let c = card(|b| b.text("Hello"));
        let html = c.render();
        assert!(html.contains(r#"class="card"#));
        assert!(html.contains(r#"class="card-body"#));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn test_card_simple() {
        let c = card_simple("Title", "Text content", "Click", "#");
        let html = c.render();
        assert!(html.contains("card-title"));
        assert!(html.contains("Title"));
        assert!(html.contains("card-text"));
        assert!(html.contains("btn btn-primary"));
    }

    #[test]
    fn test_card_with_image() {
        let c = card_with_image("/img.jpg", "Alt text", "Title", "Description");
        let html = c.render();
        assert!(html.contains("card-img-top"));
        assert!(html.contains(r#"src="/img.jpg"#));
        assert!(html.contains(r#"alt="Alt text"#));
    }

    #[test]
    fn test_card_with_header_footer() {
        let c = card_with_header_footer("Header", "Footer", |b| b.text("Body"));
        let html = c.render();
        assert!(html.contains("card-header"));
        assert!(html.contains("card-footer"));
        assert!(html.contains("Header"));
        assert!(html.contains("Footer"));
    }
}
