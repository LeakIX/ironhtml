//! Bootstrap Documentation Site Example
//!
//! This example replicates the structure of the Bootstrap documentation website,
//! focusing on the Components section. It demonstrates how to build a complete
//! documentation site using ironhtml-bootstrap.
//!
//! Run with: `cargo run --example bootstrap_docs`

use ironhtml::typed::{Document, Element};
use ironhtml_bootstrap::{
    accordion, alerts, badge, breadcrumb, buttons, cards, carousel, close_button, collapse,
    dropdown, list_group, modal, navbar, offcanvas, pagination, placeholder, progress, spinner,
    toast, tooltip, Color, NavbarExpand, Size,
};
use ironhtml_elements::{
    Body, Br, Button, Div, Form, Head, Html, Input, Li, Link, Main, Meta, Nav, Ol, Script, Section,
    Span, Style, Title, Ul, A, H1, H2, H4, H5, P,
};

extern crate alloc;
use alloc::vec;

// ============================================================
// DATA STRUCTURES
// ============================================================

/// Sidebar navigation item
struct SidebarItem {
    href: &'static str,
    label: &'static str,
    active: bool,
}

impl SidebarItem {
    const fn new(href: &'static str, label: &'static str) -> Self {
        Self {
            href,
            label,
            active: false,
        }
    }

    const fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

// ============================================================
// LAYOUT COMPONENTS
// ============================================================

/// Create the docs page header/navbar
fn docs_navbar() -> Element<Nav> {
    Element::<Nav>::new()
        .class("navbar navbar-expand-lg bg-dark navbar-dark sticky-top")
        .child::<Div, _>(|d| {
            d.class("container-fluid")
                .child::<A, _>(|a| {
                    a.class("navbar-brand")
                        .attr("href", "/")
                        .child::<Span, _>(|s| s.class("me-2").text("📘"))
                        .text("Bootstrap Docs")
                })
                .child::<Button, _>(|b| {
                    b.class("navbar-toggler")
                        .attr("type", "button")
                        .attr("data-bs-toggle", "collapse")
                        .attr("data-bs-target", "#navbarSearch")
                        .child::<Span, _>(|s| s.class("navbar-toggler-icon"))
                })
                .child::<Div, _>(|d| {
                    d.class("collapse navbar-collapse")
                        .attr("id", "navbarSearch")
                        .child::<Ul, _>(|ul| {
                            ul.class("navbar-nav me-auto")
                                .child::<Li, _>(|li| {
                                    li.class("nav-item").child::<A, _>(|a| {
                                        a.class("nav-link")
                                            .attr("href", "#")
                                            .text("Getting Started")
                                    })
                                })
                                .child::<Li, _>(|li| {
                                    li.class("nav-item").child::<A, _>(|a| {
                                        a.class("nav-link active")
                                            .attr("href", "#")
                                            .text("Components")
                                    })
                                })
                                .child::<Li, _>(|li| {
                                    li.class("nav-item").child::<A, _>(|a| {
                                        a.class("nav-link").attr("href", "#").text("Utilities")
                                    })
                                })
                        })
                        .child::<Form, _>(|f| {
                            f.class("d-flex")
                                .attr("role", "search")
                                .child::<Input, _>(|i| {
                                    i.class("form-control me-2")
                                        .attr("type", "search")
                                        .attr("placeholder", "Search...")
                                })
                        })
                })
        })
}

/// Create the sidebar navigation
fn docs_sidebar(items: &[SidebarItem]) -> Element<Nav> {
    Element::<Nav>::new()
        .class("sidebar bg-body-tertiary p-3")
        .attr("style", "width: 280px; min-height: 100vh;")
        .child::<H5, _>(|h| h.class("mb-3 text-muted").text("Components"))
        .child::<Ul, _>(|ul| {
            items
                .iter()
                .fold(ul.class("nav nav-pills flex-column"), |ul, item| {
                    ul.child::<Li, _>(|li| {
                        let link_class = if item.active {
                            "nav-link active"
                        } else {
                            "nav-link text-dark"
                        };
                        li.class("nav-item").child::<A, _>(|a| {
                            a.class(link_class).attr("href", item.href).text(item.label)
                        })
                    })
                })
        })
}

// ============================================================
// COMPONENT DOCUMENTATION SECTIONS
// ============================================================

/// Accordion documentation section
fn accordion_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "accordion")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Accordion"))
        .child::<P, _>(|p| {
            p.class("lead").text(
                "Build vertically collapsing accordions in combination with our Collapse JavaScript plugin.",
            )
        })
        // Basic example
        .child::<H4, _>(|h| h.class("mt-4").text("Basic Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                let items = vec![
                    accordion::AccordionItem {
                        id: "one".into(),
                        header: "Accordion Item #1".into(),
                        content: "This is the first item's accordion body.".into(),
                        expanded: true,
                    },
                    accordion::AccordionItem {
                        id: "two".into(),
                        header: "Accordion Item #2".into(),
                        content: "This is the second item's accordion body.".into(),
                        expanded: false,
                    },
                    accordion::AccordionItem {
                        id: "three".into(),
                        header: "Accordion Item #3".into(),
                        content: "This is the third item's accordion body.".into(),
                        expanded: false,
                    },
                ];
                accordion::accordion("accordionExample", &items)
            })
        })
        // Flush variant
        .child::<H4, _>(|h| h.class("mt-4").text("Flush"))
        .child::<P, _>(|p| {
            p.text("Add .accordion-flush to remove borders and rounded corners.")
        })
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                let items = vec![
                    accordion::AccordionItem {
                        id: "f1".into(),
                        header: "Accordion Item #1".into(),
                        content: "Flush accordion content here.".into(),
                        expanded: false,
                    },
                    accordion::AccordionItem {
                        id: "f2".into(),
                        header: "Accordion Item #2".into(),
                        content: "More flush accordion content.".into(),
                        expanded: false,
                    },
                ];
                accordion::accordion_flush("accordionFlush", &items)
            })
        })
}

