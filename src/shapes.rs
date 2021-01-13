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
    math::{point, Angle, Point, Rect, Size},
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegularPolygonFeature {
    /// The radius of the polygon's circumcircle.
    Radius(f32),
    /// The radius of the polygon's incircle.
    Apothem(f32),
    /// The length of the polygon's side.
    SideLength(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegularPolygon {
    pub sides: usize,
    pub center: Vec2,
    pub feature: RegularPolygonFeature,
}

impl RegularPolygon {
    /// Gets the radius of the polygon.
    fn radius(&self) -> f32 {
        use RegularPolygonFeature::*;
        let ratio = std::f32::consts::PI / self.sides as f32;

        match self.feature {
            Radius(r) => r,
            Apothem(a) => a * ratio.tan() / ratio.sin(),
            SideLength(s) => s / (2.0 * ratio.sin()),
        }
    }
}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self {
            sides: 3,
            center: Vec2::zero(),
            feature: RegularPolygonFeature::Radius(1.0),
        }
    }
}

impl ShapeSprite for RegularPolygon {
    fn generate_path(&self) -> Path {
        // -- Implementation details **PLEASE KEEP UPDATED** --
        // - `step`: angle between two vertices.
        // - `internal`: internal angle of the polygon.
        // - `offset`: bias to make the shape lay flat on a line parallel to the x-axis.

        use std::f32::consts::PI;
        assert!(self.sides > 2, "Polygons must have at least 3 sides");
        let n = self.sides as f32;
        let radius = self.radius();
        let internal = (n - 2.0) * PI / n;
        let offset = -internal / 2.0;

        let mut points = Vec::with_capacity(self.sides);
        let step = 2.0 * PI / n;
        for i in 0..self.sides {
            let cur_angle = offset + i as f32 * step;
            let x = self.center.x + radius * cur_angle.cos();
            let y = self.center.y + radius * cur_angle.sin();
            points.push(point(x, y));
        }

        let polygon = LyonPolygon {
            points: points.as_slice(),
            closed: true,
        };

        let mut path_builder = Builder::new();
        path_builder.add_polygon(polygon);
        path_builder.build()
    }
}
