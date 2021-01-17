//! Collection of common shapes that can be drawn.
//!
//! The structs defined in this module implement the
//! [`ShapeSprite`](crate::plugin::ShapeSprite) trait. You can also implement
//! the trait for your own shapes.

use crate::{conversions::Convert, plugin::ShapeSprite};
use bevy_math::Vec2;
use lyon_tessellation::{
    math::{point, Angle, Point, Rect, Size},
    path::{path::Builder, traits::PathBuilder, Polygon as LyonPolygon, Winding},
};

/// Defines where the origin, or pivot of the `Rectangle` should be positioned.
#[allow(missing_docs)]
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

#[allow(missing_docs)]
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
    fn add_geometry(&self, b: &mut Builder) {
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

        b.add_rectangle(
            &Rect::new(origin, Size::new(self.width, self.height)),
            Winding::Positive,
        );
    }
}

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
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for Circle {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_circle(self.center.convert(), self.radius, Winding::Positive);
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub radii: Vec2,
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
    fn add_geometry(&self, b: &mut Builder) {
        b.add_ellipse(
            self.center.convert(),
            self.radii.convert(),
            Angle::zero(),
            Winding::Positive,
        );
    }
}

#[allow(missing_docs)]
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
    fn add_geometry(&self, b: &mut Builder) {
        let points = self
            .points
            .iter()
            .map(|p| p.convert())
            .collect::<Vec<Point>>();
        let polygon: LyonPolygon<Point> = LyonPolygon {
            points: points.as_slice(),
            closed: self.closed,
        };

        b.add_polygon(polygon);
    }
}

/// The regular polygon feature used to determine the dimensions of the polygon.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegularPolygonFeature {
    /// The radius of the polygon's circumcircle.
    Radius(f32),
    /// The radius of the polygon's incircle.
    Apothem(f32),
    /// The length of the polygon's side.
    SideLength(f32),
}

#[allow(missing_docs)]
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
    fn add_geometry(&self, b: &mut Builder) {
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

        b.add_polygon(polygon);
    }
}