/// Alerts documentation section
fn alerts_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "alerts")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Alerts"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Provide contextual feedback messages for typical user actions.")
        })
        // All color variants
        .child::<H4, _>(|h| h.class("mt-4").text("Examples"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|_| alerts::alert(Color::Primary, "A simple primary alert!"))
                .child::<Div, _>(|_| alerts::alert(Color::Secondary, "A simple secondary alert!"))
                .child::<Div, _>(|_| alerts::alert(Color::Success, "A simple success alert!"))
                .child::<Div, _>(|_| alerts::alert(Color::Danger, "A simple danger alert!"))
                .child::<Div, _>(|_| alerts::alert(Color::Warning, "A simple warning alert!"))
                .child::<Div, _>(|_| alerts::alert(Color::Info, "A simple info alert!"))
        })
        // Dismissible
        .child::<H4, _>(|h| h.class("mt-4").text("Dismissing"))
        .child::<P, _>(|p| p.text("Add a dismiss button and the .alert-dismissible class."))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                alerts::alert_dismissible(Color::Warning, "Holy guacamole! You should check this.")
            })
        })
        // With heading
        .child::<H4, _>(|h| h.class("mt-4").text("Additional Content"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                alerts::alert_with_heading(
                    Color::Success,
                    "Well done!",
                    "You successfully read this important alert message.",
                    "",
                )
            })
        })
}

