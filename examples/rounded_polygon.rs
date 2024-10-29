use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let points = [
        Vec2::new(-1.0, -0.3),
        Vec2::new(0.0, -0.3),
        Vec2::new(0.0, -1.0),
        Vec2::new(1.5, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.3),
        Vec2::new(-1.0, 0.3),
    ]
    .map(|x| x * 100.);

    let shape = shapes::RoundedPolygon {
        points: points.into_iter().collect(),
        radius: 10.,
        closed: false,
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(DARK_CYAN),
    ));
}
