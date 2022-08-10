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
    app::{App, Plugin},
    asset::Assets,
    ecs::{
        query::{Changed, Or},
        system::{Query, ResMut, Resource},
    },
    log::error,
    prelude::{CoreStage, Deref, DerefMut, ParallelSystemDescriptorCoercion as _, SystemLabel},
    render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    sprite::Mesh2dHandle,
};
use lyon_tessellation::{self as tess, BuffersBuilder};

use crate::{
    draw::{DrawMode, FillMode, StrokeMode},
    entity::Path,
    render::RenderShapePlugin,
    vertex::{VertexBuffers, VertexConstructor},
};

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = lyon_tessellation::FillTessellator::new();
        let stroke_tess = lyon_tessellation::StrokeTessellator::new();
        app.insert_resource(FillTessellator(fill_tess))
            .insert_resource(StrokeTessellator(stroke_tess))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                mesh_shapes_system
                    .label(BuildShapes)
                    .after(bevy::transform::transform_propagate_system),
            )
            .add_plugin(RenderShapePlugin);
    }
}

/// [`SystemLabel`] for the system that builds the meshes for newly-added
/// or changed shapes. Resides in [`PostUpdate`](CoreStage::PostUpdate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct BuildShapes;

/// Queries all the [`ShapeBundle`]s to mesh them when they are added
/// or re-mesh them when they are changed.
#[allow(clippy::type_complexity)]
fn mesh_shapes_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<(&DrawMode, &Path, &mut Mesh2dHandle), Or<(Changed<Path>, Changed<DrawMode>)>>,
) {
    for (tess_mode, path, mut mesh) in query.iter_mut() {
        let mut buffers = VertexBuffers::new();

        match tess_mode {
            DrawMode::Fill(mode) => {
                fill(&mut fill_tess, &path.0, mode, &mut buffers);
            }
            DrawMode::Stroke(mode) => {
                stroke(&mut stroke_tess, &path.0, mode, &mut buffers);
            }
            DrawMode::Outlined {
                fill_mode,
                outline_mode,
            } => {
                fill(&mut fill_tess, &path.0, fill_mode, &mut buffers);
                stroke(&mut stroke_tess, &path.0, outline_mode, &mut buffers);
            }
        }

        mesh.0 = meshes.add(build_mesh(&buffers));
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // lyon takes &FillOptions
fn fill(
    tess: &mut ResMut<FillTessellator>,
    path: &tess::path::Path,
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
    path: &tess::path::Path,
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
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(buffers.indices.clone())));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| [v.position[0], v.position[1], 0.0])
            .collect::<Vec<[f32; 3]>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        buffers
            .vertices
            .iter()
            .map(|v| v.color)
            .collect::<Vec<[f32; 4]>>(),
    );

    mesh
}

#[derive(Resource, Deref, DerefMut)]
struct FillTessellator(lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut)]
struct StrokeTessellator(lyon_tessellation::StrokeTessellator);
