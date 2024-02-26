//! Types for defining shape color and options.

use bevy::{ecs::component::Component, render::color::LegacyColor};
use lyon_tessellation::{FillOptions, StrokeOptions};

/// Defines the fill options for the lyon tessellator and color of the generated
/// vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Fill {
    pub options: FillOptions,
    pub color: LegacyColor,
}

impl Fill {
    /// Convenience constructor requiring only the `LegacyColor`.
    #[must_use]
    pub fn color(color: LegacyColor) -> Self {
        Self {
            options: FillOptions::default(),
            color,
        }
    }
}

/// Defines the stroke options for the lyon tessellator and color of the
/// generated vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    pub options: StrokeOptions,
    pub color: LegacyColor,
}

impl Stroke {
    /// Constructor that requires a `LegacyColor` and a line width.
    #[must_use]
    pub fn new(color: LegacyColor, line_width: f32) -> Self {
        Self {
            options: StrokeOptions::default().with_line_width(line_width),
            color,
        }
    }

    /// Convenience constructor requiring only the `LegacyColor`.
    #[must_use]
    pub fn color(color: LegacyColor) -> Self {
        Self {
            options: StrokeOptions::default(),
            color,
        }
    }
}