/// Badges documentation section
fn badges_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "badges")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Badges"))
        .child::<P, _>(|p| p.class("lead").text("Small count and labeling component."))
        // Color variants
        .child::<H4, _>(|h| h.class("mt-4").text("Background Colors"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Span, _>(|_| badge::badge(Color::Primary, "Primary"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge(Color::Secondary, "Secondary"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge(Color::Success, "Success"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge(Color::Danger, "Danger"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge(Color::Warning, "Warning"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge(Color::Info, "Info"))
        })
        // Pill badges
        .child::<H4, _>(|h| h.class("mt-4").text("Pill Badges"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Span, _>(|_| badge::badge_pill(Color::Primary, "Primary"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge_pill(Color::Success, "Success"))
                .text(" ")
                .child::<Span, _>(|_| badge::badge_pill(Color::Danger, "99+"))
        })
}

/// Breadcrumb documentation section
fn breadcrumb_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "breadcrumb")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Breadcrumb"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Indicate the current page's location within a navigational hierarchy.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Nav, _>(|_| {
                let items = vec![
                    breadcrumb::BreadcrumbItem::link("Home", "/"),
                    breadcrumb::BreadcrumbItem::link("Library", "/library"),
                    breadcrumb::BreadcrumbItem::active("Data"),
                ];
                breadcrumb::breadcrumb(&items)
            })
        })
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Nav, _>(|_| {
                let items = vec![breadcrumb::BreadcrumbItem::active("Home")];
                breadcrumb::breadcrumb(&items)
            })
        })
}

/// Buttons documentation section
fn buttons_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "buttons")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Buttons"))
        .child::<P, _>(|p| {
            p.class("lead").text(
                "Use Bootstrap's custom button styles for actions in forms, dialogs, and more.",
            )
        })
        // Base styles
        .child::<H4, _>(|h| h.class("mt-4").text("Base Class"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| buttons::btn(Color::Primary, "Primary"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Secondary, "Secondary"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Success, "Success"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Danger, "Danger"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Warning, "Warning"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Info, "Info"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Light, "Light"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Dark, "Dark"))
        })
        // Outline buttons
        .child::<H4, _>(|h| h.class("mt-4").text("Outline Buttons"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| buttons::btn_outline(Color::Primary, "Primary"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_outline(Color::Secondary, "Secondary"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_outline(Color::Success, "Success"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_outline(Color::Danger, "Danger"))
        })
        // Sizes
        .child::<H4, _>(|h| h.class("mt-4").text("Sizes"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| buttons::btn_sized(Color::Primary, Size::Large, "Large"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn(Color::Primary, "Default"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_sized(Color::Primary, Size::Small, "Small"))
        })
        // Disabled
        .child::<H4, _>(|h| h.class("mt-4").text("Disabled State"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| buttons::btn_disabled(Color::Primary, "Disabled"))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_disabled(Color::Secondary, "Disabled"))
        })
}

/// Cards documentation section
fn cards_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "cards")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Cards"))
        .child::<P, _>(|p| {
            p.class("lead").text(
                "Bootstrap's cards provide a flexible and extensible content container.",
            )
        })
        // Basic card
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 18rem;")
                .child::<Div, _>(|_| {
                    cards::card_simple(
                        "Card title",
                        "Some quick example text to build on the card title and make up the bulk of the card's content.",
                        "Go somewhere",
                        "#",
                    )
                })
        })
        // Card with image
        .child::<H4, _>(|h| h.class("mt-4").text("Image Caps"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 18rem;")
                .child::<Div, _>(|_| {
                    cards::card_with_image(
                        "https://via.placeholder.com/286x180",
                        "Card image cap",
                        "Card title",
                        "This is a wider card with supporting text below.",
                    )
                })
        })
        // Card with header/footer
        .child::<H4, _>(|h| h.class("mt-4").text("Header and Footer"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 18rem;")
                .child::<Div, _>(|_| {
                    cards::card_with_header_footer("Featured", "Footer text", |body| {
                        body.child::<H5, _>(|h| h.class("card-title").text("Special title treatment"))
                            .child::<P, _>(|p| {
                                p.class("card-text").text(
                                    "With supporting text below as a natural lead-in to additional content.",
                                )
                            })
                    })
                })
        })
}

