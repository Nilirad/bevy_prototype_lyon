//! Draw 2D shapes in Bevy.
//!
//! This crate is meant to be a wrapper around the `lyon` crate to make it
//! integrate with Bevy. It's far from perfect, but it's a first attempt to draw
//! 2D shapes in Bevy.

use bevy::{
    asset::{Assets, Handle},
    ecs::ResMut,
    math::Vec2,
    render::{
        mesh::{Indices, Mesh},
        pipeline::PrimitiveTopology,
    },
    sprite::{entity::SpriteBundle, ColorMaterial, Sprite},
    transform::components::Transform,
};
use lyon_tessellation::{
    path::Path, BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};
use shape_plugin::ShapeDescriptor;

pub mod basic_shapes;
pub mod conversions;
pub mod shape_plugin;

/// Import this module as `use bevy_prototype_lyon::prelude::*` to get
/// convenient imports.
pub mod prelude {
    pub use crate::{
        basic_shapes::{CircleShape, EllipseShape, PolygonShape, RectangleOrigin, RectangleShape},
        shape_plugin::ShapePlugin,
        ShapeSprite, TessellationMode, Tessellator,
    };
    pub use lyon_tessellation::{math::point, FillOptions, LineCap, LineJoin, StrokeOptions};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

struct VertexConstructor;

impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        }
    }
}

impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0, 0.0],
        }
    }
}

pub type IndexType = u32;
pub type Buffers = VertexBuffers<Vertex, IndexType>;

fn build_mesh(buffers: &Buffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| v.position)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        buffers
            .vertices
            .iter()
            .map(|v| v.normal)
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        buffers
            .vertices
            .iter()
            .map(|v| v.uv)
            .collect::<Vec<[f32; 2]>>(),
    );

    mesh
}

fn create_sprite(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    buffers: Buffers,
    transform: Transform,
) -> SpriteBundle {
    SpriteBundle {
        material,
        mesh: meshes.add(build_mesh(&buffers)),
        sprite: Sprite {
            size: Vec2::new(1.0, 1.0),
            ..Default::default()
        },
        transform,
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
    pub fill: FillTessellator,
    pub stroke: StrokeTessellator,
}

impl Tessellator {
    /// Returns two new tessellators, one for fill and one for stroke.
    pub fn new() -> Self {
        Self {
            fill: FillTessellator::new(),
            stroke: StrokeTessellator::new(),
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

    fn draw(
        &self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> (ShapeDescriptor,)
    where
        Self: Sync + Send + Sized + Copy + 'static,
    {
        let desc = ShapeDescriptor {
            shape: Box::new(self.clone()),
            material: material.clone(),
            mode,
            transform,
        };

        (desc,)
    }

    fn fill(
        &self,
        path: &Path,
        buffers: &mut Buffers,
        options: &FillOptions,
        tessellator: &mut FillTessellator,
    ) {
        tessellator
            .tessellate_path(
                path,
                options,
                &mut BuffersBuilder::new(buffers, VertexConstructor),
            )
            .unwrap();
    }

    fn stroke(
        &self,
        path: &Path,
        buffers: &mut Buffers,
        options: &StrokeOptions,
        tessellator: &mut StrokeTessellator,
    ) {
        tessellator
            .tessellate_path(
                path,
                options,
                &mut BuffersBuilder::new(buffers, VertexConstructor),
            )
            .unwrap();
    }

    fn tessellate(
        &self,
        path: &Path,
        buffers: &mut Buffers,
        mode: TessellationMode,
        tessellator: &mut Tessellator,
    ) {
        match mode {
            TessellationMode::Fill(ref options) => {
                self.fill(path, buffers, options, &mut tessellator.fill);
            }
            TessellationMode::Stroke(ref options) => {
                self.stroke(path, buffers, options, &mut tessellator.stroke);
            }
        }
    }
}
