use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
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

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
    ));
}