/// List Group documentation section
fn list_group_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "list-group")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("List Group"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("List groups are a flexible and powerful component for displaying lists.")
        })
        // Basic example
        .child::<H4, _>(|h| h.class("mt-4").text("Basic Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 400px;")
                .child::<Ul, _>(|_| {
                    list_group::list_group(&["An item", "A second item", "A third item"])
                })
        })
        // With links
        .child::<H4, _>(|h| h.class("mt-4").text("Links and Buttons"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 400px;")
                .child::<Div, _>(|_| {
                    let items = vec![
                        list_group::ListGroupLink::new("The current link item", "#").active(),
                        list_group::ListGroupLink::new("A second link item", "#"),
                        list_group::ListGroupLink::new("A third link item", "#"),
                        list_group::ListGroupLink::new("A disabled link item", "#").disabled(),
                    ];
                    list_group::list_group_links(&items)
                })
        })
        // Flush
        .child::<H4, _>(|h| h.class("mt-4").text("Flush"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 400px;")
                .child::<Ul, _>(|_| {
                    list_group::list_group_flush(&["An item", "A second item", "A third item"])
                })
        })
        // Numbered
        .child::<H4, _>(|h| h.class("mt-4").text("Numbered"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 400px;")
                .child::<Ol, _>(|_| {
                    list_group::list_group_numbered(&["A list item", "A list item", "A list item"])
                })
        })
}

/// Progress documentation section
fn progress_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "progress")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Progress"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Documentation and examples for using Bootstrap custom progress bars.")
        })
        // Basic example
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| d.class("mb-2").child::<Div, _>(|_| progress::progress(0)))
                .child::<Div, _>(|d| d.class("mb-2").child::<Div, _>(|_| progress::progress(25)))
                .child::<Div, _>(|d| d.class("mb-2").child::<Div, _>(|_| progress::progress(50)))
                .child::<Div, _>(|d| d.class("mb-2").child::<Div, _>(|_| progress::progress(75)))
                .child::<Div, _>(|d| d.class("mb-2").child::<Div, _>(|_| progress::progress(100)))
        })
        // Colored
        .child::<H4, _>(|h| h.class("mt-4").text("Backgrounds"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_colored(25, Color::Success))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_colored(50, Color::Info))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_colored(75, Color::Warning))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_colored(100, Color::Danger))
                })
        })
        // Striped
        .child::<H4, _>(|h| h.class("mt-4").text("Striped"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_striped(25, Color::Primary))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| progress::progress_striped(50, Color::Success))
                })
        })
        // Animated
        .child::<H4, _>(|h| h.class("mt-4").text("Animated Stripes"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|_| progress::progress_animated(75, Color::Primary))
        })
}

/// Spinners documentation section
fn spinners_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "spinners")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Spinners"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Indicate the loading state of a component or page with Bootstrap spinners.")
        })
        // Border spinner
        .child::<H4, _>(|h| h.class("mt-4").text("Border Spinner"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|_| spinner::spinner())
        })
        // Colors
        .child::<H4, _>(|h| h.class("mt-4").text("Colors"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Primary))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Secondary))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Success))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Danger))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Warning))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| spinner::spinner_colored(Color::Info))
                })
        })
        // Growing spinner
        .child::<H4, _>(|h| h.class("mt-4").text("Growing Spinner"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|_| spinner::spinner_grow())
        })
        // Small spinners
        .child::<H4, _>(|h| h.class("mt-4").text("Size"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Span, _>(|s| s.class("me-2").child::<Span, _>(|_| spinner::spinner_sm()))
                .child::<Span, _>(|_| spinner::spinner_grow_sm())
        })
        // Button with spinner
        .child::<H4, _>(|h| h.class("mt-4").text("Buttons"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| buttons::btn_loading(Color::Primary, "Loading..."))
                .text(" ")
                .child::<Button, _>(|_| buttons::btn_loading_grow(Color::Primary, "Loading..."))
        })
}

