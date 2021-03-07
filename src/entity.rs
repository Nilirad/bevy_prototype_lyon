//! Custom Bevy [`Bundle`] for shapes.

use crate::{render::SHAPE_PIPELINE_HANDLE, utils::TessellationMode};
use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    math::Vec2,
    reflect::TypeUuid,
    render::{
        color::Color,
        draw::{Draw, Visible},
        mesh::Mesh,
        pipeline::{RenderPipeline, RenderPipelines},
        render_graph::base::MainPass,
        renderer::RenderResources,
    },
    sprite::{ColorMaterial, Sprite, QUAD_HANDLE},
    transform::components::{GlobalTransform, Transform},
};
use lyon_tessellation::{path::Path, FillOptions};

/// A Bevy [`Bundle`] to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: TessellationMode,
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path::new(),
            mode: TessellationMode::Fill(FillOptions::default()),
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SHAPE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            main_pass: MainPass,
            draw: Draw::default(),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Sprite::default()
            },
            material: Handle::<ColorMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "370d078f-e13b-48d8-b12a-954ba887afcb"]
struct ShapeMaterial {
    pub color: Color,
}
