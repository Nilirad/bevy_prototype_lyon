//! Draw 2D shapes in Bevy.
//!
//! This crate is meant to be a wrapper around the `lyon` crate to make it
//! integrate with Bevy. It's far from perfect, but it's a first attempt to draw
//! 2D shapes in Bevy.

use bevy::{
    prelude::*,
    render::mesh::Indices,
};
use lyon::tessellation::VertexBuffers;

pub mod basic_shapes;
pub mod path;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{
        basic_shapes::{primitive, ShapeType},
        path::{Path, PathBuilder},
        TessellationMode,
    };
    pub use lyon::{
        math::point,
        tessellation::{FillOptions, LineCap, LineJoin, StrokeOptions},
    };
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

        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        for _ in 0..num_vertices {
            normals.push([0.0, 0.0, 0.0]);
            uvs.push([0.0, 0.0]);
        }

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
pub enum TessellationMode<'options> {
    Fill(&'options lyon::tessellation::FillOptions),
    Stroke(&'options lyon::tessellation::StrokeOptions),
}
