use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
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

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ShapeBundle {
            path,
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., 75., 0.),
                ..default()
            },
            ..default()
        },
        Stroke::new(LegacyColor::BLACK, 10.0),
        Fill::color(LegacyColor::RED),
    ));
}
