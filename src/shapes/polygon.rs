//! Tools for drawing polygons.

use bevy::math::Vec2;
use lyon_tessellation::{
    math::{point, Point},
    path::{path::Builder, Polygon as LyonPolygon},
};

use crate::{geometry::Geometry, utils::ToPoint};

/// An arbitrary polygon or polyline.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    /// The vertices of a polygon.
    ///
    /// Each vertex is connected to the next.
    /// The last vertex is connected to the first,
    /// if [`closed`] is `true`.
    ///
    /// [`closed`]: Self::closed
    pub points: Vec<Vec2>,
    /// Whether the last vertex is connected to the first.
    ///
    /// If `true`, the shape is a *polygon*.
    /// If `false`, the shape is a *polygonal line* (also called *polyline*).
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

impl Geometry<Builder> for Polygon {
    fn add_geometry(&self, b: &mut Builder) {
        let points = self
            .points
            .iter()
            .map(|p| p.to_point())
            .collect::<Vec<Point>>();
        let polygon: LyonPolygon<Point> = LyonPolygon {
            points: points.as_slice(),
            closed: self.closed,
        };

        b.add_polygon(polygon);
    }
}

/// An arbitrary polygon, or polyline, with rounded vertices.
#[derive(Debug, Clone, PartialEq)]
pub struct RoundedPolygon {
    /// The vertices of a polygon.
    ///
    /// Each vertex is connected to the next.
    /// The last vertex is connected to the first,
    /// if `closed` is true.
    pub points: Vec<Vec2>,
    /// The radius of the arc used to round the corners of the vertices.
    pub radius: f32,
    /// Whether the last vertex is connected to the first.
    ///
    /// If `true`, the shape is a *polygon*.
    /// If `false`, the shape is a *polygonal line* (also called *polyline*).
    pub closed: bool,
}

impl Default for RoundedPolygon {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            radius: 0.0,
            closed: true,
        }
    }
}

impl Geometry<Builder> for RoundedPolygon {
    fn add_geometry(&self, b: &mut Builder) {
        let points = self
            .points
            .iter()
            .map(|p| p.to_point())
            .collect::<Vec<Point>>();
        let polygon: LyonPolygon<Point> = LyonPolygon {
            points: points.as_slice(),
            closed: self.closed,
        };
        lyon_algorithms::rounded_polygon::add_rounded_polygon(
            b,
            polygon,
            self.radius,
            lyon_algorithms::path::NO_ATTRIBUTES,
        );
    }
}

/// The geometric property used to define a [`RegularPolygon`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegularPolygonFeature {
    /// The radius of the polygon's circumcircle.
    Radius(f32),
    /// The radius of the polygon's incircle.
    Apothem(f32),
    /// The length of the polygon's side.
    SideLength(f32),
}

/// A polygon with all sides and angles equal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegularPolygon {
    /// The number of sides.
    ///
    /// # Panics
    ///
    /// Attempting to draw a `RegularPolygon` with a value less than `3`
    /// will result in a panic.
    pub sides: usize,
    /// The point equidistant from all its vertices.
    pub center: Vec2,
    /// The geometric property used to define the polygon.
    pub feature: RegularPolygonFeature,
}

impl RegularPolygon {
    /// Gets the radius of the polygon.
    fn radius(&self) -> f32 {
        let ratio = std::f32::consts::PI / self.sides as f32;

        match self.feature {
            RegularPolygonFeature::Radius(r) => r,
            RegularPolygonFeature::Apothem(a) => a * ratio.tan() / ratio.sin(),
            RegularPolygonFeature::SideLength(s) => s / (2.0 * ratio.sin()),
        }
    }
}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self {
            sides: 3,
            center: Vec2::ZERO,
            feature: RegularPolygonFeature::Radius(1.0),
        }
    }
}

impl Geometry<Builder> for RegularPolygon {
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
            let cur_angle = (i as f32).mul_add(step, offset);
            let x = radius.mul_add(cur_angle.cos(), self.center.x);
            let y = radius.mul_add(cur_angle.sin(), self.center.y);
            points.push(point(x, y));
        }

        let polygon = LyonPolygon {
            points: points.as_slice(),
            closed: true,
        };

        b.add_polygon(polygon);
    }
}
