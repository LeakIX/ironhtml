//! Background utilities

use alloc::format;
use alloc::string::String;
use crate::TailwindClass;

/// Background color utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundColor {
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

impl TailwindClass for BackgroundColor {
    fn to_class(&self) -> String {
        match self {
            Self::Slate(n) => format!("bg-slate-{n}"),
            Self::Gray(n) => format!("bg-gray-{n}"),
            Self::Zinc(n) => format!("bg-zinc-{n}"),
            Self::Neutral(n) => format!("bg-neutral-{n}"),
            Self::Stone(n) => format!("bg-stone-{n}"),
            Self::Red(n) => format!("bg-red-{n}"),
            Self::Orange(n) => format!("bg-orange-{n}"),
            Self::Amber(n) => format!("bg-amber-{n}"),
            Self::Yellow(n) => format!("bg-yellow-{n}"),
            Self::Lime(n) => format!("bg-lime-{n}"),
            Self::Green(n) => format!("bg-green-{n}"),
            Self::Emerald(n) => format!("bg-emerald-{n}"),
            Self::Teal(n) => format!("bg-teal-{n}"),
            Self::Cyan(n) => format!("bg-cyan-{n}"),
            Self::Sky(n) => format!("bg-sky-{n}"),
            Self::Blue(n) => format!("bg-blue-{n}"),
            Self::Indigo(n) => format!("bg-indigo-{n}"),
            Self::Violet(n) => format!("bg-violet-{n}"),
            Self::Purple(n) => format!("bg-purple-{n}"),
            Self::Fuchsia(n) => format!("bg-fuchsia-{n}"),
            Self::Pink(n) => format!("bg-pink-{n}"),
            Self::Rose(n) => format!("bg-rose-{n}"),
            Self::Black => "bg-black".into(),
            Self::White => "bg-white".into(),
            Self::Transparent => "bg-transparent".into(),
            Self::Current => "bg-current".into(),
            Self::Inherit => "bg-inherit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_color() {
        assert_eq!(BackgroundColor::Blue(500).to_class(), "bg-blue-500");
        assert_eq!(BackgroundColor::Gray(100).to_class(), "bg-gray-100");
        assert_eq!(BackgroundColor::Red(600).to_class(), "bg-red-600");
        assert_eq!(BackgroundColor::Transparent.to_class(), "bg-transparent");
        assert_eq!(BackgroundColor::White.to_class(), "bg-white");
    }
}
