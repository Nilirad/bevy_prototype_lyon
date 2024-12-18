use bevy::math::Vec2;
use lyon_tessellation::{
    math::{point, Point},
    path::{path::Builder, Polygon as LyonPolygon},
};

use crate::{geometry::Geometry, utils::ToPoint};

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

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub struct RoundedPolygon {
    pub points: Vec<Vec2>,
    pub radius: f32,
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
