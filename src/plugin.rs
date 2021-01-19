//! Contains the plugin and its helper types.
//!
//! The [`ShapePlugin`] provides the creation of shapes with minimal
//! boilerplate.
//!
//! ## How it works
//! The user spawns a [`ShapeBundle`] from a system in the
//! [`UPDATE`](bevy::app::stage::UPDATE) stage.
//!
//! Then, in the [`SHAPE`](shape_plugin_stage::SHAPE) stage, there is a system
//! that creates a mesh for each entity that has been spawned as a
//! `ShapeBundle`.

use crate::{build_mesh, entity::ShapeBundle, Buffers, VertexConstructor};
use bevy::{
    app::{stage, AppBuilder, Plugin},
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
pub mod shape_plugin_stage {
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
                stage::UPDATE,
                shape_plugin_stage::SHAPE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(shape_plugin_stage::SHAPE, shapesprite_maker.system());
    }
}

/// A bevy system. Queries all the [`ShapeBundle`]s to complete them with a
/// mesh.
fn shapesprite_maker(
    mut meshes: ResMut<Assets<Mesh>>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut query: Query<(&TessellationMode, &Path, &mut Handle<Mesh>, &mut Visible)>,
) {
    for (tess_mode, path, mut mesh, mut visible) in query.iter_mut() {
        let mut buffers = Buffers::new();

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
    }
}

/// Shape structs that implement this trait can be drawn as a shape. See the
/// [`shapes`](crate::shapes) module for some examples.
///
/// # Implementation example
///
/// ```
/// use bevy_prototype_lyon::plugin::ShapeSprite;
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
/// impl ShapeSprite for Rectangle {
///     fn add_geometry(&self, b: &mut Builder) {
///         b.add_rectangle(
///             &Rect::new(Point::zero(), Size::new(self.width, self.height)),
///             Winding::Positive,
///         );
///     }
/// }
/// ```
pub trait ShapeSprite {
    /// Mutates a Lyon path [`Builder`] adding the shape to it.
    fn add_geometry(&self, b: &mut Builder);

    /// Returns a [`ShapeBundle`] containing a [`Path`] with the designated
    /// shape.
    fn draw(
        &self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> ShapeBundle {
        let mut builder = Builder::new();
        self.add_geometry(&mut builder);

        ShapeBundle {
            path: builder.build(),
            material,
            mode,
            transform,
            ..Default::default()
        }
    }
}

/// Allows the creation of multiple shapes using only a single mesh.
pub struct Multishape(Builder);

impl Multishape {
    /// Creates a new, empty `Multishape`.
    pub fn new() -> Self {
        Self(Builder::new())
    }

    /// Adds a shape.
    pub fn add(&mut self, shape: impl ShapeSprite) -> &mut Self {
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
}

/// Generates a [`ShapeBundle`] with an arbitrary [`Path`].
pub fn draw_path(
    path: &Path,
    material: Handle<ColorMaterial>,
    mode: TessellationMode,
    transform: Transform,
) -> ShapeBundle {
    ShapeBundle {
        path: path.clone(),
        material,
        mode,
        transform,
        ..Default::default()
    }
}
