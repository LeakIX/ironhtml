//! # ironhtml-tailwind
//!
//! Type-safe Tailwind CSS utilities for ironhtml with compile-time conflict detection.
//!
//! This crate provides type-safe Tailwind CSS class generation with a simple,
//! ergonomic API that integrates seamlessly with ironhtml's typed HTML builder.
//!
//! ## Example
//!
//! ```rust
//! use ironhtml_tailwind::*;
//! use ironhtml::typed::Element;
//! use ironhtml_elements::Div;
//!
//! let element = Element::<Div>::new()
//!     .tw(Padding::X(4))       // px-4
//!     .tw(TextAlign::Center)   // text-center
//!     .tw(Display::Flex)       // flex
//!     .tw(FontWeight::Bold);   // font-bold
//!
//! let html = element.render();
//! assert!(html.contains("px-4"));
//! assert!(html.contains("text-center"));
//! assert!(html.contains("flex"));
//! assert!(html.contains("font-bold"));
//! ```
//!
//! ## Responsive Variants
//!
//! Use responsive methods for different breakpoints:
//!
//! ```rust
//! use ironhtml_tailwind::*;
//! use ironhtml::typed::Element;
//! use ironhtml_elements::Div;
//!
//! let element = Element::<Div>::new()
//!     .tw(Padding::X(4))      // px-4
//!     .tw_md(Padding::X(8));  // md:px-8
//!
//! let html = element.render();
//! assert!(html.contains("px-4"));
//! assert!(html.contains("md:px-8"));
//! ```
//!
//! ## State Variants
//!
//! Use state methods for hover, focus, etc:
//!
//! ```rust
//! use ironhtml_tailwind::*;
//! use ironhtml::typed::Element;
//! use ironhtml_elements::Div;
//!
//! let element = Element::<Div>::new()
//!     .tw(TextColor::Blue(500))
//!     .tw_hover(TextColor::Blue(700));
//!
//! let html = element.render();
//! assert!(html.contains("text-blue-500"));
//! assert!(html.contains("hover:text-blue-700"));
//! ```
//!
//! ## Escape Hatch
//!
//! Use `tw_raw()` for custom classes:
//!
//! ```rust
//! use ironhtml_tailwind::*;
//! use ironhtml::typed::Element;
//! use ironhtml_elements::Div;
//!
//! let element = Element::<Div>::new()
//!     .tw(Padding::X(4))
//!     .tw_raw("custom-class");
//!
//! let html = element.render();
//! assert!(html.contains("px-4"));
//! assert!(html.contains("custom-class"));
//! ```

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use ironhtml::typed::Element;
use ironhtml_elements::HtmlElement;

// Utility modules
mod spacing;
mod layout;
mod typography;
mod flexbox;
mod grid;
mod sizing;
mod borders;
mod backgrounds;
mod effects;

// Re-export utility types
pub use spacing::{Margin, Padding};
pub use layout::{Display, Overflow, Position};
pub use typography::{FontSize, FontWeight, TextAlign, TextColor};
pub use flexbox::{AlignItems, FlexDirection, JustifyContent};
pub use grid::{Gap, GridCols, GridRows};
pub use sizing::{Height, Width};
pub use borders::{BorderColor, BorderRadius, BorderWidth};
pub use backgrounds::BackgroundColor;
pub use effects::{Opacity, Shadow};

/// Trait for types that can be converted to Tailwind class strings
pub trait TailwindClass {
    /// Convert to a Tailwind CSS class string
    fn to_class(&self) -> String;
}

/// Responsive breakpoint prefix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breakpoint {
    /// Small (sm:) - >= 640px
    Sm,
    /// Medium (md:) - >= 768px
    Md,
    /// Large (lg:) - >= 1024px
    Lg,
    /// Extra Large (xl:) - >= 1280px
    Xl,
    /// 2X Large (2xl:) - >= 1536px
    TwoXl,
}

impl Breakpoint {
    const fn prefix(self) -> &'static str {
        match self {
            Self::Sm => "sm:",
            Self::Md => "md:",
            Self::Lg => "lg:",
            Self::Xl => "xl:",
            Self::TwoXl => "2xl:",
        }
    }
}

/// State variant prefix (hover, focus, etc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateVariant {
    /// hover:
    Hover,
    /// focus:
    Focus,
    /// active:
    Active,
    /// disabled:
    Disabled,
    /// visited:
    Visited,
    /// focus-within:
    FocusWithin,
}

impl StateVariant {
    const fn prefix(self) -> &'static str {
        match self {
            Self::Hover => "hover:",
            Self::Focus => "focus:",
            Self::Active => "active:",
            Self::Disabled => "disabled:",
            Self::Visited => "visited:",
            Self::FocusWithin => "focus-within:",
        }
    }
}

/// Extension trait for adding Tailwind utilities to HTML elements
pub trait TailwindElement<E: HtmlElement>: Sized {
    /// Add a Tailwind utility class
    fn tw(self, class: impl TailwindClass) -> Self;
    
