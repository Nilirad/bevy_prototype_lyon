//! Custom Bevy ECS bundle for shapes.
#![expect(deprecated)]

use bevy::prelude::*;
use lyon_algorithms::path::Builder;
use lyon_tessellation::{self as tess};

use crate::{
    draw::{Fill, Stroke},
    geometry::Geometry,
    plugin::COLOR_MATERIAL_HANDLE,
};

/// A Bevy `Bundle` to represent a shape.
#[deprecated(since = "0.14.0", note = "please use the `Shape` component instead.")]
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

/// `Component` describing a geometric shape.
///
/// It can be constructed using `ShapeBuilder`.
#[derive(Component, Default, Clone)]
#[require(Mesh2d, MeshMaterial2d<ColorMaterial>(color_material_handle), Transform, Visibility)]
#[non_exhaustive]
pub struct Shape {
    /// Geometry of a shape.
    pub path: tess::path::Path,
    /// Fill data, changes are propagated to the mesh.
    pub fill: Option<Fill>,
    /// Stroke data, changes are propagated to the mesh.
    pub stroke: Option<Stroke>,
}

impl Shape {
    pub(crate) fn new(path: tess::path::Path, fill: Option<Fill>, stroke: Option<Stroke>) -> Self {
        Self { path, fill, stroke }
    }

    /// Returns the reference to the path of the shape.
    #[allow(clippy::must_use_candidate)]
    pub fn path_ref(&self) -> &tess::path::Path {
        &self.path
    }

    /// Returns the fill options of the shape.
    #[allow(clippy::must_use_candidate)]
    pub fn fill(&self) -> Option<Fill> {
        self.fill
    }

    /// Returns the stroke options of the shape.
    #[allow(clippy::must_use_candidate)]
    pub fn stroke(&self) -> Option<Stroke> {
        self.stroke
    }
}

impl Geometry<Builder> for Shape {
    fn add_geometry(&self, b: &mut Builder) {
        b.extend_from_paths(&[self.path.as_slice()]);
    }
}

fn color_material_handle() -> MeshMaterial2d<ColorMaterial> {
    MeshMaterial2d(COLOR_MATERIAL_HANDLE)
}
