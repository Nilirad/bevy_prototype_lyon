//! Custom Bevy ECS bundle for shapes.

use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    prelude::Visibility,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::{GlobalTransform, Transform},
    ui::{Node, Style},
};
use lyon_tessellation::{self as tess, FillOptions};

use crate::{
    draw::{DrawMode, FillMode},
    render::ColoredMesh2d,
};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: DrawMode,
    pub colored_mesh2d: ColoredMesh2d,
    #[bundle]
    pub mesh2d: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mode: DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::WHITE,
            }),
            colored_mesh2d: ColoredMesh2d::default(),
            mesh2d: MaterialMesh2dBundle::default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component)]
pub struct Path(pub tess::path::Path);
