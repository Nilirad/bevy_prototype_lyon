//! Common shapes, like rectangles, ellipses, triangles and more.

use crate::{
    conversions::{ToLyonPoint, ToLyonVector},
    create_sprite, Geometry, ShapeSprite, TessellationMode, Tessellator,
};
use bevy::prelude::*;
use lyon_tessellation::{
    math::{Angle, Point, Rect, Size},
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
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: &TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        use RectangleOrigin::*;
        let origin = match self.origin {
            Center => Point::new(-self.width / 2.0, -self.height / 2.0),
            BottomLeft => Point::new(0.0, 0.0),
            BottomRight => Point::new(-self.width, 0.0),
            TopRight => Point::new(-self.width, -self.height),
            TopLeft => Point::new(0.0, -self.height),
        };

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_rectangle(
                        &Rect::new(origin, Size::new(self.width, self.height)),
                        options,
                        output,
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_rectangle(
                        &Rect::new(origin, Size::new(self.width, self.height)),
                        options,
                        output,
                    )
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    /// Distance of the border of the circle from the center.
    pub radius: f32,
    /// The position of the center of the circle, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
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
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: &TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_circle(self.center.to_lyon_point(), self.radius, options, output)
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_circle(self.center.to_lyon_point(), self.radius, options, output)
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub radii: Vec2,
    /// The position of the center of the ellipse, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
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
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: &TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut geometry = Geometry(VertexBuffers::new());

        match mode {
            TessellationMode::Fill(options) => {
                let ref mut output = BuffersBuilder::new(&mut geometry.0, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y, 0.0]
                });
                tessellator
                    .fill
                    .as_mut()
                    .unwrap()
                    .tessellate_ellipse(
                        self.center.to_lyon_point(),
                        self.radii.to_lyon_vector(),
                        Angle::zero(),
                        Winding::Positive,
                        options,
                        output,
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(options) => {
                let ref mut output =
                    BuffersBuilder::new(&mut geometry.0, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y, 0.0]
                    });
                tessellator
                    .stroke
                    .as_mut()
                    .unwrap()
                    .tessellate_ellipse(
                        self.center.to_lyon_point(),
                        self.radii.to_lyon_vector(),
                        Angle::zero(),
                        Winding::Positive,
                        options,
                        output,
                    )
                    .unwrap();
            }
        }

        create_sprite(material, meshes, geometry, transform.translation)
    }
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
