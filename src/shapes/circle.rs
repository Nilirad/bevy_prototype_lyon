//! Tools for drawing circles.

use bevy::math::Vec2;
use lyon_tessellation::path::{path::Builder, Winding};

use crate::{geometry::Geometry, utils::ToPoint};

/// A shape where all points are equidistant from a fixed point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    /// The distance from the [`center`] of any point of the circumference.
    ///
    /// [`center`]: Self::center
    pub radius: f32,
    /// Point equidistant from all points of the circumference.
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
