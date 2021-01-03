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
    let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());

    commands.spawn(Camera2dBundle::default()).spawn(primitive(
        material.clone(),
        &mut meshes,
        ShapeType::Circle(60.0),
        TessellationMode::Fill(&FillOptions::default()),
        Vec3::new(0.0, 0.0, 0.0).into(),
    ));
}
