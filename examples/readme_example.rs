//! This is the example that goes to the README.md file. The README.md should be
//! updated before every release.

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ShapeMaterial>>) {
    let circle = shapes::Circle {
        radius: 100.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(GeometryBuilder::build_as(
            &circle,
            materials.add(ShapeMaterial::new(Color::AQUAMARINE)),
            TessellationMode::Fill(FillOptions::default()),
            Transform::default(),
        ));
}
