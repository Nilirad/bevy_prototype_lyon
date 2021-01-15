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

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let circle = shapes::Circle {
        radius: 100.0,
        ..Default::default()
    };

    commands.spawn(Camera2dBundle::default()).spawn(circle.draw(
        materials.add(ColorMaterial::color(Color::AQUAMARINE)),
        TessellationMode::Fill(FillOptions::default()),
        Transform::default(),
    ));
}
