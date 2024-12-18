use bevy::math::Vec2;
use lyon_tessellation::path::{path::Builder, Winding};

use crate::{geometry::Geometry, utils::ToPoint};

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub center: Vec2,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec2::ZERO,
        }
    }
}

impl Geometry<Builder> for Circle {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_circle(self.center.to_point(), self.radius, Winding::Positive);
    }
}
