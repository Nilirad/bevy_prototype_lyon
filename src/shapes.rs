//! Collection of common shapes that can be drawn.
//!
//! The structs defined in this module implement the
//! [`ShapeSprite`](crate::plugin::ShapeSprite) trait. You can also implement
//! the trait for your own shapes.

use crate::{
    conversions::{ToLyonPoint, ToLyonVector},
    plugin::ShapeSprite,
};
use bevy::math::Vec2;
use lyon_tessellation::{
    math::{Angle, Point, Rect, Size},
    path::{path::Builder, traits::PathBuilder, Path, Polygon as LyonPolygon, Winding},
};

/// Defines where the origin, or pivot of the `Rectangle` should be positioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectangleOrigin {
    Center,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
}

impl Default for RectangleOrigin {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub origin: RectangleOrigin,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            origin: RectangleOrigin::default(),
        }
    }
}

impl ShapeSprite for Rectangle {
    fn generate_path(&self) -> Path {
        let origin = {
            use RectangleOrigin::*;
            match self.origin {
                Center => Point::new(-self.width / 2.0, -self.height / 2.0),
                BottomLeft => Point::new(0.0, 0.0),
                BottomRight => Point::new(-self.width, 0.0),
                TopRight => Point::new(-self.width, -self.height),
                TopLeft => Point::new(0.0, -self.height),
            }
        };

        let mut path_builder = Builder::new();
        path_builder.add_rectangle(
            &Rect::new(origin, Size::new(self.width, self.height)),
            Winding::Positive,
        );
        path_builder.build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    /// Distance of the border of the circle from the center.
    pub radius: f32,
    /// The position of the center of the circle, relative to the world
    /// translation of the [`SpriteBundle`](bevy::sprite::entity::SpriteBundle).
    pub center: Vec2,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for Circle {
    fn generate_path(&self) -> Path {
        let mut path_builder = Builder::new();
        path_builder.add_circle(self.center.to_lyon_point(), self.radius, Winding::Positive);
        path_builder.build()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub radii: Vec2,
    /// The position of the center of the ellipse, relative to the world
    /// translation of the [`SpriteBundle`](bevy::sprite::entity::SpriteBundle).
    pub center: Vec2,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            radii: Vec2::one(),
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for Ellipse {
    fn generate_path(&self) -> Path {
        let mut path_builder = Builder::new();
        path_builder.add_ellipse(
            self.center.to_lyon_point(),
            self.radii.to_lyon_vector(),
            Angle::zero(),
            Winding::Positive,
        );
        path_builder.build()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub closed: bool,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            closed: true,
        }
    }
}

impl ShapeSprite for Polygon {
    fn generate_path(&self) -> Path {
        let points = self
            .points
            .iter()
            .map(|p| p.to_lyon_point())
            .collect::<Vec<Point>>();
        let polygon: LyonPolygon<Point> = LyonPolygon {
            points: points.as_slice(),
            closed: self.closed,
        };

        let mut path_builder = Builder::new();
        path_builder.add_polygon(polygon);
        path_builder.build()
    }
}
