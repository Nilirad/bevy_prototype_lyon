use bevy::math::Vec2;
use lyon_tessellation::{
    geom::euclid::default::Size2D,
    math::{Box2D, Point},
    path::{builder::BorderRadii as LyonBorderRadii, path::Builder, Winding},
};

use crate::geometry::Geometry;

/// Defines where the origin, or pivot of the `Rectangle` should be positioned.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RectangleOrigin {
    Center,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
    CustomCenter(Vec2),
}

impl Default for RectangleOrigin {
    fn default() -> Self {
        Self::Center
    }
}

/// Radii for the four corners of a rectangle.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BorderRadii {
    /// Radius for the top left corner.
    pub top_left: f32,
    /// Radius for the top right corner.
    pub top_right: f32,
    /// Radius for the bottom left corner.
    pub bottom_left: f32,
    /// Radius for the bottom right corner.
    pub bottom_right: f32,
}

impl BorderRadii {
    /// Use a single radius for all corners.
    #[must_use]
    pub fn single(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    /// Use a single radius for the top corners and zero for the bottom corners.
    #[must_use]
    pub fn top(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            ..Default::default()
        }
    }

    /// Use a single radius for the bottom corners and zero for the top corners.
    #[must_use]
    pub fn bottom(radius: f32) -> Self {
        Self {
            bottom_left: radius,
            bottom_right: radius,
            ..Default::default()
        }
    }

    /// Use a single radius for the left corners and zero for the right corners.
    #[must_use]
    pub fn left(radius: f32) -> Self {
        Self {
            top_left: radius,
            bottom_left: radius,
            ..Default::default()
        }
    }

    /// Use a single radius for the right corners and zero for the left corners.
    #[must_use]
    pub fn right(radius: f32) -> Self {
        Self {
            top_right: radius,
            bottom_right: radius,
            ..Default::default()
        }
    }
}

impl From<BorderRadii> for LyonBorderRadii {
    fn from(source: BorderRadii) -> Self {
        // swap top and bottom
        Self {
            top_left: source.bottom_left.abs(),
            top_right: source.bottom_right.abs(),
            bottom_left: source.top_left.abs(),
            bottom_right: source.top_right.abs(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub extents: Vec2,
    pub origin: RectangleOrigin,
    pub radii: Option<BorderRadii>,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            extents: Vec2::ONE,
            origin: RectangleOrigin::default(),
            radii: None,
        }
    }
}

impl Geometry<Builder> for Rectangle {
    fn add_geometry(&self, b: &mut Builder) {
        let origin = match self.origin {
            RectangleOrigin::Center => Point::new(-self.extents.x / 2.0, -self.extents.y / 2.0),
            RectangleOrigin::BottomLeft => Point::new(0.0, 0.0),
            RectangleOrigin::BottomRight => Point::new(-self.extents.x, 0.0),
            RectangleOrigin::TopRight => Point::new(-self.extents.x, -self.extents.y),
            RectangleOrigin::TopLeft => Point::new(0.0, -self.extents.y),
            RectangleOrigin::CustomCenter(v) => {
                Point::new(v.x - self.extents.x / 2.0, v.y - self.extents.y / 2.0)
            }
        };
        let rect =
            &Box2D::from_origin_and_size(origin, Size2D::new(self.extents.x, self.extents.y));
        let Some(radii) = self.radii else {
            b.add_rectangle(rect, Winding::Positive);
            return;
        };
        b.add_rounded_rectangle(rect, &radii.into(), Winding::Positive);
    }
}
