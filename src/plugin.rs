//! Contains the plugin and its helper types.
//!
//! The [`ShapePlugin`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`ShapeBundle`](crate::entity::ShapeBundle) from a
//! system in the [`UPDATE`](bevy::app::stage::UPDATE) stage.
//!
//! Then, in the [`SHAPE`](stage::SHAPE) stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `ShapeBundle`.

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{Assets, Handle},
    ecs::{
        schedule::SystemStage,
        system::{IntoSystem, Query, ResMut},
    },
    log::error,
    prelude::{AddAsset, Added},
    render::{
        draw::Visible,
        mesh::{Indices, Mesh},
        pipeline::PrimitiveTopology,
    },
};
use lyon_tessellation::{
    self as tess, path::Path, BuffersBuilder, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
};

use crate::{entity::ShapeMaterial, utils::DrawMode};

/// Stages for this plugin.
pub mod stage {
    /// The stage where the [`ShapeBundle`](crate::entity::ShapeBundle) gets
    /// completed.
    pub const SHAPE: &str = "shape";
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
type VertexBuffers = tess::VertexBuffers<Vertex, IndexType>;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex([f32; 3]);

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
struct VertexConstructor;

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex([vertex.position().x, vertex.position().y, 0.0])
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex([vertex.position().x, vertex.position().y, 0.0])
    }
}

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.add_asset::<ShapeMaterial>()
            .insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_stage_after(
                bevy::app::CoreStage::Update,
                stage::SHAPE,
                SystemStage::parallel(),
            )
            .add_startup_system(crate::render::add_shape_pipeline.system())
            .add_system_to_stage(stage::SHAPE, complete_shape_bundle.system());
    }
}

/// A bevy system. Queries all the [`ShapeBundle`]s to complete them with a
/// mesh.
fn complete_shape_bundle(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<(&DrawMode, &Path, &mut Handle<Mesh>, &mut Visible), Added<Path>>,
) {
    for (tess_mode, path, mut mesh, mut visible) in query.iter_mut() {
        let mut buffers = VertexBuffers::new();

        match tess_mode {
            DrawMode::Fill(ref options) => {
                if let Err(e) = fill_tess.tessellate_path(
                    path,
                    options,
                    &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                ) {
                    error!("FillTessellator error: {:?}", e);
                }
            }
            DrawMode::Stroke(ref options) => {
                if let Err(e) = stroke_tess.tessellate_path(
                    path,
                    options,
                    &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                ) {
                    error!("StrokeTessellator error: {:?}", e);
                }
            }
        }

        *mesh = meshes.add(build_mesh(&buffers));
        visible.is_visible = true;
    }
}

fn build_mesh(buffers: &VertexBuffers) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| v.0)
            .collect::<Vec<[f32; 3]>>(),
    );

    mesh
}
