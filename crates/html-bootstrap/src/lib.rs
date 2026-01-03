//! # html-bootstrap
//!
//! Type-safe Bootstrap 5.3 components for Rust.
//!
//! This crate provides ergonomic, type-safe Bootstrap component generation
//! that integrates seamlessly with the `html-builder` ecosystem. Build
//! Bootstrap UIs with Rust's compile-time guarantees.
//!
//! ## Quick Start
//!
//! ```rust
//! use html_bootstrap::*;
//!
//! // Create a simple button
//! let button = buttons::btn(Color::Primary, "Click me");
//!
//! // Create an alert
//! let warning = alerts::alert(Color::Warning, "This is a warning!");
//!
//! // Create a card
//! let card = cards::card_simple(
//!     "Welcome",
//!     "Thanks for using html-bootstrap!",
//!     "Learn more",
//!     "#docs"
//! );
//! ```
//!
//! ## Reusable Components (React-like Pattern)
//!
//! Define custom components as functions, just like React components.
//! This is the recommended pattern for building maintainable UIs.
//!
//! ```rust
//! use html_bootstrap::*;
//! use html_builder::typed::Element;
//! use html_elements::{Div, A};
//!
//! // ============================================================
//! // REUSABLE COMPONENT: Product Card
//! // ============================================================
//! // Define once, use everywhere. Just like React components!
//!
//! struct Product {
//!     name: String,
//!     description: String,
//!     price: f64,
//!     image_url: String,
//! }
//!
//! fn product_card(product: &Product) -> Element<Div> {
//!     cards::card_with_image(
//!         &product.image_url,
//!         &product.name,
//!         &product.name,
//!         &product.description,
//!     )
//! }
//!
//! // ============================================================
//! // REUSABLE COMPONENT: Pricing Tier
//! // ============================================================
//!
//! struct PricingTier {
//!     name: &'static str,
//!     price: &'static str,
//!     features: Vec<&'static str>,
//!     highlighted: bool,
//! }
//!
//! fn pricing_card(tier: &PricingTier) -> Element<Div> {
//!     let color = if tier.highlighted { Color::Primary } else { Color::Light };
//!
//!     cards::card_colored(color, |body| {
//!         use html_elements::{H3, H4, Ul, Li, P};
//!
//!         body.child::<H3, _>(|h| h.class("card-title").text(tier.name))
//!             .child::<H4, _>(|h| h.text(tier.price))
//!             .child::<Ul, _>(|ul| {
//!                 tier.features.iter().fold(ul.class("list-unstyled"), |ul, feature| {
//!                     ul.child::<Li, _>(|li| li.text(feature))
//!                 })
//!             })
//!             .child::<Div, _>(|_| {
//!                 buttons::btn(Color::Primary, "Choose Plan")
//!             })
//!     })
//! }
//!
//! // ============================================================
//! // USAGE: Compose components into a page
//! // ============================================================
//!
//! fn render_product_catalog(products: &[Product]) -> Element<Div> {
//!     grid::container(|c| {
//!         c.child::<Div, _>(|_| {
//!             grid::row(|r| {
//!                 products.iter().fold(r, |row, product| {
//!                     row.child::<Div, _>(|_| {
//!                         grid::col(4, |col| {
//!                             col.child::<Div, _>(|_| product_card(product))
//!                         })
//!                     })
//!                 })
//!             })
//!         })
//!     })
//! }
//! ```
//!
//! ## Building a Complete Page
//!
//! Here's how to build a complete landing page with navbar, hero section,
//! features, and footer:
//!
//! ```rust
//! use html_bootstrap::*;
//! use html_builder::typed::{Document, Element};
//! use html_elements::*;
//!
//! // ============================================================
//! // COMPONENT: Navigation Menu
//! // ============================================================
//!
//! struct MenuItem {
//!     href: &'static str,
//!     label: &'static str,
//!     active: bool,
//! }
//!
//! fn main_navbar(brand: &str, items: &[MenuItem]) -> Element<Nav> {
//!     navbar::navbar(brand, NavbarExpand::Lg, "mainNav", |ul| {
//!         items.iter().fold(ul, |ul, item| {
//!             ul.child::<Li, _>(|_| navbar::nav_item(item.href, item.label, item.active))
//!         })
//!     })
//! }
//!
//! // ============================================================
//! // COMPONENT: Hero Section
//! // ============================================================
//!
//! fn hero_section(title: &str, subtitle: &str, cta_text: &str, cta_href: &str) -> Element<Div> {
//!     Element::<Div>::new()
//!         .class("bg-primary text-white py-5")
//!         .child::<Div, _>(|_| {
//!             grid::container(|c| {
//!                 c.class("text-center")
//!                     .child::<H1, _>(|h| h.class("display-4").text(title))
//!                     .child::<P, _>(|p| p.class("lead").text(subtitle))
//!                     .child::<A, _>(|a| {
//!                         a.class("btn btn-light btn-lg")
//!                             .attr("href", cta_href)
//!                             .text(cta_text)
//!                     })
//!             })
//!         })
//! }
//!
//! // ============================================================
//! // COMPONENT: Feature Card
//! // ============================================================
//!
//! struct Feature {
//!     icon: &'static str,  // Bootstrap icon class
//!     title: &'static str,
//!     description: &'static str,
//! }
//!
//! fn feature_card(feature: &Feature) -> Element<Div> {
//!     cards::card(|body| {
//!         body.class("text-center")
//!             .child::<I, _>(|i| i.class(feature.icon).class("fs-1 text-primary mb-3"))
//!             .child::<H5, _>(|h| h.class("card-title").text(feature.title))
//!             .child::<P, _>(|p| p.class("card-text text-muted").text(feature.description))
//!     })
//! }
//!
//! fn features_section(title: &str, features: &[Feature]) -> Element<Div> {
//!     Element::<Div>::new()
//!         .class("py-5")
//!         .child::<Div, _>(|_| {
//!             grid::container(|c| {
//!                 c.child::<H2, _>(|h| h.class("text-center mb-5").text(title))
//!                     .child::<Div, _>(|_| {
//!                         grid::row_gutter(4, |r| {
//!                             features.iter().fold(r, |row, feature| {
//!                                 row.child::<Div, _>(|_| {
//!                                     grid::col(4, |col| {
//!                                         col.child::<Div, _>(|_| feature_card(feature))
//!                                     })
//!                                 })
//!                             })
//!                         })
//!                     })
//!             })
//!         })
//! }
//!
//! // ============================================================
//! // COMPONENT: Footer
//! // ============================================================
//!
//! fn footer(copyright: &str) -> Element<Footer> {
//!     Element::<Footer>::new()
//!         .class("bg-dark text-white py-4 mt-5")
//!         .child::<Div, _>(|_| {
//!             grid::container(|c| {
//!                 c.class("text-center")
//!                     .child::<P, _>(|p| p.class("mb-0").text(copyright))
//!             })
//!         })
//! }
//!
//! // ============================================================
//! // COMPOSE: Full Landing Page
//! // ============================================================
//!
//! fn landing_page() -> Document {
//!     let menu_items = vec![
//!         MenuItem { href: "/", label: "Home", active: true },
//!         MenuItem { href: "/features", label: "Features", active: false },
//!         MenuItem { href: "/pricing", label: "Pricing", active: false },
//!         MenuItem { href: "/contact", label: "Contact", active: false },
//!     ];
//!
//!     let features = vec![
//!         Feature { icon: "bi bi-lightning", title: "Fast", description: "Blazing fast performance" },
//!         Feature { icon: "bi bi-shield", title: "Secure", description: "Enterprise-grade security" },
//!         Feature { icon: "bi bi-gear", title: "Flexible", description: "Highly customizable" },
//!     ];
//!
//!     Document::new()
//!         .doctype()
//!         .root::<Html, _>(|html| {
//!             html.attr("lang", "en")
//!                 .child::<Head, _>(|h| {
//!                     h.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
//!                      .child::<Title, _>(|t| t.text("My App"))
//!                      .child::<Link, _>(|l| {
//!                          l.attr("href", "https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css")
//!                           .attr("rel", "stylesheet")
//!                      })
//!                 })
//!                 .child::<Body, _>(|body| {
//!                     body.child::<Nav, _>(|_| main_navbar("MyApp", &menu_items))
//!                         .child::<Div, _>(|_| hero_section(
//!                             "Build Amazing Apps",
//!                             "The fastest way to create beautiful web applications",
//!                             "Get Started",
//!                             "#signup"
//!                         ))
//!                         .child::<Div, _>(|_| features_section("Features", &features))
//!                         .child::<Footer, _>(|_| footer("© 2024 MyApp. All rights reserved."))
//!                 })
//!         })
//! }
//! ```
//!
//! ## Dashboard Example
//!
//! ```rust
//! use html_bootstrap::*;
//! use html_builder::typed::Element;
//! use html_elements::*;
//!
//! // ============================================================
//! // COMPONENT: Stat Card (reusable)
//! // ============================================================
//!
//! fn stat_card(title: &str, value: &str, color: Color, trend: &str) -> Element<Div> {
//!     cards::card_border(color, |body| {
//!         body.child::<H6, _>(|h| h.class("text-muted").text(title))
//!             .child::<H2, _>(|h| h.class("mb-0").text(value))
//!             .child::<Small, _>(|s| s.class("text-success").text(trend))
//!     })
//! }
//!
//! // ============================================================
//! // COMPONENT: Activity Item (reusable)
//! // ============================================================
//!
//! struct Activity {
//!     user: String,
//!     action: String,
//!     time: String,
//! }
//!
//! fn activity_item(activity: &Activity) -> Element<Li> {
//!     Element::<Li>::new()
//!         .class("list-group-item d-flex justify-content-between")
//!         .child::<Div, _>(|d| {
//!             d.child::<Strong, _>(|s| s.text(&activity.user))
//!                 .text(" ")
//!                 .child::<Span, _>(|s| s.text(&activity.action))
//!         })
//!         .child::<Small, _>(|s| s.class("text-muted").text(&activity.time))
//! }
//!
//! // ============================================================
//! // COMPOSE: Dashboard Page
//! // ============================================================
//!
//! fn dashboard() -> Element<Div> {
//!     let activities = vec![
//!         Activity { user: "John".into(), action: "created a new project".into(), time: "5m ago".into() },
//!         Activity { user: "Jane".into(), action: "pushed 3 commits".into(), time: "10m ago".into() },
//!         Activity { user: "Bob".into(), action: "deployed to production".into(), time: "1h ago".into() },
//!     ];
//!
//!     grid::container(|c| {
//!         c.class("py-4")
//!             // Stats row
//!             .child::<Div, _>(|_| {
//!                 grid::row_gutter(4, |r| {
//!                     r.child::<Div, _>(|_| grid::col(3, |c| c.child::<Div, _>(|_| stat_card("Users", "1,234", Color::Primary, "+12%"))))
//!                      .child::<Div, _>(|_| grid::col(3, |c| c.child::<Div, _>(|_| stat_card("Revenue", "$45K", Color::Success, "+8%"))))
//!                      .child::<Div, _>(|_| grid::col(3, |c| c.child::<Div, _>(|_| stat_card("Orders", "567", Color::Info, "+23%"))))
//!                      .child::<Div, _>(|_| grid::col(3, |c| c.child::<Div, _>(|_| stat_card("Tickets", "12", Color::Warning, "-5%"))))
//!                 })
//!             })
//!             // Alert
//!             .child::<Div, _>(|_| {
//!                 alerts::alert_dismissible(Color::Info, "Welcome to your dashboard!")
//!             })
//!             // Activity section
//!             .child::<Div, _>(|_| {
//!                 grid::row(|r| {
//!                     r.child::<Div, _>(|_| {
//!                         grid::col(8, |c| {
//!                             c.child::<Div, _>(|_| {
//!                                 cards::card(|body| {
//!                                     body.child::<H5, _>(|h| h.text("Recent Activity"))
//!                                         .child::<Ul, _>(|ul| {
//!                                             activities.iter().fold(ul.class("list-group list-group-flush"), |ul, act| {
//!                                                 ul.child::<Li, _>(|_| activity_item(act))
//!                                             })
//!                                         })
//!                                 })
//!                             })
//!                         })
//!                     })
//!                     .child::<Div, _>(|_| {
//!                         grid::col(4, |c| {
//!                             c.child::<Div, _>(|_| {
//!                                 cards::card(|body| {
//!                                     body.child::<H5, _>(|h| h.text("Quick Actions"))
//!                                         .child::<Div, _>(|d| {
//!                                             d.class("d-grid gap-2")
//!                                                 .child::<Button, _>(|_| buttons::btn(Color::Primary, "New Project"))
//!                                                 .child::<Button, _>(|_| buttons::btn_outline(Color::Secondary, "View Reports"))
//!                                                 .child::<Button, _>(|_| buttons::btn_outline(Color::Success, "Export Data"))
//!                                         })
//!                                 })
//!                             })
//!                         })
//!                     })
//!                 })
//!             })
//!     })
//! }
//! ```
//!
//! ## Features
//!
//! - **Type-safe**: All components are strongly typed
//! - **Composable**: Build complex UIs from simple components
//! - **Zero runtime overhead**: All abstractions compile away
//! - **Bootstrap 5.3**: Full support for the latest Bootstrap
//! - **No JavaScript required**: Pure HTML output (add Bootstrap JS separately if needed)
//!
//! ## Feature Flags
//!
//! - `v5` (default): Bootstrap 5.3 support

#![no_std]

extern crate alloc;

pub mod alerts;
pub mod buttons;
pub mod cards;
pub mod grid;
pub mod navbar;
mod types;

pub use types::{Breakpoint, Color, NavbarExpand, Size};

// Re-export html_builder and html_elements for convenience
pub use html_builder;
pub use html_elements;
pub use html_macro;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        // Ensure types are accessible
        let _color = Color::Primary;
        let _size = Size::Large;
        let _bp = Breakpoint::Md;
        let _expand = NavbarExpand::Lg;
    }
}
