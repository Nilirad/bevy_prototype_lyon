//! Types for defining shape color and options.

use bevy::render::color::Color;
use lyon_tessellation::{FillOptions, StrokeOptions};

/// Determines how a shape will be drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawMode {
    /// The shape will be filled using the provided [`FillMode`].
    Fill(FillMode),
    /// The shape will be stroked using the provided [`StrokeMode`].
    Stroke(StrokeMode),
    /// The shape will be filled with an outline.
    Outlined {
        /// Properties about the filling.
        fill_mode: FillMode,
        /// Properties about the outline.
        outline_mode: StrokeMode,
    },
}

/// Defines the fill options for the lyon tessellator and color of the generated
/// vertices.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FillMode {
    pub options: FillOptions,
    pub color: Color,
}

impl FillMode {
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeMode {
    pub options: StrokeOptions,
    pub color: Color,
}

impl StrokeMode {
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
