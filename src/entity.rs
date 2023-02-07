//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    render::color::Color,
    sprite::MaterialMesh2dBundle,
};
use lyon_tessellation::{self as tess, FillOptions};

use crate::{
    draw::{DrawMode, FillMode},
    prelude::Geometry,
    render::ShapeMaterial,
};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    #[bundle]
    pub mesh2d_bundle: MaterialMesh2dBundle<ShapeMaterial>,
    pub path: Path,
    pub mode: DrawMode,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mode: DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::WHITE,
            }),
            mesh2d_bundle: MaterialMesh2dBundle::<ShapeMaterial>::default(),
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
