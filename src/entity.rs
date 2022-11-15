//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    render::{
        color::Color,
        view::{ComputedVisibility, Visibility},
    },
    sprite::Mesh2dHandle,
    transform::components::{GlobalTransform, Transform},
};
use lyon_tessellation::{self as tess, FillOptions};

use crate::{
    draw::{DrawMode, FillMode},
    prelude::Geometry,
    render::Shape,
};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: DrawMode,
    pub shape: Shape,
    pub mesh2d: Mesh2dHandle,
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
            shape: Shape::default(),
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

impl Geometry for Path {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
