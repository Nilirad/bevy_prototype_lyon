//! Conversions between Bevy and Lyon datatypes.
use bevy_math::Vec2;
use lyon_tessellation::math::{Point, Vector};

/// A locally defined [std::convert::Into] surrogate to overcome orphan rules.
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
