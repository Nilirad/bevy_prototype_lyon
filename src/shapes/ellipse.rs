use bevy::math::Vec2;
use lyon_tessellation::{
    math::Angle,
    path::{path::Builder, Winding},
};

use crate::{
    geometry::Geometry,
    utils::{ToPoint, ToVector},
};

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub radii: Vec2,
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
