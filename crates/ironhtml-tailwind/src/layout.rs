//! Layout utilities: display, position, overflow

use alloc::string::String;
use crate::TailwindClass;

/// Display utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    InlineBlock,
    Inline,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    Table,
    TableRow,
    TableCell,
    Hidden,
    Contents,
    FlowRoot,
}

impl TailwindClass for Display {
    fn to_class(&self) -> String {
        match self {
            Self::Block => "block".into(),
            Self::InlineBlock => "inline-block".into(),
            Self::Inline => "inline".into(),
            Self::Flex => "flex".into(),
            Self::InlineFlex => "inline-flex".into(),
            Self::Grid => "grid".into(),
            Self::InlineGrid => "inline-grid".into(),
            Self::Table => "table".into(),
            Self::TableRow => "table-row".into(),
            Self::TableCell => "table-cell".into(),
            Self::Hidden => "hidden".into(),
            Self::Contents => "contents".into(),
            Self::FlowRoot => "flow-root".into(),
        }
    }
}

/// Position utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Fixed,
    Absolute,
    Relative,
    Sticky,
}

impl TailwindClass for Position {
    fn to_class(&self) -> String {
        match self {
            Self::Static => "static".into(),
            Self::Fixed => "fixed".into(),
            Self::Absolute => "absolute".into(),
            Self::Relative => "relative".into(),
            Self::Sticky => "sticky".into(),
        }
    }
}

/// Overflow utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
    /// X-axis specific
    XAuto,
    XHidden,
    XClip,
    XVisible,
    XScroll,
    /// Y-axis specific
    YAuto,
    YHidden,
    YClip,
    YVisible,
    YScroll,
}

impl TailwindClass for Overflow {
    fn to_class(&self) -> String {
        match self {
            Self::Auto => "overflow-auto".into(),
            Self::Hidden => "overflow-hidden".into(),
            Self::Clip => "overflow-clip".into(),
            Self::Visible => "overflow-visible".into(),
            Self::Scroll => "overflow-scroll".into(),
            Self::XAuto => "overflow-x-auto".into(),
            Self::XHidden => "overflow-x-hidden".into(),
            Self::XClip => "overflow-x-clip".into(),
            Self::XVisible => "overflow-x-visible".into(),
            Self::XScroll => "overflow-x-scroll".into(),
            Self::YAuto => "overflow-y-auto".into(),
            Self::YHidden => "overflow-y-hidden".into(),
            Self::YClip => "overflow-y-clip".into(),
            Self::YVisible => "overflow-y-visible".into(),
            Self::YScroll => "overflow-y-scroll".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Display::Block.to_class(), "block");
        assert_eq!(Display::Flex.to_class(), "flex");
        assert_eq!(Display::Grid.to_class(), "grid");
        assert_eq!(Display::Hidden.to_class(), "hidden");
        assert_eq!(Display::InlineBlock.to_class(), "inline-block");
    }

    #[test]
    fn test_position() {
        assert_eq!(Position::Static.to_class(), "static");
        assert_eq!(Position::Fixed.to_class(), "fixed");
        assert_eq!(Position::Absolute.to_class(), "absolute");
        assert_eq!(Position::Relative.to_class(), "relative");
        assert_eq!(Position::Sticky.to_class(), "sticky");
    }

    #[test]
    fn test_overflow() {
        assert_eq!(Overflow::Auto.to_class(), "overflow-auto");
        assert_eq!(Overflow::Hidden.to_class(), "overflow-hidden");
        assert_eq!(Overflow::XAuto.to_class(), "overflow-x-auto");
        assert_eq!(Overflow::YHidden.to_class(), "overflow-y-hidden");
    }
}
