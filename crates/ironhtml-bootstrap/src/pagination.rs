//! Bootstrap pagination components.
//!
//! Provides type-safe Bootstrap pagination matching the
//! [Bootstrap pagination documentation](https://getbootstrap.com/docs/5.3/components/pagination/).

use ironhtml::typed::Element;
use ironhtml_elements::{Li, Nav, Span, Ul, A};

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

/// Pagination size options.
#[derive(Clone, Copy, Default)]
pub enum PaginationSize {
    Small,
    #[default]
    Default,
    Large,
}

impl PaginationSize {
    const fn class(self) -> &'static str {
        match self {
            Self::Small => "pagination-sm",
            Self::Default => "",
            Self::Large => "pagination-lg",
        }
    }
}

/// A pagination item.
pub struct PageItem {
    pub label: String,
    pub href: String,
    pub active: bool,
    pub disabled: bool,
}

impl PageItem {
    /// Create a regular page item.
    #[must_use]
    pub fn page(number: u32, href: impl Into<String>) -> Self {
        Self {
            label: number.to_string(),
            href: href.into(),
            active: false,
            disabled: false,
        }
    }

    /// Create a page item with custom label.
    #[must_use]
    pub fn labeled(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            active: false,
            disabled: false,
        }
    }

    /// Mark this item as active.
    #[must_use]
    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    /// Mark this item as disabled.
    #[must_use]
    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Create a Bootstrap pagination.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::pagination::{pagination, PageItem};
///
/// let items = vec![
///     PageItem::page(1, "#").active(),
///     PageItem::page(2, "#"),
///     PageItem::page(3, "#"),
/// ];
///
/// let nav = pagination(&items);
/// assert!(nav.render().contains("pagination"));
/// ```
#[must_use]
pub fn pagination(items: &[PageItem]) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            items.iter().fold(ul.class("pagination"), |ul, item| {
                ul.child::<Li, _>(|_| page_item(item))
            })
        })
}

/// Create a sized pagination.
#[must_use]
pub fn pagination_sized(items: &[PageItem], size: PaginationSize) -> Element<Nav> {
    let class = if size.class().is_empty() {
        "pagination".to_string()
    } else {
        format!("pagination {}", size.class())
    };

    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            items.iter().fold(ul.class(&class), |ul, item| {
                ul.child::<Li, _>(|_| page_item(item))
            })
        })
}

/// Create a pagination with previous/next buttons.
#[must_use]
pub fn pagination_with_nav(
    items: &[PageItem],
    prev_href: Option<&str>,
    next_href: Option<&str>,
) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            // Previous button
            let ul = ul.class("pagination").child::<Li, _>(|li| {
                let li_class = if prev_href.is_none() {
                    "page-item disabled"
                } else {
                    "page-item"
                };
                let href = prev_href.unwrap_or("#");
                li.class(li_class)
                    .child::<A, _>(|a| a.class("page-link").attr("href", href).text("Previous"))
            });

            // Page items
            let ul = items
                .iter()
                .fold(ul, |ul, item| ul.child::<Li, _>(|_| page_item(item)));

            // Next button
            ul.child::<Li, _>(|li| {
                let li_class = if next_href.is_none() {
                    "page-item disabled"
                } else {
                    "page-item"
                };
                let href = next_href.unwrap_or("#");
                li.class(li_class)
                    .child::<A, _>(|a| a.class("page-link").attr("href", href).text("Next"))
            })
        })
}

/// Create a pagination with icon arrows.
#[must_use]
pub fn pagination_with_arrows(
    items: &[PageItem],
    prev_href: Option<&str>,
    next_href: Option<&str>,
) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            // Previous arrow
            let ul = ul.class("pagination").child::<Li, _>(|li| {
                let li_class = if prev_href.is_none() {
                    "page-item disabled"
                } else {
                    "page-item"
                };
                let href = prev_href.unwrap_or("#");
                li.class(li_class).child::<A, _>(|a| {
                    a.class("page-link")
                        .attr("href", href)
                        .attr("aria-label", "Previous")
                        .child::<Span, _>(|s| s.attr("aria-hidden", "true").text("\u{ab}"))
                })
            });

            // Page items
            let ul = items
                .iter()
                .fold(ul, |ul, item| ul.child::<Li, _>(|_| page_item(item)));

            // Next arrow
            ul.child::<Li, _>(|li| {
                let li_class = if next_href.is_none() {
                    "page-item disabled"
                } else {
                    "page-item"
                };
                let href = next_href.unwrap_or("#");
                li.class(li_class).child::<A, _>(|a| {
                    a.class("page-link")
                        .attr("href", href)
                        .attr("aria-label", "Next")
                        .child::<Span, _>(|s| s.attr("aria-hidden", "true").text("\u{bb}"))
                })
            })
        })
}

/// Create a centered pagination.
#[must_use]
pub fn pagination_centered(items: &[PageItem]) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            items
                .iter()
                .fold(ul.class("pagination justify-content-center"), |ul, item| {
                    ul.child::<Li, _>(|_| page_item(item))
                })
        })
}

/// Create a right-aligned pagination.
#[must_use]
pub fn pagination_end(items: &[PageItem]) -> Element<Nav> {
    Element::<Nav>::new()
        .attr("aria-label", "Page navigation")
        .child::<Ul, _>(|ul| {
            items
                .iter()
                .fold(ul.class("pagination justify-content-end"), |ul, item| {
                    ul.child::<Li, _>(|_| page_item(item))
                })
        })
}

/// Create a simple page item element.
fn page_item(item: &PageItem) -> Element<Li> {
    let mut class = String::from("page-item");
    if item.active {
        class.push_str(" active");
    }
    if item.disabled {
        class.push_str(" disabled");
    }

    let mut li = Element::<Li>::new().class(&class);

    if item.active {
        li = li.attr("aria-current", "page").child::<A, _>(|a| {
            a.class("page-link")
                .attr("href", &item.href)
                .text(&item.label)
        });
    } else {
        li = li.child::<A, _>(|a| {
            a.class("page-link")
                .attr("href", &item.href)
                .text(&item.label)
        });
    }

    li
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_pagination() {
        let items = vec![
            PageItem::page(1, "#").active(),
            PageItem::page(2, "#"),
            PageItem::page(3, "#"),
        ];
        let nav = pagination(&items);
        let html = nav.render();
        assert!(html.contains("pagination"));
        assert!(html.contains("page-item active"));
        assert!(html.contains("page-link"));
    }

    #[test]
    fn test_pagination_sizes() {
        let items = vec![PageItem::page(1, "#")];

        let small = pagination_sized(&items, PaginationSize::Small);
        assert!(small.render().contains("pagination-sm"));

        let large = pagination_sized(&items, PaginationSize::Large);
        assert!(large.render().contains("pagination-lg"));
    }

    #[test]
    fn test_pagination_with_nav() {
        let items = vec![PageItem::page(1, "#").active(), PageItem::page(2, "#")];
        let nav = pagination_with_nav(&items, Some("#"), Some("#page2"));
        let html = nav.render();
        assert!(html.contains("Previous"));
        assert!(html.contains("Next"));
    }
}
