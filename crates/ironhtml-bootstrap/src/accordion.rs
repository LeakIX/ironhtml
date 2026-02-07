//! Bootstrap accordion components.
//!
//! Provides type-safe Bootstrap accordions matching the
//! [Bootstrap accordion documentation](https://getbootstrap.com/docs/5.3/components/accordion/).

use ironhtml::typed::Element;
use ironhtml_elements::{Button, Div, H2};

extern crate alloc;
use alloc::format;
use alloc::string::String;

/// An accordion item with header and content.
pub struct AccordionItem {
    pub id: String,
    pub header: String,
    pub content: String,
    pub expanded: bool,
}

/// Create a Bootstrap accordion.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::accordion::{accordion, AccordionItem};
///
/// let items = vec![
///     AccordionItem {
///         id: "one".into(),
///         header: "Accordion Item #1".into(),
///         content: "This is the first item's content.".into(),
///         expanded: true,
///     },
///     AccordionItem {
///         id: "two".into(),
///         header: "Accordion Item #2".into(),
///         content: "This is the second item's content.".into(),
///         expanded: false,
///     },
/// ];
///
/// let acc = accordion("accordionExample", &items);
/// assert!(acc.render().contains("accordion"));
/// ```
#[must_use]
pub fn accordion(id: &str, items: &[AccordionItem]) -> Element<Div> {
    let mut acc = Element::<Div>::new().class("accordion").id(id);

    for item in items {
        acc = acc.child::<Div, _>(|_| accordion_item(id, item));
    }

    acc
}

/// Create a flush accordion (no borders, square corners).
#[must_use]
pub fn accordion_flush(id: &str, items: &[AccordionItem]) -> Element<Div> {
    let mut acc = Element::<Div>::new()
        .class("accordion accordion-flush")
        .id(id);

    for item in items {
        acc = acc.child::<Div, _>(|_| accordion_item(id, item));
    }

    acc
}

/// Create a single accordion item.
fn accordion_item(parent_id: &str, item: &AccordionItem) -> Element<Div> {
    let collapse_id = format!("collapse{}", item.id);
    let heading_id = format!("heading{}", item.id);
    let target = format!("#{collapse_id}");
    let parent = format!("#{parent_id}");

    let button_class = if item.expanded {
        "accordion-button"
    } else {
        "accordion-button collapsed"
    };

    let collapse_class = if item.expanded {
        "accordion-collapse collapse show"
    } else {
        "accordion-collapse collapse"
    };

    Element::<Div>::new()
        .class("accordion-item")
        .child::<H2, _>(|h| {
            h.class("accordion-header")
                .id(&heading_id)
                .child::<Button, _>(|btn| {
                    btn.class(button_class)
                        .attr("type", "button")
                        .attr("data-bs-toggle", "collapse")
                        .attr("data-bs-target", &target)
                        .attr(
                            "aria-expanded",
                            if item.expanded { "true" } else { "false" },
                        )
                        .attr("aria-controls", &collapse_id)
                        .text(&item.header)
                })
        })
        .child::<Div, _>(|collapse| {
            collapse
                .id(&collapse_id)
                .class(collapse_class)
                .attr("aria-labelledby", &heading_id)
                .attr("data-bs-parent", &parent)
                .child::<Div, _>(|body| body.class("accordion-body").text(&item.content))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_accordion() {
        let items = vec![
            AccordionItem {
                id: "one".into(),
                header: "First".into(),
                content: "Content 1".into(),
                expanded: true,
            },
            AccordionItem {
                id: "two".into(),
                header: "Second".into(),
                content: "Content 2".into(),
                expanded: false,
            },
        ];

        let acc = accordion("test", &items);
        let html = acc.render();
        assert!(html.contains("accordion"));
        assert!(html.contains("accordion-item"));
        assert!(html.contains("accordion-button"));
        assert!(html.contains("collapse show"));
        assert!(html.contains("collapsed"));
    }
}
