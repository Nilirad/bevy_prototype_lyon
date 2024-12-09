//! Custom Bevy ECS bundle for shapes.

use bevy::prelude::*;
use lyon_tessellation::{self as tess};

use crate::{geometry::Geometry, plugin::COLOR_MATERIAL_HANDLE};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle, Clone)]
pub struct ShapeBundle {
    pub path: Shape,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub visibility: Visibility,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: default(),
            mesh: default(),
            material: MeshMaterial2d(COLOR_MATERIAL_HANDLE),
            transform: default(),
            visibility: default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component, Default, Clone)]
pub struct Shape(pub tess::path::Path);

impl Geometry for Shape {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
