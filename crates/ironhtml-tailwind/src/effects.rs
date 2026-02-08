//! Effects utilities: shadows and opacity

use crate::TailwindClass;
use alloc::string::String;

/// Shadow utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shadow {
    /// shadow-sm
    Sm,
    /// shadow (default)
    Default,
    /// shadow-md
    Md,
    /// shadow-lg
    Lg,
    /// shadow-xl
    Xl,
    /// shadow-2xl
    Xl2,
    /// shadow-inner
    Inner,
    /// shadow-none
    None,
}

impl TailwindClass for Shadow {
    fn to_class(&self) -> String {
        match self {
            Self::Sm => "shadow-sm".into(),
            Self::Default => "shadow".into(),
            Self::Md => "shadow-md".into(),
            Self::Lg => "shadow-lg".into(),
            Self::Xl => "shadow-xl".into(),
            Self::Xl2 => "shadow-2xl".into(),
            Self::Inner => "shadow-inner".into(),
            Self::None => "shadow-none".into(),
        }
    }
}

/// Opacity utilities
///
/// See [Tailwind CSS Opacity Documentation](https://tailwindcss.com/docs/opacity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opacity {
    /// opacity-0
    O0,
    /// opacity-5
    O5,
    /// opacity-10
    O10,
    /// opacity-20
    O20,
    /// opacity-25
    O25,
    /// opacity-30
    O30,
    /// opacity-40
    O40,
    /// opacity-50
    O50,
    /// opacity-60
    O60,
    /// opacity-70
    O70,
    /// opacity-75
    O75,
    /// opacity-80
    O80,
    /// opacity-90
    O90,
    /// opacity-95
    O95,
    /// opacity-100
    O100,
}

impl TailwindClass for Opacity {
    fn to_class(&self) -> String {
        match self {
            Self::O0 => "opacity-0".into(),
            Self::O5 => "opacity-5".into(),
            Self::O10 => "opacity-10".into(),
            Self::O20 => "opacity-20".into(),
            Self::O25 => "opacity-25".into(),
            Self::O30 => "opacity-30".into(),
            Self::O40 => "opacity-40".into(),
            Self::O50 => "opacity-50".into(),
            Self::O60 => "opacity-60".into(),
            Self::O70 => "opacity-70".into(),
            Self::O75 => "opacity-75".into(),
            Self::O80 => "opacity-80".into(),
            Self::O90 => "opacity-90".into(),
            Self::O95 => "opacity-95".into(),
            Self::O100 => "opacity-100".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow() {
        assert_eq!(Shadow::Sm.to_class(), "shadow-sm");
        assert_eq!(Shadow::Default.to_class(), "shadow");
        assert_eq!(Shadow::Lg.to_class(), "shadow-lg");
        assert_eq!(Shadow::None.to_class(), "shadow-none");
    }

    #[test]
    fn test_opacity() {
        assert_eq!(Opacity::O0.to_class(), "opacity-0");
        assert_eq!(Opacity::O50.to_class(), "opacity-50");
        assert_eq!(Opacity::O100.to_class(), "opacity-100");
    }
}
