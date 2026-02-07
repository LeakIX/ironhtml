//! Shared type definitions for Bootstrap components.

use core::fmt;

/// Bootstrap contextual colors.
///
/// Used for buttons, alerts, badges, and other components.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::Color;
///
/// let color = Color::Primary;
/// assert_eq!(color.as_str(), "primary");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Color {
    /// Returns the Bootstrap class suffix for this color.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Danger => "danger",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Bootstrap component sizes.
///
/// ## Example
///
/// ```rust
/// use ironhtml_bootstrap::Size;
///
/// let size = Size::Large;
/// assert_eq!(size.as_btn_class(), "btn-lg");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Size {
    Small,
    #[default]
    Normal,
    Large,
}

impl Size {
    /// Returns the button size class (e.g., "btn-sm", "btn-lg").
    /// Returns empty string for normal size.
    #[must_use]
    pub const fn as_btn_class(self) -> &'static str {
        match self {
            Self::Small => "btn-sm",
            Self::Normal => "",
            Self::Large => "btn-lg",
        }
    }
}

/// Bootstrap responsive breakpoints.
///
/// | Breakpoint | Size     | Class infix |
/// |------------|----------|-------------|
/// | Sm         | >=576px  | sm          |
/// | Md         | >=768px  | md          |
/// | Lg         | >=992px  | lg          |
/// | Xl         | >=1200px | xl          |
/// | Xxl        | >=1400px | xxl         |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    /// Small devices (>=576px)
    Sm,
    /// Medium devices (>=768px)
    Md,
    /// Large devices (>=992px)
    Lg,
    /// Extra large devices (>=1200px)
    Xl,
    /// Extra extra large devices (>=1400px)
    Xxl,
}

impl Breakpoint {
    /// Returns the Bootstrap class infix for this breakpoint.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
            Self::Xxl => "xxl",
        }
    }
}

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Navbar expand breakpoint.
///
/// Determines at which screen size the navbar expands from collapsed to horizontal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NavbarExpand {
    /// Always expanded (navbar-expand)
    Always,
    /// Expand at small breakpoint (navbar-expand-sm)
    Sm,
    /// Expand at medium breakpoint (navbar-expand-md)
    Md,
    /// Expand at large breakpoint (navbar-expand-lg) - most common
    #[default]
    Lg,
    /// Expand at extra large breakpoint (navbar-expand-xl)
    Xl,
    /// Expand at extra extra large breakpoint (navbar-expand-xxl)
    Xxl,
}

impl NavbarExpand {
    /// Returns the Bootstrap navbar expand class.
    #[must_use]
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Always => "navbar-expand",
            Self::Sm => "navbar-expand-sm",
            Self::Md => "navbar-expand-md",
            Self::Lg => "navbar-expand-lg",
            Self::Xl => "navbar-expand-xl",
            Self::Xxl => "navbar-expand-xxl",
        }
    }
}