/// Navbar documentation section
fn navbar_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "navbar")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Navbar"))
        .child::<P, _>(|p| {
            p.class("lead").text(
                "Documentation and examples for Bootstrap's powerful, responsive navigation header.",
            )
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Nav, _>(|_| {
                navbar::navbar("Navbar", NavbarExpand::Lg, "navbarExample", |ul| {
                    ul.child::<Li, _>(|_| navbar::nav_item("/", "Home", true))
                        .child::<Li, _>(|_| navbar::nav_item("/features", "Features", false))
                        .child::<Li, _>(|_| navbar::nav_item("/pricing", "Pricing", false))
                        .child::<Li, _>(|_| navbar::nav_item_disabled("Disabled"))
                })
            })
        })
}

/// Carousel documentation section
fn carousel_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "carousel")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Carousel"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("A slideshow component for cycling through elements.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Basic Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                let items = vec![
                    carousel::CarouselItem::new(
                        "https://via.placeholder.com/800x400/007bff/ffffff?text=First+Slide",
                        "First slide",
                    )
                    .active()
                    .caption(
                        "First slide label",
                        "Some representative placeholder content.",
                    ),
                    carousel::CarouselItem::new(
                        "https://via.placeholder.com/800x400/6c757d/ffffff?text=Second+Slide",
                        "Second slide",
                    )
                    .caption("Second slide label", "Some more placeholder content."),
                    carousel::CarouselItem::new(
                        "https://via.placeholder.com/800x400/28a745/ffffff?text=Third+Slide",
                        "Third slide",
                    )
                    .caption("Third slide label", "Even more placeholder content."),
                ];
                carousel::carousel_with_indicators("carouselExample", &items)
            })
        })
}

/// Close Button documentation section
fn close_button_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "close-button")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Close Button"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("A generic close button for dismissing content like modals and alerts.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Examples"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| close_button::close_button())
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Disabled"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| close_button::close_button_disabled())
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Dark Variant"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3 bg-dark p-3")
                .child::<Button, _>(|_| close_button::close_button_white())
        })
}

/// Collapse documentation section
fn collapse_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "collapse")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Collapse"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Toggle the visibility of content with a few classes and JavaScript plugins.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<P, _>(|p| {
                    p.child::<Button, _>(|_| {
                        collapse::collapse_button("collapseExample", "Toggle content")
                    })
                    .text(" ")
                    .child::<A, _>(|_| collapse::collapse_link("collapseExample", "Link"))
                })
                .child::<Div, _>(|_| {
                    collapse::collapse_content("collapseExample", |div| {
                        div.child::<Div, _>(|d| {
                            d.class("card card-body")
                                .text("Some placeholder content for the collapse component.")
                        })
                    })
                })
        })
}

/// Dropdown documentation section
fn dropdown_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "dropdowns")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Dropdowns"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Toggle contextual overlays for displaying lists of links and more.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Single Button"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                let items = vec![
                    dropdown::DropdownItem::link("Action", "#"),
                    dropdown::DropdownItem::link("Another action", "#"),
                    dropdown::DropdownItem::divider(),
                    dropdown::DropdownItem::link("Separated link", "#"),
                ];
                dropdown::dropdown(Color::Primary, "Dropdown button", &items)
            })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Split Button"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Div, _>(|_| {
                let items = vec![
                    dropdown::DropdownItem::link("Action", "#"),
                    dropdown::DropdownItem::link("Another action", "#"),
                    dropdown::DropdownItem::divider(),
                    dropdown::DropdownItem::link("Separated link", "#"),
                ];
                dropdown::dropdown_split(Color::Success, "Action", "#", &items)
            })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Directions"))
        .child::<Div, _>(|d| {
            let items = vec![dropdown::DropdownItem::link("Action", "#")];
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| dropdown::dropup(Color::Secondary, "Dropup", &items))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block me-2")
                        .child::<Div, _>(|_| dropdown::dropend(Color::Secondary, "Dropend", &items))
                })
                .child::<Div, _>(|d| {
                    d.class("d-inline-block").child::<Div, _>(|_| {
                        dropdown::dropstart(Color::Secondary, "Dropstart", &items)
                    })
                })
        })
}

