//! Contains the plugin and its helper types.
//!
//! The [`ShapePlugin`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`ShapeBundle`] from a system in the
//! [`UPDATE`](bevy::app::stage::UPDATE) stage.
//!
//! Then, in the [`SHAPE`](stage::SHAPE) stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `ShapeBundle`.

use crate::{
    build_mesh,
    entity::{Processed, ShapeBundle},
    VertexBuffers, VertexConstructor,
};
use bevy::{
    app::{AppBuilder, Plugin},
    asset::{Assets, Handle},
    ecs::{IntoSystem, Query, ResMut, SystemStage},
    render::{draw::Visible, mesh::Mesh},
    sprite::ColorMaterial,
    transform::components::Transform,
};
use lyon_tessellation::{
    path::{path::Builder, Path},
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
};

/// Stages for this plugin.
pub mod stage {
    /// The stage where the [`ShapeBundle`](super::ShapeBundle) gets completed.
    pub const SHAPE: &str = "shape";
}

/// Determines if a shape must be filled or stroked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TessellationMode {
    /// The shape will be filled with the provided [`FillOptions`].
    Fill(FillOptions),
    /// The shape will be filled with the provided [`StrokeOptions`].
    Stroke(StrokeOptions),
}

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let fill_tess = FillTessellator::new();
        let stroke_tess = StrokeTessellator::new();
        app.add_resource(fill_tess)
            .add_resource(stroke_tess)
            .add_stage_after(
                bevy::app::stage::UPDATE,
                stage::SHAPE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(stage::SHAPE, complete_shape_bundle.system());
    }
}

/// A bevy system. Queries all the [`ShapeBundle`]s to complete them with a
/// mesh.
fn complete_shape_bundle(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<(
        &TessellationMode,
        &Path,
        &mut Handle<Mesh>,
        &mut Visible,
        &mut Processed,
    )>,
) {
    for (tess_mode, path, mut mesh, mut visible, mut processed) in query.iter_mut() {
        if processed.0 {
            continue;
        }

        let mut buffers = VertexBuffers::new();

        match tess_mode {
            TessellationMode::Fill(ref options) => {
                fill_tess
                    .tessellate_path(
                        path,
                        options,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(ref options) => {
                stroke_tess
                    .tessellate_path(
                        path,
                        options,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                    )
                    .unwrap();
            }
        }

        *mesh = meshes.add(build_mesh(&buffers));
        visible.is_visible = true;
        *processed = Processed(true);
    }
}

/// Shape structs that implement this trait can be drawn as a shape. See the
/// [`shapes`](crate::shapes) module for some examples.
///
/// # Implementation example
///
/// ```
/// use bevy_prototype_lyon::plugin::Geometry;
/// use lyon_tessellation::{
///     math::{Point, Rect, Size},
///     path::{path::Builder, traits::PathBuilder, Path, Winding},
/// };
///
/// // First, create a struct to hold the shape features:
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// pub struct Rectangle {
///     pub width: f32,
///     pub height: f32,
/// }
///
/// // Implementing the `Default` trait is not required, but it may facilitate the
/// // definition of the shape before spawning it.
/// impl Default for Rectangle {
///     fn default() -> Self {
///         Self {
///             width: 1.0,
///             height: 1.0,
///         }
///     }
/// }
///
/// // Finally, implement the `generate_path` method.
/// impl Geometry for Rectangle {
///     fn add_geometry(&self, b: &mut Builder) {
///         b.add_rectangle(
///             &Rect::new(Point::zero(), Size::new(self.width, self.height)),
///             Winding::Positive,
///         );
///     }
/// }
/// ```
pub trait Geometry {
    /// Adds the geometry of the shape to the given Lyon [`Builder`].
    fn add_geometry(&self, b: &mut Builder);
}

impl Geometry for Path {
    fn add_geometry(&self, b: &mut Builder) {
        b.concatenate(&[self.as_slice()])
    }
}

/// Allows the creation of multiple shapes using only a single mesh.
pub struct GeometryBuilder(Builder);

impl GeometryBuilder {
    /// Creates a new, empty `ShapeBuilder`.
    pub fn new() -> Self {
        Self(Builder::new())
    }

    /// Adds a shape.
    pub fn add(&mut self, shape: &impl Geometry) -> &mut Self {
        shape.add_geometry(&mut self.0);

        self
    }

    /// Generates a [`ShapeBundle`] with all the added shapes.
    pub fn build(
        self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> ShapeBundle {
        ShapeBundle {
            path: self.0.build(),
            material,
            mode,
            transform,
            ..Default::default()
        }
    }

    /// Generates a [`ShapeBundle`] with only one shape.
    pub fn build_as(
        shape: &impl Geometry,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> ShapeBundle {
        let mut multishape = Self::new();
        multishape.add(shape);
        multishape.build(material, mode, transform)
    }
}
