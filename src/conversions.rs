//! Conversions between `bevy` and `lyon` datatypes.
use bevy::math::Vec2;
use lyon_tessellation::math::Point;

pub trait BevyVec2ToLyonPoint {
    fn to_lyon_point(self) -> Point;
}

impl BevyVec2ToLyonPoint for Vec2 {
    fn to_lyon_point(self) -> Point {
        Point::new(self.x, self.y)
    }
}
