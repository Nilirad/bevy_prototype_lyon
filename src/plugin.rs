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

use crate::vertex::{VertexBuffers, VertexConstructor};
use bevy::{
    app::{App, Plugin},
    asset::{Assets, Handle},
    ecs::{
        query::{Changed, Or},
        schedule::{StageLabel, SystemStage},
        system::{Query, ResMut},
    },
    log::error,
    render::{
        mesh::{Indices, Mesh},
        pipeline::PrimitiveTopology,
    },
};
use lyon_tessellation::{path::Path, BuffersBuilder, FillTessellator, StrokeTessellator};

use crate::draw::{DrawMode, FillMode, StrokeMode};

/// Stages for this plugin.
#[derive(Debug, Clone, Eq, Hash, PartialEq, StageLabel)]
pub enum Stage {
    /// The stage where the [`ShapeBundle`](crate::entity::ShapeBundle) gets
    /// completed.
    Shape,
}

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.insert_resource(fill_tess)
            .insert_resource(stroke_tess)
            .add_stage_after(
                bevy::app::CoreStage::Update,
                Stage::Shape,
                SystemStage::parallel(),
            )
            .add_system_to_stage(Stage::Shape, mesh_shapes_system);

        crate::render::add_shape_pipeline(&mut app.world);
        crate::render::add_ui_shape_pipeline(&mut app.world);
    }
}

/// Queries all the [`ShapeBundle`]s to mesh them when they are added
/// or re-mesh them when they are changed.
#[allow(clippy::type_complexity)]
fn mesh_shapes_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<(&DrawMode, &Path, &mut Handle<Mesh>), Or<(Changed<Path>, Changed<DrawMode>)>>,
) {
    for (tess_mode, path, mut mesh) in query.iter_mut() {
        let mut buffers = VertexBuffers::new();

        match tess_mode {
            DrawMode::Fill(mode) => {
                fill(&mut fill_tess, path, mode, &mut buffers);
            }
            DrawMode::Stroke(mode) => {
                stroke(&mut stroke_tess, path, mode, &mut buffers);
            }
            DrawMode::Outlined {
                fill_mode,
                outline_mode,
            } => {
                fill(&mut fill_tess, path, fill_mode, &mut buffers);
                stroke(&mut stroke_tess, path, outline_mode, &mut buffers);
            }
        }

        *mesh = meshes.add(build_mesh(&buffers));
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &FillOptions
fn fill(
    tess: &mut ResMut<FillTessellator>,
    path: &Path,
    mode: &FillMode,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { color: mode.color }),
    ) {
        error!("FillTessellator error: {:?}", e);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &StrokeOptions
fn stroke(
    tess: &mut ResMut<StrokeTessellator>,
    path: &Path,
    mode: &StrokeMode,
    buffers: &mut VertexBuffers,
) {
    if let Err(e) = tess.tessellate_path(
        path,
        &mode.options,
        &mut BuffersBuilder::new(buffers, VertexConstructor { color: mode.color }),
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