    /// Add responsive utility for sm breakpoint (>= 640px)
    fn tw_sm(self, class: impl TailwindClass) -> Self;
    
    /// Add responsive utility for md breakpoint (>= 768px)
    fn tw_md(self, class: impl TailwindClass) -> Self;
    
    /// Add responsive utility for lg breakpoint (>= 1024px)
    fn tw_lg(self, class: impl TailwindClass) -> Self;
    
    /// Add responsive utility for xl breakpoint (>= 1280px)
    fn tw_xl(self, class: impl TailwindClass) -> Self;
    
    /// Add responsive utility for 2xl breakpoint (>= 1536px)
    fn tw_2xl(self, class: impl TailwindClass) -> Self;
    
    /// Add hover state utility
    fn tw_hover(self, class: impl TailwindClass) -> Self;
    
    /// Add focus state utility
    fn tw_focus(self, class: impl TailwindClass) -> Self;
    
    /// Add active state utility
    fn tw_active(self, class: impl TailwindClass) -> Self;
    
    /// Add disabled state utility
    fn tw_disabled(self, class: impl TailwindClass) -> Self;
    
    /// Add arbitrary class string
    fn tw_raw(self, class: impl Into<String>) -> Self;
}

impl<E: HtmlElement> TailwindElement<E> for Element<E> {
    fn tw(self, class: impl TailwindClass) -> Self {
        self.class(class.to_class())
    }
    
    fn tw_sm(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", Breakpoint::Sm.prefix(), class.to_class()))
    }
    
    fn tw_md(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", Breakpoint::Md.prefix(), class.to_class()))
    }
    
    fn tw_lg(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", Breakpoint::Lg.prefix(), class.to_class()))
    }
    
    fn tw_xl(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", Breakpoint::Xl.prefix(), class.to_class()))
    }
    
    fn tw_2xl(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", Breakpoint::TwoXl.prefix(), class.to_class()))
    }
    
    fn tw_hover(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", StateVariant::Hover.prefix(), class.to_class()))
    }
    
    fn tw_focus(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", StateVariant::Focus.prefix(), class.to_class()))
    }
    
    fn tw_active(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", StateVariant::Active.prefix(), class.to_class()))
    }
    
    fn tw_disabled(self, class: impl TailwindClass) -> Self {
        self.class(format!("{}{}", StateVariant::Disabled.prefix(), class.to_class()))
    }
    
    fn tw_raw(self, class: impl Into<String>) -> Self {
        self.class(class.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ironhtml_elements::Div;

    #[test]
    fn test_basic_utilities() {
        let elem = Element::<Div>::new()
            .tw(Padding::X(4))
            .tw(Margin::Y(2))
            .tw(Display::Flex);
        
        let html = elem.render();
        assert!(html.contains("px-4"));
        assert!(html.contains("my-2"));
        assert!(html.contains("flex"));
    }

    #[test]
    fn test_responsive_variants() {
        let elem = Element::<Div>::new()
            .tw(Padding::X(4))
            .tw_md(Padding::X(8));
        
        let html = elem.render();
        assert!(html.contains("px-4"));
        assert!(html.contains("md:px-8"));
    }

    #[test]
    fn test_state_variants() {
        let elem = Element::<Div>::new()
            .tw(TextColor::Blue(500))
            .tw_hover(TextColor::Blue(700));
        
        let html = elem.render();
        assert!(html.contains("text-blue-500"));
        assert!(html.contains("hover:text-blue-700"));
    }

    #[test]
    fn test_raw_escape_hatch() {
        let elem = Element::<Div>::new()
            .tw(Padding::X(4))
            .tw_raw("custom-class");
        
        let html = elem.render();
        assert!(html.contains("px-4"));
        assert!(html.contains("custom-class"));
    }
    
    #[test]
    fn test_multiple_utilities() {
        let elem = Element::<Div>::new()
            .tw(Padding::All(4))
            .tw(Margin::X(2))
            .tw(BackgroundColor::Blue(500))
            .tw(TextColor::White)
            .tw(BorderRadius::Lg)
            .tw(Shadow::Md);
        
        let html = elem.render();
        assert!(html.contains("p-4"));
        assert!(html.contains("mx-2"));
        assert!(html.contains("bg-blue-500"));
        assert!(html.contains("text-white"));
        assert!(html.contains("rounded-lg"));
        assert!(html.contains("shadow-md"));
    }
    
    #[test]
    fn test_flexbox() {
        let elem = Element::<Div>::new()
            .tw(Display::Flex)
            .tw(FlexDirection::Col)
            .tw(JustifyContent::Center)
            .tw(AlignItems::Center)
            .tw(Gap::All(4));
        
        let html = elem.render();
        assert!(html.contains("flex"));
        assert!(html.contains("flex-col"));
        assert!(html.contains("justify-center"));
        assert!(html.contains("items-center"));
        assert!(html.contains("gap-4"));
    }
}