/// Modal documentation section
fn modal_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "modal")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Modal"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Add dialogs to your site for lightboxes, notifications, or custom content.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Live Demo"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| {
                    modal::modal_button("exampleModal", Color::Primary, "Launch demo modal")
                })
                .child::<Div, _>(|_| {
                    modal::modal_with_footer(
                        "exampleModal",
                        modal::ModalSize::Default,
                        "Modal title",
                        |body| body.text("This is modal body content."),
                        "Save changes",
                    )
                })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Sizes"))
        .child::<P, _>(|p| {
            p.text("Modals come in small, default, large, extra-large, and fullscreen sizes.")
        })
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| {
                    modal::modal_button("smallModal", Color::Primary, "Small modal")
                })
                .text(" ")
                .child::<Button, _>(|_| {
                    modal::modal_button("largeModal", Color::Primary, "Large modal")
                })
                .text(" ")
                .child::<Button, _>(|_| {
                    modal::modal_button("xlModal", Color::Primary, "Extra large modal")
                })
        })
}

/// Offcanvas documentation section
fn offcanvas_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "offcanvas")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Offcanvas"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Build hidden sidebars into your project for navigation and more.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| {
                    offcanvas::offcanvas_button("offcanvasExample", Color::Primary, "Toggle offcanvas")
                })
                .child::<Div, _>(|_| {
                    offcanvas::offcanvas(
                        "offcanvasExample",
                        offcanvas::OffcanvasPlacement::Start,
                        "Offcanvas",
                        |body| {
                            body.text("Some text as placeholder. In real life you can have elements here.")
                        },
                    )
                })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Placement"))
        .child::<P, _>(|p| {
            p.text("Offcanvas can be placed on the start, end, top, or bottom of the viewport.")
        })
}

/// Pagination documentation section
fn pagination_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "pagination")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Pagination"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Indicate a series of related content exists across multiple pages.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Basic Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Nav, _>(|_| {
                let items = vec![
                    pagination::PageItem::page(1, "#").active(),
                    pagination::PageItem::page(2, "#"),
                    pagination::PageItem::page(3, "#"),
                ];
                pagination::pagination(&items)
            })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("With Icons"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Nav, _>(|_| {
                let items = vec![
                    pagination::PageItem::page(1, "#").active(),
                    pagination::PageItem::page(2, "#"),
                    pagination::PageItem::page(3, "#"),
                ];
                pagination::pagination_with_arrows(&items, Some("#"), Some("#"))
            })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Sizes"))
        .child::<Div, _>(|d| {
            let items = vec![
                pagination::PageItem::page(1, "#").active(),
                pagination::PageItem::page(2, "#"),
                pagination::PageItem::page(3, "#"),
            ];
            d.class("bd-example mb-3")
                .child::<Nav, _>(|_| {
                    pagination::pagination_sized(&items, pagination::PaginationSize::Large)
                })
                .child::<Nav, _>(|_| {
                    pagination::pagination_sized(&items, pagination::PaginationSize::Small)
                })
        })
}

/// Placeholder documentation section
fn placeholder_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "placeholders")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Placeholders"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Use loading placeholders for your components or pages to indicate content is loading.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Example"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .attr("style", "max-width: 18rem;")
                .child::<Div, _>(|_| placeholder::placeholder_card())
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Width"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Span, _>(|_| placeholder::placeholder(placeholder::PlaceholderWidth::Col6))
                .child::<Br, _>(|b| b)
                .child::<Span, _>(|_| placeholder::placeholder(placeholder::PlaceholderWidth::Col4))
                .child::<Br, _>(|b| b)
                .child::<Span, _>(|_| placeholder::placeholder(placeholder::PlaceholderWidth::Col8))
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Color"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Span, _>(|_| {
                    placeholder::placeholder_colored(placeholder::PlaceholderWidth::Col12, Color::Primary)
                })
                .child::<Br, _>(|b| b)
                .child::<Span, _>(|_| {
                    placeholder::placeholder_colored(placeholder::PlaceholderWidth::Col12, Color::Success)
                })
                .child::<Br, _>(|b| b)
                .child::<Span, _>(|_| {
                    placeholder::placeholder_colored(placeholder::PlaceholderWidth::Col12, Color::Danger)
                })
        })
}

