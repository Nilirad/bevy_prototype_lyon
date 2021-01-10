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

pub mod shape_plugin_stage {
    pub const SHAPE: &str = "shape";
}

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

pub struct ShapeDescriptor {
    pub shape: Box<dyn ShapeSprite + Send + Sync>,
    pub material: Handle<ColorMaterial>,
    pub mode: TessellationMode,
    pub transform: Transform,
}

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

        commands.spawn(sprite_bundle).despawn(entity);
    }
}
