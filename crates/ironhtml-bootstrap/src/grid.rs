//! Bootstrap grid system components.
//!
//! The grid system uses containers, rows, and columns to layout content.
//!
//! ## Example
//!
//! ```rust
//! use ironhtml_bootstrap::grid::*;
//! use ironhtml::typed::Element;
//! use ironhtml_elements::Div;
//!
//! let layout = container(|c| {
//!     c.child::<Div, _>(|_| {
//!         row(|r| {
//!             r.child::<Div, _>(|_| col(4, |c| c.text("Column 1")))
//!              .child::<Div, _>(|_| col(4, |c| c.text("Column 2")))
//!              .child::<Div, _>(|_| col(4, |c| c.text("Column 3")))
//!         })
//!     })
//! });
//!
//! let html = layout.render();
//! assert!(html.contains(r#"class="container"#));
//! assert!(html.contains(r#"class="row"#));
//! assert!(html.contains(r#"class="col-4"#));
//! ```

use crate::Breakpoint;
use ironhtml::typed::Element;
use ironhtml_elements::Div;

/// Create a Bootstrap container.
///
/// Generates: `<div class="container">...</div>`
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::grid::container;
///
/// let c = container(|c| c.text("Content"));
/// assert!(c.render().contains(r#"<div class="container">"#));
/// ```
#[must_use]
pub fn container<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("container"))
}

/// Create a fluid container (100% width).
///
/// Generates: `<div class="container-fluid">...</div>`
#[must_use]
pub fn container_fluid<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("container-fluid"))
}

/// Create a responsive container.
///
/// Generates: `<div class="container-{breakpoint}">...</div>`
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{grid::container_bp, Breakpoint};
///
/// let c = container_bp(Breakpoint::Md, |c| c.text("Content"));
/// assert!(c.render().contains(r#"class="container-md"#));
/// ```
#[must_use]
pub fn container_bp<F>(bp: Breakpoint, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = alloc::format!("container-{}", bp.as_str());
    f(Element::<Div>::new().class(&class))
}

/// Create a row.
///
/// Generates: `<div class="row">...</div>`
#[must_use]
pub fn row<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("row"))
}

/// Create a row with custom gutter.
///
/// Generates: `<div class="row g-{gutter}">...</div>`
#[must_use]
pub fn row_gutter<F>(gutter: u8, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = alloc::format!("row g-{gutter}");
    f(Element::<Div>::new().class(&class))
}

/// Create an auto-width column.
///
/// Generates: `<div class="col">...</div>`
#[must_use]
pub fn col_auto<F>(f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    f(Element::<Div>::new().class("col"))
}

/// Create a column with specific size (1-12).
///
/// Generates: `<div class="col-{size}">...</div>`
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::grid::col;
///
/// let c = col(6, |c| c.text("Half width"));
/// assert!(c.render().contains(r#"class="col-6"#));
/// ```
#[must_use]
pub fn col<F>(size: u8, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = alloc::format!("col-{size}");
    f(Element::<Div>::new().class(&class))
}

/// Create a responsive column.
///
/// Generates: `<div class="col-{breakpoint}-{size}">...</div>`
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::{grid::col_bp, Breakpoint};
///
/// let c = col_bp(Breakpoint::Md, 6, |c| c.text("Half on medium+"));
/// assert!(c.render().contains(r#"class="col-md-6"#));
/// ```
#[must_use]
pub fn col_bp<F>(bp: Breakpoint, size: u8, f: F) -> Element<Div>
where
    F: FnOnce(Element<Div>) -> Element<Div>,
{
    let class = alloc::format!("col-{}-{size}", bp.as_str());
    f(Element::<Div>::new().class(&class))
}

extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let c = container(|c| c.text("Hello"));
        assert!(c.render().contains(r#"<div class="container">"#));
    }

    #[test]
    fn test_row_and_columns() {
        let layout = row(|r| {
            r.child::<Div, _>(|_| col(4, |c| c.text("A")))
                .child::<Div, _>(|_| col(8, |c| c.text("B")))
        });
        let html = layout.render();
        assert!(html.contains(r#"class="row"#));
        assert!(html.contains(r#"class="col-4"#));
        assert!(html.contains(r#"class="col-8"#));
    }
}
