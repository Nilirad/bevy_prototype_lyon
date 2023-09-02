//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    prelude::{Handle, SpatialBundle},
    sprite::Mesh2dHandle,
};
use lyon_tessellation::{self as tess};

use crate::{prelude::Geometry, render::ShapeMaterial};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle, Default)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ShapeMaterial>,
    pub spatial: SpatialBundle,
}

#[allow(missing_docs)]
#[derive(Component, Default)]
pub struct Path(pub tess::path::Path);

impl Geometry for Path {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
