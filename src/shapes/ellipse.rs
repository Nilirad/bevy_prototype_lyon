//! Tools for drawing ellipses.

use bevy::math::Vec2;
use lyon_tessellation::{
    math::Angle,
    path::{path::Builder, Winding},
};

use crate::{
    geometry::Geometry,
    utils::{ToPoint, ToVector},
};

/// A shape similar to a circle under a non-uniform scaling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    /// The horizontal and vertical radii.
    pub radii: Vec2,
    /// The midpoint of the horizontal and vertical axes.
    pub center: Vec2,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            radii: Vec2::ONE,
            center: Vec2::ZERO,
        }
    }
}

impl Geometry<Builder> for Ellipse {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_ellipse(
            self.center.to_point(),
            self.radii.to_vector(),
            Angle::zero(),
            Winding::Positive,
        );
    }
}
