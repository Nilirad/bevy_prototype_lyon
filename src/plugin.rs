//! Contains the plugin and its helper types.
//!
//! The `ShapePlugin` provides the creation of shapes with minimal boilerplate.
//!
//! ## How it works
//! When the user calls the [`ShapeSprite::draw`] or [`Multishape::build`]
//! method from a system in the [`UPDATE`](bevy_app::stage::UPDATE) stage, it
//! will return a `(ShapeDescriptor,)` type, a single element tuple that have to
//! be feeded to Bevy's [`Commands::spawn`](bevy_ecs::Commands::spawn) method
//! as a bundle.
//!
//! Then, in the [`SHAPE`](shape_plugin_stage::SHAPE) stage, there is a system
//! that for each entity containing `ShapeDescriptor`, it inserts the
//! [`SpriteBundle`] components into the entity and then removes the
//! `ShapeDescriptor` component.

use crate::{build_mesh, Buffers, VertexConstructor};
use bevy_app::{stage, AppBuilder, Plugin};
use bevy_asset::{Assets, Handle};
use bevy_ecs::{Commands, Entity, IntoSystem, Query, ResMut, SystemStage};
use bevy_math::Vec2;
use bevy_render::mesh::Mesh;
use bevy_sprite::{entity::SpriteBundle, ColorMaterial, Sprite};
use bevy_transform::components::Transform;
use lyon_tessellation::{
    path::{path::Builder, Path},
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
};

/// Stages for this plugin.
pub mod shape_plugin_stage {
    /// The stage where the [`ShapeDescriptor`](super::ShapeDescriptor)s are
    /// replaced with `SpriteBundles`.
    pub const SHAPE: &str = "shape";
}

/// Determines if a shape must be filled or stroked.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TessellationMode {
    /// The shape should be filled with the provided [`FillOptions`].
    Fill(FillOptions),
    /// The shape should be filled with the provided [`StrokeOptions`].
    Stroke(StrokeOptions),
}

/// A couple of `lyon` fill and stroke tessellators.
pub struct Tessellator {
    /// Tessellates the entire shape defined by the lyon [`Path`].
    pub fill: FillTessellator,
    /// Tessellates the border of the shape defined by the lyon [`Path`].
    pub stroke: StrokeTessellator,
}

impl Tessellator {
    /// Creates a new `Tessellator` data structure, containing the two types of
    /// Lyon tessellator.
    pub fn new() -> Self {
        Self {
            fill: FillTessellator::new(),
            stroke: StrokeTessellator::new(),
        }
    }
}

/// A plugin that provides resources and a system to draw shapes in Bevy with
/// less boilerplate.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let tessellator = Tessellator::new();
        app.add_resource(tessellator)
            .add_stage_after(
                stage::UPDATE,
                shape_plugin_stage::SHAPE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(shape_plugin_stage::SHAPE, shapesprite_maker.system());
    }
}

/// An intermediate representation that contains all the data to create a
/// [`SpriteBundle`] with a custom mesh.
///
/// If spawned into the [`World`](bevy_ecs::World) during the
/// [`UPDATE`](bevy_app::stage::UPDATE) stage, it will be replaced by a custom
/// `SpriteBundle` corresponding to the
/// shape.
#[allow(missing_docs)]
pub struct ShapeDescriptor {
    pub path: Path,
    pub material: Handle<ColorMaterial>,
    pub mode: TessellationMode,
    pub transform: Transform,
}

/// A bevy system. Queries all the [`ShapeDescriptor`]s to create a
/// `SpriteBundle` for each one, before deleting them.
fn shapesprite_maker(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tessellator: ResMut<Tessellator>,
    query: Query<(Entity, &ShapeDescriptor)>,
) {
    for (entity, shape_descriptor) in query.iter() {
        let mut buffers = Buffers::new();

        match shape_descriptor.mode {
            TessellationMode::Fill(ref options) => {
                tessellator
                    .fill
                    .tessellate_path(
                        &shape_descriptor.path,
                        options,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(ref options) => {
                tessellator
                    .stroke
                    .tessellate_path(
                        &shape_descriptor.path,
                        options,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                    )
                    .unwrap();
            }
        }

        let sprite_bundle = SpriteBundle {
            material: shape_descriptor.material.clone(),
            mesh: meshes.add(build_mesh(&buffers)),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            transform: shape_descriptor.transform,
            ..Default::default()
        };

        commands.insert(entity, sprite_bundle);
        commands.remove_one::<ShapeDescriptor>(entity);
    }
}

/// Shape structs that implement this trait can be transformed into a
/// [`SpriteBundle`]. See the [`shapes`](crate::shapes) module for some
/// examples.
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

    /// Returns a [`ShapeDescriptor`] bundle for the shape.
    fn draw(
        &self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> (ShapeDescriptor,) {
        let mut builder = Builder::new();
        self.add_geometry(&mut builder);

        let desc = ShapeDescriptor {
            path: builder.build(),
            material: material.clone(),
            mode,
            transform,
        };

        (desc,)
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

    /// Generates a `(ShapeDescriptor,)` with all the added shapes.
    pub fn build(
        self,
        material: Handle<ColorMaterial>,
        mode: TessellationMode,
        transform: Transform,
    ) -> (ShapeDescriptor,) {
        let desc = ShapeDescriptor {
            path: self.0.build(),
            material: material.clone(),
            mode,
            transform,
        };

        (desc,)
    }
}

/// Generates a [`ShapeDescriptor`] bundle with an arbitrary Path.
pub fn draw_path(
    path: &Path,
    material: Handle<ColorMaterial>,
    mode: TessellationMode,
    transform: Transform,
) -> (ShapeDescriptor,) {
    let desc = ShapeDescriptor {
        path: path.clone(),
        material: material.clone(),
        mode,
        transform,
    };

    (desc,)
}
