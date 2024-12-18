use bevy::math::Vec2;
use lyon_tessellation::path::{path::Builder, Polygon as LyonPolygon};

use crate::{geometry::Geometry, utils::ToPoint};

/// A simple line segment, specified by two points.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(pub Vec2, pub Vec2);

impl Geometry<Builder> for Line {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_polygon(LyonPolygon {
            points: &[self.0.to_point(), self.1.to_point()],
            closed: false,
        });
    }
}
