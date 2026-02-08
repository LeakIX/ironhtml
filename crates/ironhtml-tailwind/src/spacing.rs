//! Spacing utilities: padding and margin

use alloc::format;
use alloc::string::String;
use crate::TailwindClass;

/// Padding utilities (p-*, px-*, py-*, pt-*, pb-*, pl-*, pr-*)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padding {
    /// Padding on all sides (p-*)
    All(u8),
    /// Padding left and right (px-*)
    X(u8),
    /// Padding top and bottom (py-*)
    Y(u8),
    /// Padding top (pt-*)
    T(u8),
    /// Padding bottom (pb-*)
    B(u8),
    /// Padding left (pl-*)
    L(u8),
    /// Padding right (pr-*)
    R(u8),
    /// Arbitrary padding value (p-[value])
    Arbitrary(ArbitraryValue),
}

/// Margin utilities (m-*, mx-*, my-*, mt-*, mb-*, ml-*, mr-*)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Margin {
    /// Margin on all sides (m-*)
    All(u8),
    /// Margin left and right (mx-*)
    X(u8),
    /// Margin top and bottom (my-*)
    Y(u8),
    /// Margin top (mt-*)
    T(u8),
    /// Margin bottom (mb-*)
    B(u8),
    /// Margin left (ml-*)
    L(u8),
    /// Margin right (mr-*)
    R(u8),
    /// Auto margin (m-auto, mx-auto, etc)
    Auto(MarginAxis),
    /// Arbitrary margin value (m-[value])
    Arbitrary(ArbitraryValue),
}

/// Axis for auto margins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginAxis {
    /// All sides
    All,
    /// X axis (left and right)
    X,
    /// Y axis (top and bottom)
    Y,
    /// Top only
    T,
    /// Bottom only
    B,
    /// Left only
    L,
    /// Right only
    R,
}

/// Arbitrary value for utilities (e.g., px-[13px], m-[1.5rem])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbitraryValue {
    /// The value inside brackets
    pub value: &'static str,
    /// The axis/direction
    pub axis: SpacingAxis,
}

/// Spacing axis for arbitrary values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingAxis {
    All,
    X,
    Y,
    T,
    B,
    L,
    R,
}

impl TailwindClass for Padding {
    fn to_class(&self) -> String {
        match self {
            Self::All(n) => format!("p-{n}"),
            Self::X(n) => format!("px-{n}"),
            Self::Y(n) => format!("py-{n}"),
            Self::T(n) => format!("pt-{n}"),
            Self::B(n) => format!("pb-{n}"),
            Self::L(n) => format!("pl-{n}"),
            Self::R(n) => format!("pr-{n}"),
            Self::Arbitrary(arb) => {
                let prefix = match arb.axis {
                    SpacingAxis::All => "p",
                    SpacingAxis::X => "px",
                    SpacingAxis::Y => "py",
                    SpacingAxis::T => "pt",
                    SpacingAxis::B => "pb",
                    SpacingAxis::L => "pl",
                    SpacingAxis::R => "pr",
                };
                format!("{prefix}-[{}]", arb.value)
            }
        }
    }
}

impl TailwindClass for Margin {
    fn to_class(&self) -> String {
        match self {
            Self::All(n) => format!("m-{n}"),
            Self::X(n) => format!("mx-{n}"),
            Self::Y(n) => format!("my-{n}"),
            Self::T(n) => format!("mt-{n}"),
            Self::B(n) => format!("mb-{n}"),
            Self::L(n) => format!("ml-{n}"),
            Self::R(n) => format!("mr-{n}"),
            Self::Auto(axis) => {
                let prefix = match axis {
                    MarginAxis::All => "m",
                    MarginAxis::X => "mx",
                    MarginAxis::Y => "my",
                    MarginAxis::T => "mt",
                    MarginAxis::B => "mb",
                    MarginAxis::L => "ml",
                    MarginAxis::R => "mr",
                };
                format!("{prefix}-auto")
            }
            Self::Arbitrary(arb) => {
                let prefix = match arb.axis {
                    SpacingAxis::All => "m",
                    SpacingAxis::X => "mx",
                    SpacingAxis::Y => "my",
                    SpacingAxis::T => "mt",
                    SpacingAxis::B => "mb",
                    SpacingAxis::L => "ml",
                    SpacingAxis::R => "mr",
                };
                format!("{prefix}-[{}]", arb.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        assert_eq!(Padding::All(4).to_class(), "p-4");
        assert_eq!(Padding::X(8).to_class(), "px-8");
        assert_eq!(Padding::Y(2).to_class(), "py-2");
        assert_eq!(Padding::T(1).to_class(), "pt-1");
        assert_eq!(Padding::B(3).to_class(), "pb-3");
        assert_eq!(Padding::L(6).to_class(), "pl-6");
        assert_eq!(Padding::R(12).to_class(), "pr-12");
    }

    #[test]
    fn test_margin() {
        assert_eq!(Margin::All(4).to_class(), "m-4");
        assert_eq!(Margin::X(8).to_class(), "mx-8");
        assert_eq!(Margin::Y(2).to_class(), "my-2");
        assert_eq!(Margin::T(1).to_class(), "mt-1");
        assert_eq!(Margin::B(3).to_class(), "mb-3");
        assert_eq!(Margin::L(6).to_class(), "ml-6");
        assert_eq!(Margin::R(12).to_class(), "mr-12");
    }

    #[test]
    fn test_margin_auto() {
        assert_eq!(Margin::Auto(MarginAxis::All).to_class(), "m-auto");
        assert_eq!(Margin::Auto(MarginAxis::X).to_class(), "mx-auto");
        assert_eq!(Margin::Auto(MarginAxis::Y).to_class(), "my-auto");
    }

    #[test]
    fn test_arbitrary_values() {
        let arb = ArbitraryValue {
            value: "13px",
            axis: SpacingAxis::X,
        };
        assert_eq!(Padding::Arbitrary(arb).to_class(), "px-[13px]");
        assert_eq!(Margin::Arbitrary(arb).to_class(), "mx-[13px]");
    }
}
