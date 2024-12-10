//! Types for defining shape color and options.

use bevy::{
    color::{Color, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza},
    ecs::component::Component,
};
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
    pub fn color(color: impl Into<Color>) -> Self {
        Self {
            options: FillOptions::default(),
            color: color.into(),
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            options: FillOptions::default(),
            color: bevy::color::palettes::css::WHITE.into(),
        }
    }
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Self::color(color)
    }
}

impl From<Srgba> for Fill {
    fn from(color: Srgba) -> Self {
        Self::color(color)
    }
}

impl From<LinearRgba> for Fill {
    fn from(color: LinearRgba) -> Self {
        Self::color(color)
    }
}

impl From<Hsla> for Fill {
    fn from(color: Hsla) -> Self {
        Self::color(color)
    }
}

impl From<Hsva> for Fill {
    fn from(color: Hsva) -> Self {
        Self::color(color)
    }
}

impl From<Hwba> for Fill {
    fn from(color: Hwba) -> Self {
        Self::color(color)
    }
}

impl From<Laba> for Fill {
    fn from(color: Laba) -> Self {
        Self::color(color)
    }
}

impl From<Lcha> for Fill {
    fn from(color: Lcha) -> Self {
        Self::color(color)
    }
}

impl From<Oklaba> for Fill {
    fn from(color: Oklaba) -> Self {
        Self::color(color)
    }
}

impl From<Oklcha> for Fill {
    fn from(color: Oklcha) -> Self {
        Self::color(color)
    }
}

impl From<Xyza> for Fill {
    fn from(color: Xyza) -> Self {
        Self::color(color)
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
    pub fn new(color: impl Into<Color>, line_width: f32) -> Self {
        Self {
            options: StrokeOptions::default().with_line_width(line_width),
            color: color.into(),
        }
    }

    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: impl Into<Color>) -> Self {
        Self {
            options: StrokeOptions::default(),
            color: color.into(),
        }
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            options: StrokeOptions::default(),
            color: bevy::color::palettes::css::BLACK.into(),
        }
    }
}

impl From<(Color, f32)> for Stroke {
    fn from(value: (Color, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Srgba, f32)> for Stroke {
    fn from(value: (Srgba, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(LinearRgba, f32)> for Stroke {
    fn from(value: (LinearRgba, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Hsla, f32)> for Stroke {
    fn from(value: (Hsla, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Hsva, f32)> for Stroke {
    fn from(value: (Hsva, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Hwba, f32)> for Stroke {
    fn from(value: (Hwba, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Laba, f32)> for Stroke {
    fn from(value: (Laba, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Lcha, f32)> for Stroke {
    fn from(value: (Lcha, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Oklaba, f32)> for Stroke {
    fn from(value: (Oklaba, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Oklcha, f32)> for Stroke {
    fn from(value: (Oklcha, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Xyza, f32)> for Stroke {
    fn from(value: (Xyza, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}
