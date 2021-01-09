use crate::{ShapeSprite, TessellationMode, Tessellator};
use bevy::prelude::*;

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let tessellator = Tessellator::new();
        app.add_resource(tessellator)
            .add_system_to_stage(stage::POST_UPDATE, shapesprite_maker.system());
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
        assert_eq!(
            shape_descriptor.transform.translation,
            Vec3::new(800.0, 0.0, 0.0)
        );
        commands
            .spawn(shape_descriptor.shape.generate_sprite(
                shape_descriptor.material.clone(),
                &mut meshes,
                &mut tessellator,
                shape_descriptor.mode.clone(),
                shape_descriptor.transform,
            ))
            .despawn(entity);
    }
}
