//! Common shapes, like rectangles, ellipses, triangles and more.

use crate::{create_sprite, Geometry, TessellationMode};
use bevy::prelude::*;
use lyon::{
    math::Size,
    tessellation::{
        basic_shapes::{self, BorderRadii},
        math::{Point, Rect},
        BuffersBuilder, FillAttributes, StrokeAttributes, VertexBuffers,
    },
};

/// Basic shapes descriptors, used in [`primitive`](primitive).
pub enum ShapeType {
    Triangle(Point, Point, Point),
    Quad(Point, Point, Point, Point),
    Rectangle {
        width: f32,
        height: f32,
    },
    RoundedRectangle {
        width: f32,
        height: f32,
        border_radius: f32,
    },
    Circle(f32),
    Ellipse {
        radius_x: f32,
        radius_y: f32,
    },
    Polyline {
        points: Vec<Point>,
        closed: bool,
    },
    //ConvexPolyline(Vec<Point>), // TODO: Too much of an hassle to implement.
}

/// Returns a `SpriteComponents` bundle using the given [`ShapeType`](ShapeType)
/// and [`TessellationMode`](crate::TessellationMode).
pub fn primitive(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    shape_type: ShapeType,
    tessellation_mode: TessellationMode,
    translation: Vec3,
) -> SpriteComponents {
    let mut geometry = Geometry(VertexBuffers::new());
    match tessellation_mode {
        TessellationMode::Fill(options) => {
            let ref mut output =
                BuffersBuilder::new(&mut geometry.0, |pos: Point| [pos.x, pos.y, 0.0]);
            match shape_type {
                ShapeType::Triangle(..) => {
                    panic!("Fill triangle is not available.");
                }
                ShapeType::Quad(v1, v2, v3, v4) => {
                    basic_shapes::fill_quad(v1, v2, v3, v4, options, output).unwrap();
                }
                ShapeType::Rectangle { width, height } => {
                    let ref rect = Rect::new(Point::zero(), Size::new(width, height));
                    basic_shapes::fill_rectangle(rect, options, output).unwrap();
                }
                ShapeType::RoundedRectangle {
                    width,
                    height,
                    border_radius,
                } => {
                    let ref rect = Rect::new(Point::zero(), Size::new(width, height));
                    let ref radii = BorderRadii::new_all_same(border_radius);
                    basic_shapes::fill_rounded_rectangle(rect, radii, options, output).unwrap();
                }
                ShapeType::Circle(radius) => {
                    basic_shapes::fill_circle(Point::zero(), radius, options, output).unwrap();
                }
                ShapeType::Ellipse { .. } => {
                    panic!("Fill ellipse is not available.");
                }
                ShapeType::Polyline { points, .. } => {
                    let ref mut tessellator = lyon::tessellation::FillTessellator::new();
                    basic_shapes::fill_polyline(
                        points,
                        tessellator,
                        options,
                        &mut BuffersBuilder::new(
                            &mut geometry.0,
                            |pos: Point, _: FillAttributes| [pos.x, pos.y, 0.0],
                        ),
                    )
                    .unwrap();
                }
            }
        }
        TessellationMode::Stroke(options) => {
            let ref mut output =
                BuffersBuilder::new(&mut geometry.0, |pos: Point, _: StrokeAttributes| {
                    [pos.x, pos.y, 0.0]
                });
            match shape_type {
                ShapeType::Triangle(v1, v2, v3) => {
                    basic_shapes::stroke_triangle(v1, v2, v3, options, output).unwrap();
                }
                ShapeType::Quad(v1, v2, v3, v4) => {
                    basic_shapes::stroke_quad(v1, v2, v3, v4, options, output).unwrap();
                }
                ShapeType::Rectangle { width, height } => {
                    let ref rect = Rect::new(Point::zero(), Size::new(width, height));
                    basic_shapes::stroke_rectangle(rect, options, output).unwrap();
                }
                ShapeType::RoundedRectangle {
                    width,
                    height,
                    border_radius,
                } => {
                    let ref rect = Rect::new(Point::zero(), Size::new(width, height));
                    let ref radii = BorderRadii::new_all_same(border_radius);
                    basic_shapes::stroke_rounded_rectangle(rect, radii, options, output).unwrap();
                }
                ShapeType::Circle(radius) => {
                    basic_shapes::stroke_circle(Point::zero(), radius, options, output).unwrap();
                }
                ShapeType::Ellipse { radius_x, radius_y } => {
                    basic_shapes::stroke_ellipse(
                        Point::zero(),
                        (radius_x, radius_y).into(),
                        lyon::math::Angle::zero(),
                        options,
                        output,
                    )
                    .unwrap();
                }
                ShapeType::Polyline { points, closed } => {
                    basic_shapes::stroke_polyline(points, closed, options, output).unwrap();
                }
            }
        }
    }

    create_sprite(material, meshes, geometry, translation)
}
