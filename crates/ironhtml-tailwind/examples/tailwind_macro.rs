//! Example demonstrating type-safe Tailwind CSS with the html! macro
//!
//! This example shows how to combine the html! macro with type-safe Tailwind utilities.
//! Since the macro doesn't directly support `.tw()` methods, we use helper functions to
//! create type-safe components that can be used within the macro.
//!
//! Run with: `cargo run --example tailwind_macro`

use ironhtml::html;
use ironhtml::typed::Element;
use ironhtml_elements::{
    Body, Button, Div, H1, H2, H3, Head, Html, Meta, P, Script, Title,
};
use ironhtml_tailwind::{
    BackgroundColor, BorderColor, BorderRadius, BorderWidth, Display, FontSize, FontWeight, Gap,
    GridCols, Height, Margin, Padding, Shadow, TailwindElement, TextColor,
};

// Helper function to create a type-safe card component
fn card_component() -> Element<Div> {
    Element::<Div>::new()
        .tw(BackgroundColor::White)
        .tw(BorderRadius::Lg)
        .tw(Shadow::Md)
        .tw(Padding::All(6))
        .tw_hover(Shadow::Lg)
        .child::<H2, _>(|h| {
            h.tw(FontSize::Xl2)
                .tw(FontWeight::Bold)
                .tw(TextColor::Gray(900))
                .tw(Margin::B(2))
                .text("Welcome to ironhtml")
        })
        .child::<P, _>(|p| {
            p.tw(TextColor::Gray(600))
                .tw(Margin::B(4))
                .text("Type-safe HTML with Tailwind CSS utilities")
        })
        .child::<Button, _>(|_| {
            button_component("Get Started")
        })
}

// Type-safe button component
fn button_component(text: &str) -> Element<Button> {
    Element::<Button>::new()
        .tw(BackgroundColor::Blue(500))
        .tw(TextColor::White)
        .tw(Padding::X(4))
        .tw(Padding::Y(2))
        .tw(BorderRadius::Default)
        .tw_hover(BackgroundColor::Blue(600))
        .text(text)
}

// Type-safe feature card
fn feature_card(_icon: &str, title: &str, description: &str) -> Element<Div> {
    Element::<Div>::new()
        .tw(BackgroundColor::Gray(50))
        .tw(Padding::All(6))
        .tw(BorderRadius::Lg)
        .tw(BorderWidth::Default)
        .tw(BorderColor::Gray(200))
        .child::<H3, _>(|h| {
            h.tw(FontSize::Xl)
                .tw(FontWeight::Bold)
                .tw(TextColor::Blue(700))
                .tw(Margin::B(2))
                .text(title)
        })
        .child::<P, _>(|p| {
            p.tw(TextColor::Gray(600))
                .text(description)
        })
}

// Type-safe hero section
fn hero_section() -> Element<Div> {
    Element::<Div>::new()
        .tw_raw("bg-gradient-to-r from-blue-500 to-purple-600")
        .tw(TextColor::White)
        .tw(BorderRadius::Xl)
        .tw(Padding::All(8))
        .tw(Margin::B(8))
        .child::<H2, _>(|h| {
            h.tw(FontSize::Xl3)
                .tw(FontWeight::Bold)
                .tw(Margin::B(4))
                .text("Build Beautiful UIs")
        })
        .child::<P, _>(|p| {
            p.tw(FontSize::Lg)
                .tw(Margin::B(6))
                .text("Combine the power of Rust's type system with Tailwind CSS")
        })
        .child::<Div, _>(|d| {
            d.tw(Display::Flex)
                .tw(Gap::All(4))
                .child::<Button, _>(|_| {
                    Element::<Button>::new()
                        .tw(BackgroundColor::White)
                        .tw(TextColor::Blue(600))
                        .tw(Padding::X(6))
                        .tw(Padding::Y(3))
                        .tw(BorderRadius::Lg)
                        .tw(FontWeight::SemiBold)
                        .tw_hover(BackgroundColor::Gray(100))
                        .text("Learn More")
                })
                .child::<Button, _>(|_| {
                    Element::<Button>::new()
                        .tw(BorderWidth::Width2)
                        .tw(BorderColor::White)
                        .tw(Padding::X(6))
                        .tw(Padding::Y(3))
                        .tw(BorderRadius::Lg)
                        .tw(FontWeight::SemiBold)
                        .tw_hover(BackgroundColor::White)
                        .tw_hover(TextColor::Blue(600))
                        .text("Documentation")
                })
        })
}

