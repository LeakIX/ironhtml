//! Border utilities

use alloc::format;
use alloc::string::String;
use crate::TailwindClass;

/// Border width utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderWidth {
    /// border (1px)
    Default,
    /// border-0
    None,
    /// border-2
    Width2,
    /// border-4
    Width4,
    /// border-8
    Width8,
    /// border-x-{n}
    X(u8),
    /// border-y-{n}
    Y(u8),
    /// border-t-{n}
    T(u8),
    /// border-b-{n}
    B(u8),
    /// border-l-{n}
    L(u8),
    /// border-r-{n}
    R(u8),
}

impl TailwindClass for BorderWidth {
    fn to_class(&self) -> String {
        match self {
            Self::Default => "border".into(),
            Self::None => "border-0".into(),
            Self::Width2 => "border-2".into(),
            Self::Width4 => "border-4".into(),
            Self::Width8 => "border-8".into(),
            Self::X(n) if *n == 0 => "border-x-0".into(),
            Self::X(n) if *n == 2 => "border-x-2".into(),
            Self::X(n) if *n == 4 => "border-x-4".into(),
            Self::X(n) if *n == 8 => "border-x-8".into(),
            Self::X(_) => "border-x".into(),
            Self::Y(n) if *n == 0 => "border-y-0".into(),
            Self::Y(n) if *n == 2 => "border-y-2".into(),
            Self::Y(n) if *n == 4 => "border-y-4".into(),
            Self::Y(n) if *n == 8 => "border-y-8".into(),
            Self::Y(_) => "border-y".into(),
            Self::T(n) if *n == 0 => "border-t-0".into(),
            Self::T(n) if *n == 2 => "border-t-2".into(),
            Self::T(n) if *n == 4 => "border-t-4".into(),
            Self::T(n) if *n == 8 => "border-t-8".into(),
            Self::T(_) => "border-t".into(),
            Self::B(n) if *n == 0 => "border-b-0".into(),
            Self::B(n) if *n == 2 => "border-b-2".into(),
            Self::B(n) if *n == 4 => "border-b-4".into(),
            Self::B(n) if *n == 8 => "border-b-8".into(),
            Self::B(_) => "border-b".into(),
            Self::L(n) if *n == 0 => "border-l-0".into(),
            Self::L(n) if *n == 2 => "border-l-2".into(),
            Self::L(n) if *n == 4 => "border-l-4".into(),
            Self::L(n) if *n == 8 => "border-l-8".into(),
            Self::L(_) => "border-l".into(),
            Self::R(n) if *n == 0 => "border-r-0".into(),
            Self::R(n) if *n == 2 => "border-r-2".into(),
            Self::R(n) if *n == 4 => "border-r-4".into(),
            Self::R(n) if *n == 8 => "border-r-8".into(),
            Self::R(_) => "border-r".into(),
        }
    }
}

/// Border color utilities (same as text colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderColor {
    Slate(u16),
    Gray(u16),
    Zinc(u16),
    Neutral(u16),
    Stone(u16),
    Red(u16),
    Orange(u16),
    Amber(u16),
    Yellow(u16),
    Lime(u16),
    Green(u16),
    Emerald(u16),
    Teal(u16),
    Cyan(u16),
    Sky(u16),
    Blue(u16),
    Indigo(u16),
    Violet(u16),
    Purple(u16),
    Fuchsia(u16),
    Pink(u16),
    Rose(u16),
    Black,
    White,
    Transparent,
    Current,
    Inherit,
}

impl TailwindClass for BorderColor {
    fn to_class(&self) -> String {
        match self {
            Self::Slate(n) => format!("border-slate-{n}"),
            Self::Gray(n) => format!("border-gray-{n}"),
            Self::Zinc(n) => format!("border-zinc-{n}"),
            Self::Neutral(n) => format!("border-neutral-{n}"),
            Self::Stone(n) => format!("border-stone-{n}"),
            Self::Red(n) => format!("border-red-{n}"),
            Self::Orange(n) => format!("border-orange-{n}"),
            Self::Amber(n) => format!("border-amber-{n}"),
            Self::Yellow(n) => format!("border-yellow-{n}"),
            Self::Lime(n) => format!("border-lime-{n}"),
            Self::Green(n) => format!("border-green-{n}"),
            Self::Emerald(n) => format!("border-emerald-{n}"),
            Self::Teal(n) => format!("border-teal-{n}"),
            Self::Cyan(n) => format!("border-cyan-{n}"),
            Self::Sky(n) => format!("border-sky-{n}"),
            Self::Blue(n) => format!("border-blue-{n}"),
            Self::Indigo(n) => format!("border-indigo-{n}"),
            Self::Violet(n) => format!("border-violet-{n}"),
            Self::Purple(n) => format!("border-purple-{n}"),
            Self::Fuchsia(n) => format!("border-fuchsia-{n}"),
            Self::Pink(n) => format!("border-pink-{n}"),
            Self::Rose(n) => format!("border-rose-{n}"),
            Self::Black => "border-black".into(),
            Self::White => "border-white".into(),
            Self::Transparent => "border-transparent".into(),
            Self::Current => "border-current".into(),
            Self::Inherit => "border-inherit".into(),
        }
    }
}

/// Border radius utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderRadius {
    /// rounded-none
    None,
    /// rounded-sm
    Sm,
    /// rounded
    Default,
    /// rounded-md
    Md,
    /// rounded-lg
    Lg,
    /// rounded-xl
    Xl,
    /// rounded-2xl
    Xl2,
    /// rounded-3xl
    Xl3,
    /// rounded-full
    Full,
}

impl TailwindClass for BorderRadius {
    fn to_class(&self) -> String {
        match self {
            Self::None => "rounded-none".into(),
            Self::Sm => "rounded-sm".into(),
            Self::Default => "rounded".into(),
            Self::Md => "rounded-md".into(),
            Self::Lg => "rounded-lg".into(),
            Self::Xl => "rounded-xl".into(),
            Self::Xl2 => "rounded-2xl".into(),
            Self::Xl3 => "rounded-3xl".into(),
            Self::Full => "rounded-full".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_width() {
        assert_eq!(BorderWidth::Default.to_class(), "border");
        assert_eq!(BorderWidth::None.to_class(), "border-0");
        assert_eq!(BorderWidth::Width2.to_class(), "border-2");
        assert_eq!(BorderWidth::X(2).to_class(), "border-x-2");
        assert_eq!(BorderWidth::Y(4).to_class(), "border-y-4");
    }

    #[test]
    fn test_border_color() {
        assert_eq!(BorderColor::Blue(500).to_class(), "border-blue-500");
        assert_eq!(BorderColor::Gray(200).to_class(), "border-gray-200");
        assert_eq!(BorderColor::Transparent.to_class(), "border-transparent");
    }

    #[test]
    fn test_border_radius() {
        assert_eq!(BorderRadius::None.to_class(), "rounded-none");
        assert_eq!(BorderRadius::Default.to_class(), "rounded");
        assert_eq!(BorderRadius::Lg.to_class(), "rounded-lg");
        assert_eq!(BorderRadius::Full.to_class(), "rounded-full");
    }
}
