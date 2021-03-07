//! This is the example that goes to the README.md file. The README.md should be
//! updated before every release.

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: 100.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(GeometryBuilder::build_as(
            &circle,
            ShapeColors::outlined(Color::GOLD, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Transform::default(),
        ));
}
