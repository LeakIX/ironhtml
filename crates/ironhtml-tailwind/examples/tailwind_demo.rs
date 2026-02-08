//! Example demonstrating type-safe Tailwind CSS usage with ironhtml
//!
//! Run with: `cargo run --example tailwind_demo`

use ironhtml::typed::{Document, Element};
use ironhtml_elements::{
    Body, Button, Div, H1, H2, H3, Head, Html, Meta, P, Script, Title,
};
use ironhtml_tailwind::{
    BackgroundColor, BorderColor, BorderRadius, BorderWidth, Display, FlexDirection,
    FontSize, FontWeight, Gap, GridCols, Margin, Padding, Shadow, TailwindElement,
    TextColor, Width,
};

fn main() {
    let page = Document::new()
        .doctype()
        .root::<Html, _>(|html| {
            html.attr("lang", "en")
                .child::<Head, _>(|head| {
                    head.child::<Meta, _>(|m| m.attr("charset", "UTF-8"))
                        .child::<Meta, _>(|m| {
                            m.attr("name", "viewport")
                                .attr("content", "width=device-width, initial-scale=1.0")
                        })
                        .child::<Title, _>(|t| t.text("Tailwind CSS Demo"))
                        .child::<Script, _>(|s| s.attr("src", "https://cdn.tailwindcss.com"))
                })
                .child::<Body, _>(|body| {
                    body.tw(BackgroundColor::Gray(50))
                        .tw(Padding::All(8))
                        .child::<Div, _>(|_| container())
                })
        })
        .build();

    println!("{page}");
}

/// Main container with card layout
fn container() -> Element<Div> {
    Element::<Div>::new()
        .tw(Width::Full)
        .tw(Display::Flex)
        .tw(FlexDirection::Col)
        .tw(Gap::All(8))
        .child::<H1, _>(|h| {
            h.tw(FontSize::Xl4)
                .tw(FontWeight::Bold)
                .tw(TextColor::Gray(900))
                .tw(Margin::B(8))
                .text("Tailwind CSS with ironhtml")
        })
        .child::<Div, _>(|_| hero_card())
        .child::<Div, _>(|_| {
            Element::<Div>::new()
                .tw(Display::Grid)
                .tw(GridCols::Cols(3))
                .tw(Gap::All(6))
                .tw_md(GridCols::Cols(1))
                .child::<Div, _>(|_| feature_card("Fast", "Blazing fast type-safe HTML", "blue"))
                .child::<Div, _>(|_| {
                    feature_card("Safe", "Compile-time Tailwind validation", "green")
                })
                .child::<Div, _>(|_| {
                    feature_card("Easy", "Ergonomic API with full IDE support", "purple")
                })
        })
        .child::<Div, _>(|_| button_showcase())
}

/// Hero card component
fn hero_card() -> Element<Div> {
    Element::<Div>::new()
        .tw(BackgroundColor::Blue(500))
        .tw(TextColor::White)
        .tw(Padding::All(8))
        .tw(BorderRadius::Lg)
        .tw(Shadow::Xl)
        .child::<H2, _>(|h| {
            h.tw(FontSize::Xl3)
                .tw(FontWeight::Bold)
                .tw(Margin::B(4))
                .text("Type-Safe Tailwind CSS")
        })
        .child::<P, _>(|p| {
            p.tw(FontSize::Lg)
                .tw(Margin::B(6))
                .text("Build beautiful UIs with Rust's type system ensuring your Tailwind classes are always valid.")
        })
        .child::<Div, _>(|d| {
            d.tw(Display::Flex)
                .tw(Gap::All(4))
                .child::<Button, _>(|_| {
                    Element::<Button>::new()
                        .tw(BackgroundColor::White)
                        .tw(TextColor::Blue(600))
                        .tw(FontWeight::SemiBold)
                        .tw(Padding::X(6))
                        .tw(Padding::Y(3))
                        .tw(BorderRadius::Md)
                        .tw_hover(BackgroundColor::Gray(100))
                        .text("Get Started")
                })
                .child::<Button, _>(|_| {
                    Element::<Button>::new()
                        .tw(BorderWidth::Width2)
                        .tw(BorderColor::White)
                        .tw(TextColor::White)
                        .tw(FontWeight::SemiBold)
                        .tw(Padding::X(6))
                        .tw(Padding::Y(3))
                        .tw(BorderRadius::Md)
                        .tw_hover(BackgroundColor::Blue(600))
                        .text("Learn More")
                })
        })
}

/// Feature card component
fn feature_card(title: &str, description: &str, color: &str) -> Element<Div> {
    let (bg_color, text_color, border_color) = match color {
        "blue" => (
            BackgroundColor::Blue(50),
            TextColor::Blue(700),
            BorderColor::Blue(200),
        ),
        "green" => (
            BackgroundColor::Green(50),
            TextColor::Green(700),
            BorderColor::Green(200),
        ),
        "purple" => (
            BackgroundColor::Purple(50),
            TextColor::Purple(700),
            BorderColor::Purple(200),
        ),
        _ => (
            BackgroundColor::Gray(50),
            TextColor::Gray(700),
            BorderColor::Gray(200),
        ),
    };

    Element::<Div>::new()
        .tw(bg_color)
        .tw(Padding::All(6))
        .tw(BorderRadius::Lg)
        .tw(BorderWidth::Default)
        .tw(border_color)
        .tw_hover(Shadow::Md)
        .child::<H3, _>(|h| {
            h.tw(text_color)
                .tw(FontSize::Xl)
                .tw(FontWeight::Bold)
                .tw(Margin::B(2))
                .text(title)
        })
        .child::<P, _>(|p| p.tw(TextColor::Gray(600)).text(description))
}

/// Button showcase
fn button_showcase() -> Element<Div> {
    Element::<Div>::new()
        .tw(BackgroundColor::White)
        .tw(Padding::All(8))
        .tw(BorderRadius::Lg)
        .tw(Shadow::Default)
        .child::<H2, _>(|h| {
            h.tw(FontSize::Xl2)
                .tw(FontWeight::Bold)
                .tw(Margin::B(6))
                .text("Button Variants")
        })
        .child::<Div, _>(|d| {
            d.tw(Display::Flex)
                .tw(Gap::All(4))
                .tw(FlexDirection::Row)
                .tw_md(FlexDirection::Col)
                .child::<Button, _>(|_| {
                    styled_button(
                        "Primary",
                        BackgroundColor::Blue(600),
                        BackgroundColor::Blue(700),
                    )
                })
                .child::<Button, _>(|_| {
                    styled_button(
                        "Success",
                        BackgroundColor::Green(600),
                        BackgroundColor::Green(700),
                    )
                })
                .child::<Button, _>(|_| {
                    styled_button(
                        "Danger",
                        BackgroundColor::Red(600),
                        BackgroundColor::Red(700),
                    )
                })
                .child::<Button, _>(|_| {
                    styled_button(
                        "Warning",
                        BackgroundColor::Yellow(500),
                        BackgroundColor::Yellow(600),
                    )
                })
        })
}

/// Styled button helper
fn styled_button(text: &str, bg: BackgroundColor, hover_bg: BackgroundColor) -> Element<Button> {
    Element::<Button>::new()
        .tw(bg)
        .tw(TextColor::White)
        .tw(FontWeight::Medium)
        .tw(Padding::X(6))
        .tw(Padding::Y(3))
        .tw(BorderRadius::Md)
        .tw(Shadow::Sm)
        .tw_hover(hover_bg)
        .tw_hover(Shadow::Md)
        .tw_active(Shadow::Default)
        .text(text)
}
