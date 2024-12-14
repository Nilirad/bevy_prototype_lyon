use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let path = ShapePath::new()
        .move_to(Vec2::new(0., 0.))
        .cubic_bezier_to(
            Vec2::new(70., 70.),
            Vec2::new(175., -35.),
            Vec2::new(0., -140.),
        )
        .cubic_bezier_to(
            Vec2::new(-175., -35.),
            Vec2::new(-70., 70.),
            Vec2::new(0., 0.),
        )
        .close();

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        ShapeBuilder::with(&path)
            .fill(RED)
            .stroke((BLACK, 10.0))
            .build(),
        Transform::from_xyz(0., 75., 0.),
    ));
}
