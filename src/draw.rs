//! Types for defining shape color and options.

use bevy::{ecs::component::Component, render::color::Color};
use lyon_tessellation::{FillOptions, StrokeOptions};

/// Defines the fill options for the lyon tessellator and color of the generated
/// vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Fill {
    pub options: FillOptions,
    pub color: Color,
}

impl Fill {
    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: Color) -> Self {
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
    pub color: Color,
}

impl Stroke {
    /// Constructor that requires a `Color` and a line width.
    #[must_use]
    pub fn new(color: Color, line_width: f32) -> Self {
        Self {
            options: StrokeOptions::default().with_line_width(line_width),
            color,
        }
    }

    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: Color) -> Self {
        Self {
            options: StrokeOptions::default(),
            color,
        }
    }
}
