//! Shared type definitions for Bootstrap components.

use core::fmt;

/// Bootstrap contextual colors.
///
/// Used for buttons, alerts, badges, and other components.
///
/// ## Example
///
/// ```rust
/// use html_bootstrap::Color;
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
    pub const fn as_str(&self) -> &'static str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Success => "success",
            Color::Danger => "danger",
            Color::Warning => "warning",
            Color::Info => "info",
            Color::Light => "light",
            Color::Dark => "dark",
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
/// use html_bootstrap::Size;
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
    pub const fn as_btn_class(&self) -> &'static str {
        match self {
            Size::Small => "btn-sm",
            Size::Normal => "",
            Size::Large => "btn-lg",
        }
    }
}

/// Bootstrap responsive breakpoints.
///
/// | Breakpoint | Size     | Class infix |
/// |------------|----------|-------------|
/// | Sm         | ≥576px   | sm          |
/// | Md         | ≥768px   | md          |
/// | Lg         | ≥992px   | lg          |
/// | Xl         | ≥1200px  | xl          |
/// | Xxl        | ≥1400px  | xxl         |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    /// Small devices (≥576px)
    Sm,
    /// Medium devices (≥768px)
    Md,
    /// Large devices (≥992px)
    Lg,
    /// Extra large devices (≥1200px)
    Xl,
    /// Extra extra large devices (≥1400px)
    Xxl,
}

impl Breakpoint {
    /// Returns the Bootstrap class infix for this breakpoint.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
            Breakpoint::Xxl => "xxl",
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
    pub const fn as_class(&self) -> &'static str {
        match self {
            NavbarExpand::Always => "navbar-expand",
            NavbarExpand::Sm => "navbar-expand-sm",
            NavbarExpand::Md => "navbar-expand-md",
            NavbarExpand::Lg => "navbar-expand-lg",
            NavbarExpand::Xl => "navbar-expand-xl",
            NavbarExpand::Xxl => "navbar-expand-xxl",
        }
    }
}
