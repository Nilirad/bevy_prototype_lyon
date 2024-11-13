use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let rect = shapes::Rectangle {
        extents: Vec2::splat(175.0),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(25.0)),
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rect),
            ..default()
        },
        Stroke::new(BLACK, 10.0),
        Fill::color(RED),
    ));
}
