use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(0., 0.));
    path_builder.cubic_bezier_to(
        Vec2::new(70., 70.),
        Vec2::new(175., -35.),
        Vec2::new(0., -140.),
    );
    path_builder.cubic_bezier_to(
        Vec2::new(-175., -35.),
        Vec2::new(-70., 70.),
        Vec2::new(0., 0.),
    );
    path_builder.close();
    let path = path_builder.build();

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        path,
        Transform::from_xyz(0., 75., 0.),
        Stroke::new(BLACK, 10.0),
        Fill::color(RED),
    ));
}
