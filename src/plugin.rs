//! Contains the plugin and its helper types.
//!
//! The `ShapePlugin`, used at its fullest, provides the creation of shapes with
//! minimal boilerplate.
//!
//! ## How it works
//! When the user calls the [`ShapeSprite::draw`](super::ShapeSprite::draw)
//! method from a system in the `UPDATE` stage, it will return a
//! `(ShapeDescriptor, )` type, a single element tuple that gets feeded to
//! Bevy's `Commands::spawn` method as a bundle.
//!
//! Then, in the [`SHAPE`](shape_plugin_stage::SHAPE) stage, there is a system
//! that for each entity containing `ShapeDescriptor`, it inserts the
//! `SpriteBundle` components into the entity and then removes the
//! `ShapeDescriptor` component.

// TODO: Show use of the alternative drawing function.

use crate::{build_mesh, Buffers, ShapeSprite, TessellationMode, Tessellator, VertexConstructor};
use bevy::{
    app::{stage, AppBuilder, Plugin},
    asset::{Assets, Handle},
    ecs::{Commands, Entity, IntoSystem, Query, ResMut, SystemStage},
    math::Vec2,
    prelude::SpriteBundle,
    render::mesh::Mesh,
    sprite::{ColorMaterial, Sprite},
    transform::components::Transform,
};
use lyon_tessellation::BuffersBuilder;

/// Stages for this plugin.
pub mod shape_plugin_stage {
    /// The stage where the [`ShapeDescriptor`](super::ShapeDescriptor)s are
    /// replaced with `SpriteBundles`.
    pub const SHAPE: &str = "shape";
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
/// `SpriteBundle` with a custom mesh.
pub struct ShapeDescriptor {
    pub shape: Box<dyn ShapeSprite + Send + Sync>,
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
        let path = shape_descriptor.shape.generate_path();

        let mut buffers = Buffers::new();

        match shape_descriptor.mode {
            TessellationMode::Fill(ref options) => {
                tessellator
                    .fill
                    .tessellate_path(
                        &path,
                        options,
                        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
                    )
                    .unwrap();
            }
            TessellationMode::Stroke(ref options) => {
                tessellator
                    .stroke
                    .tessellate_path(
                        &path,
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
