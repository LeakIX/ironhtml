//! Flexbox utilities

use crate::TailwindClass;
use alloc::string::String;

/// Flex direction utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Col,
    ColReverse,
}

impl TailwindClass for FlexDirection {
    fn to_class(&self) -> String {
        match self {
            Self::Row => "flex-row".into(),
            Self::RowReverse => "flex-row-reverse".into(),
            Self::Col => "flex-col".into(),
            Self::ColReverse => "flex-col-reverse".into(),
        }
    }
}

/// Justify content utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
    Stretch,
}

impl TailwindClass for JustifyContent {
    fn to_class(&self) -> String {
        match self {
            Self::Start => "justify-start".into(),
            Self::End => "justify-end".into(),
            Self::Center => "justify-center".into(),
            Self::Between => "justify-between".into(),
            Self::Around => "justify-around".into(),
            Self::Evenly => "justify-evenly".into(),
            Self::Stretch => "justify-stretch".into(),
        }
    }
}

/// Align items utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl TailwindClass for AlignItems {
    fn to_class(&self) -> String {
        match self {
            Self::Start => "items-start".into(),
            Self::End => "items-end".into(),
            Self::Center => "items-center".into(),
            Self::Baseline => "items-baseline".into(),
            Self::Stretch => "items-stretch".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction() {
        assert_eq!(FlexDirection::Row.to_class(), "flex-row");
        assert_eq!(FlexDirection::Col.to_class(), "flex-col");
        assert_eq!(FlexDirection::RowReverse.to_class(), "flex-row-reverse");
    }

    #[test]
    fn test_justify_content() {
        assert_eq!(JustifyContent::Start.to_class(), "justify-start");
        assert_eq!(JustifyContent::Center.to_class(), "justify-center");
        assert_eq!(JustifyContent::Between.to_class(), "justify-between");
    }

    #[test]
    fn test_align_items() {
        assert_eq!(AlignItems::Start.to_class(), "items-start");
        assert_eq!(AlignItems::Center.to_class(), "items-center");
        assert_eq!(AlignItems::Stretch.to_class(), "items-stretch");
    }
}
