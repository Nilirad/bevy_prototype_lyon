//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{ComputedVisibility, GlobalTransform, Transform, Visibility},
    render::color::Color,
    sprite::Mesh2dHandle,
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
    // The `Handle<Mesh>` needs to be wrapped in a `Mesh2dHandle` to use 2d rendering instead of 3d
    pub mesh2d: Mesh2dHandle,
    // These other components are needed for 2d meshes to be rendered
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
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
            mesh2d: Mesh2dHandle::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component)]
pub struct Path(pub tess::path::Path);
