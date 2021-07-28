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
};
use lyon_tessellation::{path::Path, FillOptions};

use crate::{render::SHAPE_PIPELINE_HANDLE, utils::DrawMode};

/// The colors assigned to a shape.
pub struct ShapeColors {
    /// The main color of the shape. It is the only color for fill and stroke
    /// shapes, and the fill color for the outlined shapes.
    pub main: Color,
    /// The outline color of the shape.
    pub outline: Color,
}

impl ShapeColors {
    /// Creates a `ShapeColors` structure with only one color.
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self {
            main: color,
            outline: Color::BLACK,
        }
    }

    /// Creates a complete `ShapeColor`, with main and outline colors.
    #[must_use]
    pub const fn outlined(fill: Color, outline: Color) -> Self {
        Self {
            main: fill,
            outline,
        }
    }
}

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: DrawMode,
    pub mesh: Handle<Mesh>,
    pub colors: ShapeColors,
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
            mode: DrawMode::Fill(FillOptions::default()),
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
            colors: ShapeColors {
                main: Color::WHITE,
                outline: Color::BLACK,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}
