# ironhtml-tailwind

Type-safe Tailwind CSS utilities for [ironhtml](https://github.com/LeakIX/ironhtml) with ergonomic API and full IDE support.

## Features

- **Type-safe**: All Tailwind utilities are strongly typed enums
- **Ergonomic**: Fluent API that integrates seamlessly with ironhtml
- **Responsive**: Built-in support for responsive breakpoints (sm, md, lg, xl, 2xl)
- **State variants**: Support for hover, focus, active, and other pseudo-classes
- **Arbitrary values**: Escape hatch for custom classes
- **No runtime overhead**: All abstractions compile away
- **no_std compatible**: Works in embedded and WASM environments

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ironhtml = "1.0"
ironhtml-tailwind = "1.0"
```

## Usage

### With html! Macro (Recommended)

The html! macro provides the most ergonomic syntax for building HTML with Tailwind:

```rust
use ironhtml::html;

let card = html! {
    div.class("bg-white rounded-lg shadow-md p-6 hover:shadow-lg") {
        h2.class("text-2xl font-bold text-gray-900 mb-2") {
            "Welcome"
        }
        p.class("text-gray-600 mb-4") {
            "Type-safe HTML with Tailwind utilities"
        }
        button.class("bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600") {
            "Get Started"
        }
    }
};
```

See the [macro example](examples/tailwind_macro.rs) for a complete demonstration.

### With Typed API

For type-safe Tailwind utilities:

```rust
use ironhtml::typed::Element;
use ironhtml_elements::Div;
use ironhtml_tailwind::*;

let element = Element::<Div>::new()
    .tw(Padding::X(4))       // px-4
    .tw(TextAlign::Center)   // text-center
    .tw(Display::Flex)       // flex
    .tw(FontWeight::Bold);   // font-bold

let html = element.render();
// <div class="px-4 text-center flex font-bold"></div>
```

### Responsive Design

```rust
use ironhtml::typed::Element;
use ironhtml_elements::Div;
use ironhtml_tailwind::*;

let element = Element::<Div>::new()
    .tw(Padding::X(4))      // Mobile: px-4
    .tw_md(Padding::X(8))   // Tablet: md:px-8
    .tw_lg(Padding::X(12)); // Desktop: lg:px-12
```

### Interactive States

```rust
use ironhtml::typed::Element;
use ironhtml_elements::Button;
use ironhtml_tailwind::*;

let button = Element::<Button>::new()
    .tw(BackgroundColor::Blue(500))
    .tw_hover(BackgroundColor::Blue(700))
    .tw_active(BackgroundColor::Blue(800))
    .text("Click me");
```

### Complete Component Example

```rust
use ironhtml::typed::Element;
use ironhtml_elements::{Div, H3, P};
use ironhtml_tailwind::*;

fn card(title: &str, description: &str) -> Element<Div> {
    Element::<Div>::new()
        .tw(BackgroundColor::White)
        .tw(Padding::All(6))
        .tw(BorderRadius::Lg)
        .tw(Shadow::Default)
        .tw_hover(Shadow::Lg)
        .child::<H3, _>(|h| {
            h.tw(FontSize::Xl)
                .tw(FontWeight::Bold)
                .tw(Margin::B(2))
                .text(title)
        })
        .child::<P, _>(|p| {
            p.tw(TextColor::Gray(600))
                .text(description)
        })
}
```

## Supported Utilities

### Spacing
- **Padding**: `Padding::X(4)`, `Padding::Y(2)`, `Padding::All(8)`, etc.
- **Margin**: `Margin::X(4)`, `Margin::Auto(MarginAxis::X)`, etc.

### Layout
- **Display**: `Display::Flex`, `Display::Grid`, `Display::Block`, etc.
- **Position**: `Position::Relative`, `Position::Absolute`, etc.
- **Overflow**: `Overflow::Hidden`, `Overflow::Auto`, etc.

### Typography
- **Font Size**: `FontSize::Xl`, `FontSize::Base`, etc.
- **Font Weight**: `FontWeight::Bold`, `FontWeight::Medium`, etc.
- **Text Align**: `TextAlign::Center`, `TextAlign::Left`, etc.
- **Text Color**: `TextColor::Blue(500)`, `TextColor::Gray(700)`, etc.

### Flexbox
- **Flex Direction**: `FlexDirection::Row`, `FlexDirection::Col`, etc.
- **Justify Content**: `JustifyContent::Center`, `JustifyContent::Between`, etc.
- **Align Items**: `AlignItems::Center`, `AlignItems::Start`, etc.

### Grid
- **Grid Columns**: `GridCols::Cols(3)`, `GridCols::Cols(12)`, etc.
- **Grid Rows**: `GridRows::Rows(2)`, etc.
- **Gap**: `Gap::All(4)`, `Gap::X(2)`, `Gap::Y(8)`, etc.

### Sizing
- **Width**: `Width::Full`, `Width::Scaled(64)`, `Width::Fraction { numerator: 1, denominator: 2 }`
- **Height**: `Height::Full`, `Height::Screen`, etc.

### Borders
- **Border Width**: `BorderWidth::Default`, `BorderWidth::Width2`, etc.
- **Border Color**: `BorderColor::Gray(300)`, `BorderColor::Blue(500)`, etc.
- **Border Radius**: `BorderRadius::Lg`, `BorderRadius::Full`, etc.

### Backgrounds
- **Background Color**: `BackgroundColor::Blue(500)`, `BackgroundColor::White`, etc.

### Effects
- **Shadow**: `Shadow::Md`, `Shadow::Lg`, `Shadow::Xl`, etc.
- **Opacity**: `Opacity::O50`, `Opacity::O75`, etc.

## Responsive Breakpoints

- `tw_sm()` - Small devices (≥ 640px)
- `tw_md()` - Medium devices (≥ 768px)
- `tw_lg()` - Large devices (≥ 1024px)
- `tw_xl()` - Extra large devices (≥ 1280px)
- `tw_2xl()` - 2X large devices (≥ 1536px)

## State Variants

- `tw_hover()` - Hover state
- `tw_focus()` - Focus state
- `tw_active()` - Active state
- `tw_disabled()` - Disabled state

## Escape Hatch

For custom or arbitrary classes not covered by the type-safe API:

```rust
element.tw_raw("custom-utility-class")
```

## Examples

Run the included examples:

```bash
# Type-safe Tailwind utilities example
cargo run --example tailwind_demo

# html! macro with Tailwind example
cargo run --example tailwind_macro
```

Both examples generate complete HTML pages demonstrating various Tailwind utilities.

## License

MIT