/// Toast documentation section
fn toast_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "toasts")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Toasts"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Push notifications to your visitors with a toast, a lightweight and customizable alert message.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Basic"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|_| toast::toast_show("Hello, world! This is a toast message.", "11 mins ago"))
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Color Schemes"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| toast::toast_colored(Color::Primary, "Primary toast message."))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| toast::toast_colored(Color::Success, "Success toast message."))
                })
                .child::<Div, _>(|d| {
                    d.class("mb-2")
                        .child::<Div, _>(|_| toast::toast_colored(Color::Danger, "Danger toast message."))
                })
        })
}

/// Tooltip documentation section
fn tooltip_section() -> Element<Section> {
    Element::<Section>::new()
        .attr("id", "tooltips")
        .class("mb-5")
        .child::<H2, _>(|h| h.class("border-bottom pb-2").text("Tooltips"))
        .child::<P, _>(|p| {
            p.class("lead")
                .text("Tooltips and popovers powered by CSS and JavaScript.")
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Tooltips"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3")
                .child::<Button, _>(|_| {
                    tooltip::tooltip_button(
                        Color::Secondary,
                        "Tooltip on top",
                        "Tooltip on top",
                        tooltip::Placement::Top,
                    )
                })
                .text(" ")
                .child::<Button, _>(|_| {
                    tooltip::tooltip_button(
                        Color::Secondary,
                        "Tooltip on right",
                        "Tooltip on right",
                        tooltip::Placement::Right,
                    )
                })
                .text(" ")
                .child::<Button, _>(|_| {
                    tooltip::tooltip_button(
                        Color::Secondary,
                        "Tooltip on bottom",
                        "Tooltip on bottom",
                        tooltip::Placement::Bottom,
                    )
                })
                .text(" ")
                .child::<Button, _>(|_| {
                    tooltip::tooltip_button(
                        Color::Secondary,
                        "Tooltip on left",
                        "Tooltip on left",
                        tooltip::Placement::Left,
                    )
                })
        })
        .child::<H4, _>(|h| h.class("mt-4").text("Popovers"))
        .child::<Div, _>(|d| {
            d.class("bd-example mb-3").child::<Button, _>(|_| {
                tooltip::popover_button(
                    Color::Danger,
                    "Click to toggle popover",
                    "Popover title",
                    "And here's some amazing content. It's very engaging.",
                    tooltip::Placement::Right,
                )
            })
        })
}

// ============================================================
// PAGE COMPOSITION
// ============================================================

/// Sidebar navigation items for the docs page
fn sidebar_items() -> alloc::vec::Vec<SidebarItem> {
    alloc::vec![
        SidebarItem::new("#accordion", "Accordion"),
        SidebarItem::new("#alerts", "Alerts").active(),
        SidebarItem::new("#badges", "Badges"),
        SidebarItem::new("#breadcrumb", "Breadcrumb"),
        SidebarItem::new("#buttons", "Buttons"),
        SidebarItem::new("#cards", "Cards"),
        SidebarItem::new("#carousel", "Carousel"),
        SidebarItem::new("#close-button", "Close Button"),
        SidebarItem::new("#collapse", "Collapse"),
        SidebarItem::new("#dropdowns", "Dropdowns"),
        SidebarItem::new("#list-group", "List Group"),
        SidebarItem::new("#modal", "Modal"),
        SidebarItem::new("#navbar", "Navbar"),
        SidebarItem::new("#offcanvas", "Offcanvas"),
        SidebarItem::new("#pagination", "Pagination"),
        SidebarItem::new("#placeholders", "Placeholders"),
        SidebarItem::new("#progress", "Progress"),
        SidebarItem::new("#spinners", "Spinners"),
        SidebarItem::new("#toasts", "Toasts"),
        SidebarItem::new("#tooltips", "Tooltips"),
    ]
}

