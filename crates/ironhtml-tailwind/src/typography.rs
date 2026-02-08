//! Typography utilities: font size, weight, text align, and color

use alloc::format;
use alloc::string::String;
use crate::TailwindClass;

/// Font size utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSize {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
    Xl7,
    Xl8,
    Xl9,
}

impl TailwindClass for FontSize {
    fn to_class(&self) -> String {
        match self {
            Self::Xs => "text-xs".into(),
            Self::Sm => "text-sm".into(),
            Self::Base => "text-base".into(),
            Self::Lg => "text-lg".into(),
            Self::Xl => "text-xl".into(),
            Self::Xl2 => "text-2xl".into(),
            Self::Xl3 => "text-3xl".into(),
            Self::Xl4 => "text-4xl".into(),
            Self::Xl5 => "text-5xl".into(),
            Self::Xl6 => "text-6xl".into(),
            Self::Xl7 => "text-7xl".into(),
            Self::Xl8 => "text-8xl".into(),
            Self::Xl9 => "text-9xl".into(),
        }
    }
}

/// Font weight utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl TailwindClass for FontWeight {
    fn to_class(&self) -> String {
        match self {
            Self::Thin => "font-thin".into(),
            Self::ExtraLight => "font-extralight".into(),
            Self::Light => "font-light".into(),
            Self::Normal => "font-normal".into(),
            Self::Medium => "font-medium".into(),
            Self::SemiBold => "font-semibold".into(),
            Self::Bold => "font-bold".into(),
            Self::ExtraBold => "font-extrabold".into(),
            Self::Black => "font-black".into(),
        }
    }
}

/// Text alignment utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

impl TailwindClass for TextAlign {
    fn to_class(&self) -> String {
        match self {
            Self::Left => "text-left".into(),
            Self::Center => "text-center".into(),
            Self::Right => "text-right".into(),
            Self::Justify => "text-justify".into(),
            Self::Start => "text-start".into(),
            Self::End => "text-end".into(),
        }
    }
}

/// Text color utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextColor {
    // Slate
    Slate(u16),
    // Gray
    Gray(u16),
    // Zinc
    Zinc(u16),
    // Neutral
    Neutral(u16),
    // Stone
    Stone(u16),
    // Red
    Red(u16),
    // Orange
    Orange(u16),
    // Amber
    Amber(u16),
    // Yellow
    Yellow(u16),
    // Lime
    Lime(u16),
    // Green
    Green(u16),
    // Emerald
    Emerald(u16),
    // Teal
    Teal(u16),
    // Cyan
    Cyan(u16),
    // Sky
    Sky(u16),
    // Blue
    Blue(u16),
    // Indigo
    Indigo(u16),
    // Violet
    Violet(u16),
    // Purple
    Purple(u16),
    // Fuchsia
    Fuchsia(u16),
    // Pink
    Pink(u16),
    // Rose
    Rose(u16),
    // Special colors
    Black,
    White,
    Transparent,
    Current,
    Inherit,
}

impl TailwindClass for TextColor {
    fn to_class(&self) -> String {
        match self {
            Self::Slate(n) => format!("text-slate-{n}"),
            Self::Gray(n) => format!("text-gray-{n}"),
            Self::Zinc(n) => format!("text-zinc-{n}"),
            Self::Neutral(n) => format!("text-neutral-{n}"),
            Self::Stone(n) => format!("text-stone-{n}"),
            Self::Red(n) => format!("text-red-{n}"),
            Self::Orange(n) => format!("text-orange-{n}"),
            Self::Amber(n) => format!("text-amber-{n}"),
            Self::Yellow(n) => format!("text-yellow-{n}"),
            Self::Lime(n) => format!("text-lime-{n}"),
            Self::Green(n) => format!("text-green-{n}"),
            Self::Emerald(n) => format!("text-emerald-{n}"),
            Self::Teal(n) => format!("text-teal-{n}"),
            Self::Cyan(n) => format!("text-cyan-{n}"),
            Self::Sky(n) => format!("text-sky-{n}"),
            Self::Blue(n) => format!("text-blue-{n}"),
            Self::Indigo(n) => format!("text-indigo-{n}"),
            Self::Violet(n) => format!("text-violet-{n}"),
            Self::Purple(n) => format!("text-purple-{n}"),
            Self::Fuchsia(n) => format!("text-fuchsia-{n}"),
            Self::Pink(n) => format!("text-pink-{n}"),
            Self::Rose(n) => format!("text-rose-{n}"),
            Self::Black => "text-black".into(),
            Self::White => "text-white".into(),
            Self::Transparent => "text-transparent".into(),
            Self::Current => "text-current".into(),
            Self::Inherit => "text-inherit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_size() {
        assert_eq!(FontSize::Xs.to_class(), "text-xs");
        assert_eq!(FontSize::Base.to_class(), "text-base");
        assert_eq!(FontSize::Xl.to_class(), "text-xl");
        assert_eq!(FontSize::Xl2.to_class(), "text-2xl");
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Thin.to_class(), "font-thin");
        assert_eq!(FontWeight::Normal.to_class(), "font-normal");
        assert_eq!(FontWeight::Bold.to_class(), "font-bold");
        assert_eq!(FontWeight::Black.to_class(), "font-black");
    }

    #[test]
    fn test_text_align() {
        assert_eq!(TextAlign::Left.to_class(), "text-left");
        assert_eq!(TextAlign::Center.to_class(), "text-center");
        assert_eq!(TextAlign::Right.to_class(), "text-right");
        assert_eq!(TextAlign::Justify.to_class(), "text-justify");
    }

    #[test]
    fn test_text_color() {
        assert_eq!(TextColor::Blue(500).to_class(), "text-blue-500");
        assert_eq!(TextColor::Red(700).to_class(), "text-red-700");
        assert_eq!(TextColor::Gray(300).to_class(), "text-gray-300");
        assert_eq!(TextColor::Black.to_class(), "text-black");
        assert_eq!(TextColor::White.to_class(), "text-white");
    }
}
