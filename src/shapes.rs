//! Collection of common shapes that can be drawn.
//!
//! The structs defined in this module implement the
//! [`Geometry`](crate::geometry::Geometry) trait. You can also implement
//! the trait for your own shapes.

use crate::{geometry::Geometry, utils::Convert};
use bevy::math::Vec2;
use lyon_tessellation::{
    math::{point, Angle, Point, Rect, Size, Vector},
    path::{
        builder::WithSvg,
        path::Builder,
        traits::{PathBuilder, SvgPathBuilder},
        ArcFlags, Polygon as LyonPolygon, Winding,
    },
};
use svgtypes::{Path, PathSegment};

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

impl Geometry for Rectangle {
    fn add_geometry(&self, b: &mut Builder) {
        let origin = match self.origin {
            RectangleOrigin::Center => Point::new(-self.width / 2.0, -self.height / 2.0),
            RectangleOrigin::BottomLeft => Point::new(0.0, 0.0),
            RectangleOrigin::BottomRight => Point::new(-self.width, 0.0),
            RectangleOrigin::TopRight => Point::new(-self.width, -self.height),
            RectangleOrigin::TopLeft => Point::new(0.0, -self.height),
            RectangleOrigin::CustomCenter(v) => {
                Point::new(v.x - self.width / 2.0, v.y - self.height / 2.0)
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

impl Geometry for Circle {
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

impl Geometry for Ellipse {
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

impl Geometry for Polygon {
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
            center: Vec2::zero(),
            feature: RegularPolygonFeature::Radius(1.0),
        }
    }
}

impl Geometry for RegularPolygon {
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

/// A simple line segment, specified by two points.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line(pub Vec2, pub Vec2);

impl Geometry for Line {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_polygon(LyonPolygon {
            points: &[self.0.convert(), self.1.convert()],
            closed: false,
        });
    }
}
///An easy way to display svg paths as a shape, takes an svg path string and a
///document size(Vec2).
///
///For documentation on svg paths: https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths
///
///Make sure that your units are pixels(px) and that the transform of the \<g\>
///in your svg document is set to transform="translate(0,0)" so as to not
///offset the coordinates of the paths
///
///In inkscape for example, to turn your units into pixels, you:
///1) Go to File>Document Properties>General>Display Units and set it to px
///
///2) In File>Document Properties>Custom Size>Units set it to px, also, this
/// size would be used for svg_doc_size_in_px
///
///3) In File>Document Properties>Scale>Scale x make sure it is set to 1 User
/// unit per px
///
///Example exists in the examples folder
pub struct SvgPathShape {
    ///The document size of the svg art, make sure the units are in pixels
    pub svg_doc_size_in_px: Vec2,
    ///The string that describes the path, make sure the units are in pixels
    ///and that the transform of the \<g\> in your svg document is set to
    ///transform="translate(0,0)" so as to not offset the coordinates of the
    ///paths
    pub svg_path_string: String,
}
impl Geometry for SvgPathShape {
    fn add_geometry(&self, b: &mut Builder) {
        let builder = Builder::new();
        let mut svg_builder = WithSvg::new(builder);
        let p: Path = self.svg_path_string.parse().unwrap();
        let offset_x = self.svg_doc_size_in_px.x / 2.;
        let offset_y = self.svg_doc_size_in_px.y / 2.;
        let mut used_move_command = false;

        for path_segment in p.0 {
            match path_segment {
                PathSegment::MoveTo { abs, x, y } => {
                    if abs || !used_move_command {
                        svg_builder
                            .move_to(Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y));
                        used_move_command = true;
                    } else {
                        svg_builder.relative_move_to(Vector::new(x as f32, y as f32 * -1.));
                    }
                }
                PathSegment::LineTo { abs, x, y } => {
                    if abs {
                        svg_builder
                            .line_to(Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y));
                    } else {
                        svg_builder.relative_line_to(Vector::new(x as f32, y as f32 * -1.));
                    }
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    if abs {
                        svg_builder.horizontal_line_to(x as f32 - offset_x);
                    } else {
                        svg_builder.relative_horizontal_line_to(x as f32);
                    }
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    if abs {
                        svg_builder.vertical_line_to(y as f32 * -1. + offset_y);
                    } else {
                        svg_builder.relative_vertical_line_to(y as f32 * -1.);
                    }
                }
                PathSegment::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    if abs {
                        svg_builder.cubic_bezier_to(
                            Point::new(x1 as f32 - offset_x, y1 as f32 * -1. + offset_y),
                            Point::new(x2 as f32 - offset_x, y2 as f32 * -1. + offset_y),
                            Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y),
                        );
                    } else {
                        svg_builder.relative_cubic_bezier_to(
                            Vector::new(x1 as f32, y1 as f32 * -1.),
                            Vector::new(x2 as f32, y2 as f32 * -1.),
                            Vector::new(x as f32, y as f32 * -1.),
                        );
                    }
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    if abs {
                        svg_builder.smooth_cubic_bezier_to(
                            Point::new(x2 as f32 - offset_x, y2 as f32 * -1. + offset_y),
                            Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y),
                        );
                    } else {
                        svg_builder.smooth_relative_cubic_bezier_to(
                            Vector::new(x2 as f32, y2 as f32 * -1.),
                            Vector::new(x as f32, y as f32 * -1.),
                        );
                    }
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    if abs {
                        svg_builder.quadratic_bezier_to(
                            Point::new(x1 as f32 - offset_x, y1 as f32 * -1. + offset_y),
                            Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y),
                        );
                    } else {
                        svg_builder.relative_quadratic_bezier_to(
                            Vector::new(x1 as f32, y1 as f32 * -1.),
                            Vector::new(x as f32, y as f32 * -1.),
                        );
                    }
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    if abs {
                        svg_builder.smooth_quadratic_bezier_to(Point::new(
                            x as f32 - offset_x,
                            y as f32 * -1. + offset_y,
                        ));
                    } else {
                        svg_builder.smooth_relative_quadratic_bezier_to(Vector::new(
                            x as f32,
                            y as f32 * -1.,
                        ));
                    }
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x,
                    y,
                } => {
                    if abs {
                        svg_builder.arc_to(
                            Vector::new(rx as f32, ry as f32),
                            Angle {
                                radians: x_axis_rotation as f32,
                            },
                            ArcFlags { sweep, large_arc },
                            Point::new(x as f32 - offset_x, y as f32 * -1. + offset_y),
                        );
                    } else {
                        svg_builder.relative_arc_to(
                            Vector::new(rx as f32, ry as f32),
                            Angle {
                                radians: x_axis_rotation as f32,
                            },
                            ArcFlags { sweep, large_arc },
                            Vector::new(x as f32, y as f32 * -1.),
                        );
                    }
                }
                PathSegment::ClosePath { abs: _ } => {
                    svg_builder.close();
                }
            }
        }
        let path = svg_builder.build();
        b.concatenate(&[path.as_slice()]);
    }
}
