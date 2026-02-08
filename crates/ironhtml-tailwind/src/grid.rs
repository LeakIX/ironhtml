//! Grid utilities

use alloc::format;
use alloc::string::String;
use crate::TailwindClass;

/// Grid columns utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCols {
    /// grid-cols-{n}
    Cols(u8),
    /// grid-cols-none
    None,
}

impl TailwindClass for GridCols {
    fn to_class(&self) -> String {
        match self {
            Self::Cols(n) => format!("grid-cols-{n}"),
            Self::None => "grid-cols-none".into(),
        }
    }
}

/// Grid rows utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridRows {
    /// grid-rows-{n}
    Rows(u8),
    /// grid-rows-none
    None,
}

impl TailwindClass for GridRows {
    fn to_class(&self) -> String {
        match self {
            Self::Rows(n) => format!("grid-rows-{n}"),
            Self::None => "grid-rows-none".into(),
        }
    }
}

/// Gap utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gap {
    /// gap-{n}
    All(u8),
    /// gap-x-{n}
    X(u8),
    /// gap-y-{n}
    Y(u8),
}

impl TailwindClass for Gap {
    fn to_class(&self) -> String {
        match self {
            Self::All(n) => format!("gap-{n}"),
            Self::X(n) => format!("gap-x-{n}"),
            Self::Y(n) => format!("gap-y-{n}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_cols() {
        assert_eq!(GridCols::Cols(3).to_class(), "grid-cols-3");
        assert_eq!(GridCols::Cols(12).to_class(), "grid-cols-12");
        assert_eq!(GridCols::None.to_class(), "grid-cols-none");
    }

    #[test]
    fn test_grid_rows() {
        assert_eq!(GridRows::Rows(2).to_class(), "grid-rows-2");
        assert_eq!(GridRows::None.to_class(), "grid-rows-none");
    }

    #[test]
    fn test_gap() {
        assert_eq!(Gap::All(4).to_class(), "gap-4");
        assert_eq!(Gap::X(2).to_class(), "gap-x-2");
        assert_eq!(Gap::Y(8).to_class(), "gap-y-8");
    }
}