/// Build the complete documentation page
fn build_docs_page() -> Document {
    let sidebar_items = sidebar_items();

    Document::new()
        .doctype()
        .root::<Html, _>(|html| {
            html.attr("lang", "en")
                .attr("data-bs-theme", "light")
                .child::<Head, _>(|h| {
                    h.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
                        .child::<Meta, _>(|m| {
                            m.attr("name", "viewport")
                                .attr("content", "width=device-width, initial-scale=1")
                        })
                        .child::<Title, _>(|t| t.text("Bootstrap Components - Documentation"))
                        .child::<Link, _>(|l| {
                            l.attr(
                                "href",
                                "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css",
                            )
                            .attr("rel", "stylesheet")
                        })
                        .child::<Style, _>(|s| {
                            s.text(
                                r"
                            .bd-example {
                                padding: 1.5rem;
                                border: 1px solid #dee2e6;
                                border-radius: 0.375rem;
                                background-color: #fff;
                            }
                            .sidebar {
                                position: sticky;
                                top: 56px;
                                height: calc(100vh - 56px);
                                overflow-y: auto;
                            }
                            section {
                                scroll-margin-top: 70px;
                            }
                        ",
                            )
                        })
                })
                .child::<Body, _>(|body| {
                    body.child::<Nav, _>(|_| docs_navbar())
                        .child::<Div, _>(|d| {
                            d.class("d-flex")
                                // Sidebar
                                .child::<Nav, _>(|_| docs_sidebar(&sidebar_items))
                                // Main content
                                .child::<Main, _>(|m| {
                                    m.class("flex-grow-1 p-4")
                                        .child::<Div, _>(|d| {
                                            d.class("container-fluid")
                                                .child::<H1, _>(|h| {
                                                    h.class("display-5 mb-4").text("Components")
                                                })
                                                .child::<P, _>(|p| {
                                                    p.class("lead text-muted mb-5").text(
                                                    "Dozens of reusable components built on top of Bootstrap.",
                                                )
                                                })
                                                // Component sections
                                                .child::<Section, _>(|_| accordion_section())
                                                .child::<Section, _>(|_| alerts_section())
                                                .child::<Section, _>(|_| badges_section())
                                                .child::<Section, _>(|_| breadcrumb_section())
                                                .child::<Section, _>(|_| buttons_section())
                                                .child::<Section, _>(|_| cards_section())
                                                .child::<Section, _>(|_| carousel_section())
                                                .child::<Section, _>(|_| close_button_section())
                                                .child::<Section, _>(|_| collapse_section())
                                                .child::<Section, _>(|_| dropdown_section())
                                                .child::<Section, _>(|_| list_group_section())
                                                .child::<Section, _>(|_| modal_section())
                                                .child::<Section, _>(|_| navbar_section())
                                                .child::<Section, _>(|_| offcanvas_section())
                                                .child::<Section, _>(|_| pagination_section())
                                                .child::<Section, _>(|_| placeholder_section())
                                                .child::<Section, _>(|_| progress_section())
                                                .child::<Section, _>(|_| spinners_section())
                                                .child::<Section, _>(|_| toast_section())
                                                .child::<Section, _>(|_| tooltip_section())
                                        })
                                })
                        })
                        // Bootstrap JS
                        .child::<Script, _>(|s| {
                            s.attr(
                                "src",
                                "https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js",
                            )
                        })
                })
        })
}

fn main() {
    let doc = build_docs_page();
    let html = doc.render();

    println!("{html}");

    // Calculate stats
    let component_count = 20; // Number of documented components
    let example_count = 50; // Approximate number of examples

    eprintln!("\n=== Bootstrap Documentation Site ===");
    eprintln!("Components documented: {component_count}");
    eprintln!("Examples rendered: {example_count}");
    eprintln!("HTML size: {} bytes", html.len());
    eprintln!("\nThis example demonstrates building a complete documentation site");
    eprintln!("similar to getbootstrap.com using ironhtml-bootstrap.");
}
