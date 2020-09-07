//! Types to lay out and draw paths.

use crate::{create_sprite, Geometry};
use bevy::prelude::*;
use lyon::{
    math::Point,
    path::{self, Builder},
    tessellation::{
        BuffersBuilder, FillAttributes, FillOptions, FillTessellator, StrokeAttributes,
        StrokeOptions, StrokeTessellator, VertexBuffers,
    },
};

/// Used to construct a [`Path`](Path) using the builder pattern.
pub struct PathBuilder(pub Builder);

impl PathBuilder {
    /// Creates a new `PathBuilder` and moves its current postition to
    /// `position`.
    pub fn new() -> Self {
        Self(path::Path::builder())
    }

    /// Moves the current position of the path without adding any drawing
    /// commands. The initial position its reset to the new position, such that
    /// calling [close](PathBuilder::close) will draw a line to this new point.
    pub fn move_to(&mut self, point: Point) {
        self.0.move_to(point);
    }

    /// Adds a line from the current position to `point`.
    pub fn line_to(&mut self, point: Point) {
        self.0.line_to(point);
    }

    /// Adds a line from the current position to the initial position of the
    /// path.
    pub fn close(&mut self) {
        self.0.close();
    }

    /// Adds a quadratic bezier curve to `point` using the `control` point.
    pub fn quadratic_bezier_to(&mut self, control: Point, point: Point) {
        self.0.quadratic_bezier_to(control, point);
    }

    /// Adds a cubic bezier curve to `point` using two control points.
    pub fn cubic_bezier_to(&mut self, control_1: Point, control_2: Point, point: Point) {
        self.0.cubic_bezier_to(control_1, control_2, point);
    }

    // TODO: Implement arc.

    /// Builds the path.
    pub fn build(self) -> Path {
        Path(self.0.build())
    }
}

/// Contains path data that can be used to get a `SpriteComponents` bundle with
/// a custom shape. Check out [`PathBuilder`](PathBuilder) to construct it.
pub struct Path(path::Path);

impl Path {
    /// Returns a `SpriteComponents` with the filled path as the mesh.
    pub fn fill(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        translation: Translation,
        options: &FillOptions,
    ) -> SpriteComponents {
        let mut tessellator = FillTessellator::new();
        let mut geometry = Geometry(VertexBuffers::new());
        tessellator
            .tessellate_path(
                self.0.as_slice(),
                options,
                &mut BuffersBuilder::new(&mut geometry.0, |pos: Point, _: FillAttributes| {
                    [pos.x, pos.y, 0.0]
                }),
            )
            .unwrap();

        create_sprite(material, meshes, geometry, translation)
    }

    /// Returns a `SpriteComponents` with the stroked path as the mesh.
    pub fn stroke(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        translation: Translation,
        options: &StrokeOptions,
    ) -> SpriteComponents {
        let mut tessellator = StrokeTessellator::new();
        let mut geometry = Geometry(VertexBuffers::new());
        tessellator
            .tessellate_path(
                self.0.as_slice(),
                options,
                &mut BuffersBuilder::new(&mut geometry.0, |pos: Point, _: StrokeAttributes| {
                    [pos.x, pos.y, 0.0]
                }),
            )
            .unwrap();

        create_sprite(material, meshes, geometry, translation)
    }
}
