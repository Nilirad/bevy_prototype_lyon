//! Custom Bevy ECS bundle for shapes.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use lyon_tessellation::{self as tess};

use crate::geometry::Geometry;

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[allow(missing_docs)]
#[derive(Component, Default)]
pub struct Path(pub tess::path::Path);

impl Geometry for Path {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
