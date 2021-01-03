//! Common shapes, like rectangles, ellipses, triangles and more.

use crate::{create_sprite, Geometry, TessellationMode};
use bevy::prelude::*;
use lyon_tessellation::{
    math::{Point, Rect, Size},
    path::{Polygon, Winding},
    BuffersBuilder, FillTessellator, FillVertex, StrokeTessellator, StrokeVertex, VertexBuffers,
};

/// Basic shapes descriptors, used in [`primitive`](primitive).
pub enum ShapeType {
    Rectangle { width: f32, height: f32 },
    Circle(f32),
    Ellipse { radius_x: f32, radius_y: f32 },
    Polygon { points: Vec<Point>, closed: bool },
}

/// Returns a `SpriteBundle` bundle using the given [`ShapeType`](ShapeType)
/// and [`TessellationMode`](crate::TessellationMode).
pub fn primitive(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    shape_type: ShapeType,
    tessellation_mode: TessellationMode,
    translation: Vec3,
) -> SpriteBundle {
    let mut geometry = Geometry(VertexBuffers::new());
    match tessellation_mode {
        TessellationMode::Fill(options) => {
            let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                [vertex.position().x, vertex.position().y, 0.0]
            });
            // TODO: Instantiate the tessellator only once.
            let mut tessellator = FillTessellator::new();
            match shape_type {
                ShapeType::Rectangle { width, height } => {
                    tessellator
                        .tessellate_rectangle(
                            &Rect::new(Point::zero(), Size::new(width, height)),
                            options,
                            output,
                        )
                        .unwrap();
                }
                ShapeType::Circle(radius) => {
                    tessellator
                        .tessellate_circle(Point::zero(), radius, options, output)
                        .unwrap();
                }
                ShapeType::Ellipse { radius_x, radius_y } => {
                    tessellator
                        .tessellate_ellipse(
                            Point::zero(),
                            (radius_x, radius_y).into(),
                            lyon_tessellation::math::Angle::zero(),
                            Winding::Positive,
                            options,
                            output,
                        )
                        .unwrap();
                }
                ShapeType::Polygon { points, closed } => {
                    let polygon = Polygon {
                        points: points.as_slice(),
                        closed,
                    };
                    tessellator
                        .tessellate_polygon(polygon, options, output)
                        .unwrap();
                }
            }
        }
        TessellationMode::Stroke(options) => {
            let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                [vertex.position().x, vertex.position().y, 0.0]
            });
            let mut tessellator = StrokeTessellator::new();
            match shape_type {
                ShapeType::Rectangle { width, height } => {
                    tessellator
                        .tessellate_rectangle(
                            &Rect::new(Point::zero(), Size::new(width, height)),
                            options,
                            output,
                        )
                        .unwrap();
                }
                ShapeType::Circle(radius) => {
                    tessellator
                        .tessellate_circle(Point::zero(), radius, options, output)
                        .unwrap();
                }
                ShapeType::Ellipse { radius_x, radius_y } => {
                    tessellator
                        .tessellate_ellipse(
                            Point::zero(),
                            (radius_x, radius_y).into(),
                            lyon_tessellation::math::Angle::zero(),
                            Winding::Positive,
                            options,
                            output,
                        )
                        .unwrap();
                }
                ShapeType::Polygon { points, closed } => {
                    let polygon = Polygon {
                        points: points.as_slice(),
                        closed,
                    };
                    tessellator
                        .tessellate_polygon(polygon, options, output)
                        .unwrap();
                }
            }
        }
    }

    create_sprite(material, meshes, geometry, translation)
}
