use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let circle = CircleShape {
        radius: 100.0,
        ..Default::default()
    };

    let mut tessellator = Tessellator::only_fill();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(circle.generate_sprite(
            materials.add(ColorMaterial::color(Color::AQUAMARINE)),
            &mut meshes,
            &mut tessellator,
            TessellationMode::Fill(FillOptions::default()),
            Transform::default(),
        ));
}
