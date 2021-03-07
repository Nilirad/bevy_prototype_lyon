//! Utility types and conversion traits.

use bevy::math::Vec2;
use lyon_tessellation::{
    math::{Point, Vector},
    FillOptions, StrokeOptions,
};

/// Determines how a shape is drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawMode {
    /// The shape will be filled with the provided [`FillOptions`].
    Fill(FillOptions),
    /// The shape will be filled with the provided [`StrokeOptions`].
    Stroke(StrokeOptions),
}

/// A locally defined [`std::convert::Into`] surrogate to overcome orphan rules.
pub trait Convert<T>: Sized {
    /// Converts the value to `T`.
    fn convert(self) -> T;
}

impl Convert<Point> for Vec2 {
    fn convert(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl Convert<Vec2> for Point {
    fn convert(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Convert<Vector> for Vec2 {
    fn convert(self) -> Vector {
        Vector::new(self.x, self.y)
    }
}
