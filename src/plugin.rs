use crate::{ShapeSprite, TessellationMode, Tessellator};
use bevy::{
    app::{stage, AppBuilder, Plugin},
    asset::{Assets, Handle},
    ecs::{Commands, Entity, IntoSystem, Query, ResMut, SystemStage},
    render::mesh::Mesh,
    sprite::ColorMaterial,
    transform::components::Transform,
};

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
        let sprite_bundle = shape_descriptor.shape.generate_sprite(
            &path,
            shape_descriptor.material.clone(),
            &mut meshes,
            &mut tessellator,
            shape_descriptor.mode,
            shape_descriptor.transform,
        );

        commands.spawn(sprite_bundle).despawn(entity);
    }
}
