//! Custom Bevy ECS bundle for shapes.

use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    render::{
        color::Color,
        draw::{Draw, Visible},
        mesh::Mesh,
        pipeline::{RenderPipeline, RenderPipelines},
        render_graph::base::MainPass,
    },
    sprite::QUAD_HANDLE,
    transform::components::{GlobalTransform, Transform},
    ui::{Node, Style},
};
use lyon_tessellation::{path::Path, FillOptions};

use crate::{
    render::{SHAPE_PIPELINE_HANDLE, UI_SHAPE_PIPELINE_HANDLE},
    utils::{DrawMode, FillMode},
};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: DrawMode,
    pub mesh: Handle<Mesh>,
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
            mode: DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::WHITE,
            }),
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SHAPE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Visible::default()
            },
            main_pass: MainPass,
            draw: Draw::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

/// A Bevy `Bundle` to represent a shape visible by a UI camera.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct UiShapeBundle {
    pub node: Node,
    pub style: Style,
    pub path: Path,
    pub mode: DrawMode,
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for UiShapeBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            style: Style::default(),
            path: Path::new(),
            mode: DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::WHITE,
            }),
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                UI_SHAPE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Visible::default()
            },
            draw: Draw::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}
