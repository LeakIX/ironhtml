//! Sizing utilities: width and height

use crate::TailwindClass;
use alloc::format;
use alloc::string::String;

/// Width utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    /// w-{n} (where n is 0-96 in multiples of 4px via Tailwind scale)
    Scaled(u8),
    /// w-auto
    Auto,
    /// w-full (100%)
    Full,
    /// w-screen (100vw)
    Screen,
    /// w-min
    Min,
    /// w-max
    Max,
    /// w-fit
    Fit,
    /// w-1/2, w-1/3, etc
    Fraction { numerator: u8, denominator: u8 },
}

impl TailwindClass for Width {
    fn to_class(&self) -> String {
        match self {
            Self::Scaled(n) => format!("w-{n}"),
            Self::Auto => "w-auto".into(),
            Self::Full => "w-full".into(),
            Self::Screen => "w-screen".into(),
            Self::Min => "w-min".into(),
            Self::Max => "w-max".into(),
            Self::Fit => "w-fit".into(),
            Self::Fraction {
                numerator,
                denominator,
            } => format!("w-{numerator}/{denominator}"),
        }
    }
}

/// Height utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Height {
    /// h-{n}
    Scaled(u8),
    /// h-auto
    Auto,
    /// h-full (100%)
    Full,
    /// h-screen (100vh)
    Screen,
    /// h-min
    Min,
    /// h-max
    Max,
    /// h-fit
    Fit,
    /// h-1/2, h-1/3, etc
    Fraction { numerator: u8, denominator: u8 },
}

impl TailwindClass for Height {
    fn to_class(&self) -> String {
        match self {
            Self::Scaled(n) => format!("h-{n}"),
            Self::Auto => "h-auto".into(),
            Self::Full => "h-full".into(),
            Self::Screen => "h-screen".into(),
            Self::Min => "h-min".into(),
            Self::Max => "h-max".into(),
            Self::Fit => "h-fit".into(),
            Self::Fraction {
                numerator,
                denominator,
            } => format!("h-{numerator}/{denominator}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        assert_eq!(Width::Scaled(64).to_class(), "w-64");
        assert_eq!(Width::Auto.to_class(), "w-auto");
        assert_eq!(Width::Full.to_class(), "w-full");
        assert_eq!(Width::Screen.to_class(), "w-screen");
        assert_eq!(
            Width::Fraction {
                numerator: 1,
                denominator: 2
            }
            .to_class(),
            "w-1/2"
        );
    }

    #[test]
    fn test_height() {
        assert_eq!(Height::Scaled(32).to_class(), "h-32");
        assert_eq!(Height::Auto.to_class(), "h-auto");
        assert_eq!(Height::Full.to_class(), "h-full");
        assert_eq!(Height::Screen.to_class(), "h-screen");
        assert_eq!(
            Height::Fraction {
                numerator: 2,
                denominator: 3
            }
            .to_class(),
            "h-2/3"
        );
    }
}
