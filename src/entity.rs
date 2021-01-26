//! Custom Bevy [`Bundle`] for shapes.

use crate::utils::TessellationMode;
use bevy::{
    asset::Handle,
    ecs::Bundle,
    math::Vec2,
    render::{
        draw::{Draw, Visible},
        mesh::Mesh,
        pipeline::{RenderPipeline, RenderPipelines},
        render_graph::base::MainPass,
    },
    sprite::{ColorMaterial, Sprite, QUAD_HANDLE, SPRITE_PIPELINE_HANDLE},
    transform::components::{GlobalTransform, Transform},
};
use lyon_tessellation::{path::Path, FillOptions};

/// Component that marks a [`ShapeBundle`] as completed or not.
pub struct Processed(pub bool);

/// A Bevy [`Bundle`] to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: TessellationMode,
    pub processed: Processed,
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
            processed: Processed(false),
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
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
