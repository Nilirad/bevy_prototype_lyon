//! Conversions between `bevy` and `lyon` datatypes.
use bevy::math::Vec2;
use lyon_tessellation::math::{Point, Vector};

pub trait ToLyonPoint {
    fn to_lyon_point(self) -> Point;
}

pub trait ToLyonVector {
    fn to_lyon_vector(self) -> Vector;
}

impl ToLyonPoint for Vec2 {
    fn to_lyon_point(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl ToLyonVector for Vec2 {
    fn to_lyon_vector(self) -> Vector {
        Vector::new(self.x, self.y)
    }
}