#[allow(clippy::too_many_lines)]
fn main() {
    // Approach 1: Use html! macro with raw class strings (simple, but not type-safe)
    let simple_card = html! {
        div.class("bg-white rounded-lg shadow-md p-6") {
            h2.class("text-2xl font-bold mb-2") { "Simple Card" }
            p.class("text-gray-600") { "Using raw Tailwind classes" }
        }
    };

    println!("<!-- Simple Card (raw strings) -->");
    println!("{}", simple_card.render());
    println!();

    // Approach 2: Use type-safe component functions
    println!("<!-- Type-safe Card Component -->");
    println!("{}", card_component().render());
    println!();

    // Approach 3: Mix html! macro for structure with type-safe utilities
    let features = vec![
        ("Fast", "Blazing fast compilation"),
        ("Safe", "Type-safe at compile time"),
        ("Easy", "Intuitive API design"),
    ];

    // Use type-safe wrapper for grid
    let grid = Element::<Div>::new()
        .tw(Display::Grid)
        .tw(GridCols::Cols(1))
        .tw_md(GridCols::Cols(3))
        .tw(Gap::All(6))
        .children::<Div, _, _>(features, |feature, _| {
            feature_card("", feature.0, feature.1)
        });

    println!("<!-- Feature Grid (type-safe) -->");
    println!("{}", grid.render());
    println!();

    // Approach 4: Build entire page with type-safe utilities
    let page = ironhtml::typed::Document::new()
        .doctype()
        .root::<Html, _>(|html| {
            html.attr("lang", "en")
                .child::<Head, _>(|head| {
                    head.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
                        .child::<Meta, _>(|m| {
                            m.attr("name", "viewport")
                                .attr("content", "width=device-width, initial-scale=1.0")
                        })
                        .child::<Title, _>(|t| t.text("Type-Safe Tailwind Demo"))
                        .child::<Script, _>(|s| {
                            s.attr("src", "https://cdn.tailwindcss.com")
                        })
                })
                .child::<Body, _>(|body| {
                    body.tw(BackgroundColor::Gray(50))
                        .tw(Height::Screen)
                        .child::<Div, _>(|container| {
                            container
                                .tw_raw("container mx-auto")
                                .tw(Padding::X(4))
                                .tw(Padding::Y(8))
                                .child::<H1, _>(|h| {
                                    h.tw(FontSize::Xl4)
                                        .tw(FontWeight::Bold)
                                        .tw(TextColor::Gray(900))
                                        .tw(Margin::B(8))
                                        .text("Type-Safe Tailwind CSS")
                                })
                                .child::<Div, _>(|_| hero_section())
                                .child::<Div, _>(|grid_container| {
                                    grid_container
                                        .tw(Display::Grid)
                                        .tw(GridCols::Cols(1))
                                        .tw_md(GridCols::Cols(3))
                                        .tw(Gap::All(6))
                                        .child::<Div, _>(|_| {
                                            Element::<Div>::new()
                                                .tw(BackgroundColor::White)
                                                .tw(Padding::All(6))
                                                .tw(BorderRadius::Lg)
                                                .tw(Shadow::Default)
                                                .tw_hover(Shadow::Lg)
                                                .child::<Div, _>(|d| {
                                                    d.tw(FontSize::Xl4)
                                                        .tw(Margin::B(4))
                                                        .text("🚀")
                                                })
                                                .child::<H3, _>(|h| {
                                                    h.tw(FontSize::Xl)
                                                        .tw(FontWeight::Bold)
                                                        .tw(Margin::B(2))
                                                        .text("Fast Development")
                                                })
                                                .child::<P, _>(|p| {
                                                    p.tw(TextColor::Gray(600))
                                                        .text("Type-safe Tailwind utilities")
                                                })
                                        })
                                        .child::<Div, _>(|_| {
                                            Element::<Div>::new()
                                                .tw(BackgroundColor::White)
                                                .tw(Padding::All(6))
                                                .tw(BorderRadius::Lg)
                                                .tw(Shadow::Default)
                                                .tw_hover(Shadow::Lg)
                                                .child::<Div, _>(|d| {
                                                    d.tw(FontSize::Xl4)
                                                        .tw(Margin::B(4))
                                                        .text("🛡️")
                                                })
                                                .child::<H3, _>(|h| {
                                                    h.tw(FontSize::Xl)
                                                        .tw(FontWeight::Bold)
                                                        .tw(Margin::B(2))
                                                        .text("Type Safety")
                                                })
                                                .child::<P, _>(|p| {
                                                    p.tw(TextColor::Gray(600))
                                                        .text("Compile-time guarantees")
                                                })
                                        })
                                        .child::<Div, _>(|_| {
                                            Element::<Div>::new()
                                                .tw(BackgroundColor::White)
                                                .tw(Padding::All(6))
                                                .tw(BorderRadius::Lg)
                                                .tw(Shadow::Default)
                                                .tw_hover(Shadow::Lg)
                                                .child::<Div, _>(|d| {
                                                    d.tw(FontSize::Xl4)
                                                        .tw(Margin::B(4))
                                                        .text("📦")
                                                })
                                                .child::<H3, _>(|h| {
                                                    h.tw(FontSize::Xl)
                                                        .tw(FontWeight::Bold)
                                                        .tw(Margin::B(2))
                                                        .text("No Dependencies")
                                                })
                                                .child::<P, _>(|p| {
                                                    p.tw(TextColor::Gray(600))
                                                        .text("Pure Rust implementation")
                                                })
                                        })
                                })
                        })
                })
        })
        .build();

    println!("<!-- Full Type-Safe Page -->");
    println!("{page}");
}
