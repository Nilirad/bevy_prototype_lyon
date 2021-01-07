//! Draw 2D shapes in Bevy.
//!
//! This crate is meant to be a wrapper around the `lyon` crate to make it
//! integrate with Bevy. It's far from perfect, but it's a first attempt to draw
//! 2D shapes in Bevy.

use bevy::{prelude::*, render::mesh::Indices};
use lyon_tessellation::{
    FillOptions, FillTessellator, StrokeOptions, StrokeTessellator, VertexBuffers,
};

pub mod basic_shapes;
pub mod conversions;
pub mod path;
pub mod shape_plugin;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{
        basic_shapes::{CircleShape, EllipseShape, PolygonShape, RectangleOrigin, RectangleShape},
        path::{Path, PathBuilder},
        shape_plugin::ShapePlugin,
        ShapeSprite, TessellationMode, Tessellator,
    };
    pub use lyon_tessellation::{math::point, FillOptions, LineCap, LineJoin, StrokeOptions};
}

/// A memory buffer that lyon will fill with vertex and index data. It is not
/// ready for Bevy consumption, so it must be converted first.
struct Geometry(pub VertexBuffers<[f32; 3], u32>);

/// Converts a lyon buffer into a bevy mesh.
impl From<Geometry> for Mesh {
    fn from(geometry: Geometry) -> Self {
        let num_vertices = geometry.0.vertices.len();
        let mut mesh = Self::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(geometry.0.indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, geometry.0.vertices);

        let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; num_vertices];
        let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0]; num_vertices];

        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh
    }
}

/// Returns a `SpriteBundle` bundle with the given [`Geometry`](Geometry)
/// and `ColorMaterial`.
fn create_sprite(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    geometry: Geometry,
    translation: Vec3,
) -> SpriteBundle {
    SpriteBundle {
        material,
        mesh: meshes.add(geometry.into()),
        sprite: Sprite {
            size: Vec2::new(1.0, 1.0),
            ..Default::default()
        },
        transform: Transform::from_translation(translation),
        ..Default::default()
    }
}

/// Determines if a shape or path must be filled or stroked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TessellationMode {
    Fill(FillOptions),
    Stroke(StrokeOptions),
}

/// A couple of `lyon` fill and stroke tessellators.
pub struct Tessellator {
    pub fill: Option<FillTessellator>,
    pub stroke: Option<StrokeTessellator>,
}

impl Tessellator {
    /// Returns two new tessellators, one for fill and one for stroke.
    pub fn new() -> Self {
        Self {
            fill: Some(FillTessellator::new()),
            stroke: Some(StrokeTessellator::new()),
        }
    }

    /// Returns a new fill tessellator.
    pub fn only_fill() -> Self {
        Self {
            fill: Some(FillTessellator::new()),
            stroke: None,
        }
    }

    /// Returns a new stroke tessellator.
    pub fn only_stroke() -> Self {
        Self {
            fill: None,
            stroke: Some(StrokeTessellator::new()),
        }
    }
}

/// Shape structs that implement this trait can be transformed into a
/// `SpriteBundle`. See [`basic_shapes`] module for examples.
pub trait ShapeSprite {
    /// Returns a `SpriteBundle` with a custom mesh.
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle;
}
