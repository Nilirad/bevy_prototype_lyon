//! Contains the plugin and its helper types.
//!
//! The [`ShapePlugin`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`ShapeBundle`](crate::entity::ShapeBundle) from a
//! system in the `UPDATE` stage.
//!
//! Then, in [`Stage::Shape`] stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `ShapeBundle`.

use bevy::{
    app::{AppBuilder, Plugin},
    asset::{Assets, Handle},
    ecs::{
        query::Added,
        schedule::{StageLabel, SystemStage},
        system::{IntoSystem, Query, ResMut},
    },
    log::error,
    render::{
        color::Color,
        draw::Visible,
        mesh::{Indices, Mesh},
        pipeline::PrimitiveTopology,
    },
};
use lyon_tessellation::{
    self as tess, path::Path, BuffersBuilder, FillTessellator, FillVertex, FillVertexConstructor,
    StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
};
use tess::{FillOptions, StrokeOptions};

use crate::{entity::ShapeColors, utils::DrawMode};

/// Stages for this plugin.
#[derive(Debug, Clone, Eq, Hash, PartialEq, StageLabel)]
pub enum Stage {
    /// The stage where the [`ShapeBundle`](crate::entity::ShapeBundle) gets
    /// completed.
    Shape,
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
type VertexBuffers = tess::VertexBuffers<Vertex, IndexType>;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
struct VertexConstructor {
    color: Color,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: [
                self.color.r(),
                self.color.g(),
                self.color.b(),
                self.color.a(),
            ],
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: [
                self.color.r(),
                self.color.g(),
                self.color.b(),
                self.color.a(),
            ],
        }
    }
}

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_stage_after(
                bevy::app::CoreStage::Update,
                Stage::Shape,
                SystemStage::parallel(),
            )
            .add_system_to_stage(Stage::Shape, complete_shape_bundle.system());
        
        crate::render::add_shape_pipeline(app.world_mut());
    }
}

/// A bevy system. Queries all the [`ShapeBundle`]s to complete them with a
/// mesh.
#[allow(clippy::type_complexity)]
fn complete_shape_bundle(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<
        (
            &DrawMode,
            &Path,
            &mut Handle<Mesh>,
            &ShapeColors,
            &mut Visible,
        ),
        Added<Path>,
    >,
) {
    for (tess_mode, path, mut mesh, colors, mut visible) in query.iter_mut() {
        let mut buffers = VertexBuffers::new();

        match tess_mode {
            DrawMode::Fill(options) => {
                fill(&mut fill_tess, path, options, &mut buffers, colors.main);
            }
            DrawMode::Stroke(options) => {
                stroke(&mut stroke_tess, path, options, &mut buffers, colors.main);
            }
            DrawMode::Outlined {
                fill_options,
                outline_options,
            } => {
                fill(
                    &mut fill_tess,
                    path,
                    fill_options,
                    &mut buffers,
                    colors.main,
                );
                stroke(
                    &mut stroke_tess,
                    path,
                    outline_options,
                    &mut buffers,
                    colors.outline,
                );
            }
        }

        *mesh = meshes.add(build_mesh(&buffers));
        visible.is_visible = true;
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &FillOptions
fn fill(
    tess: &mut ResMut<FillTessellator>,
    path: &Path,
    options: &FillOptions,
    buffers: &mut VertexBuffers,
    vertex_color: Color,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        options,
        &mut BuffersBuilder::new(
            buffers,
            VertexConstructor {
                color: vertex_color,
            },
        ),
    ) {
        error!("FillTessellator error: {:?}", e);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &StrokeOptions
fn stroke(
    tess: &mut ResMut<StrokeTessellator>,
    path: &Path,
    options: &StrokeOptions,
    buffers: &mut VertexBuffers,
    vertex_color: Color,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        options,
        &mut BuffersBuilder::new(
            buffers,
            VertexConstructor {
                color: vertex_color,
            },
        ),
    ) {
        error!("StrokeTessellator error: {:?}", e);
    }
}

fn build_mesh(buffers: &VertexBuffers) -> Mesh {
    pub const ATTRIBUTE_POSITION_2D: &str = "Vertex_Position_2D";

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.set_attribute(
        ATTRIBUTE_POSITION_2D,
        buffers
            .vertices
            .iter()
            .map(|v| v.position)
            .collect::<Vec<[f32; 2]>>(),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_COLOR,
        buffers
            .vertices
            .iter()
            .map(|v| v.color)
            .collect::<Vec<[f32; 4]>>(),
    );

    mesh
}
