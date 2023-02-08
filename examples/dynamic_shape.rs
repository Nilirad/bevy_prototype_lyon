use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(change_draw_mode_system)
        .add_system(change_number_of_sides)
        .add_system(rotate_shape_system)
        .run();
}

#[derive(Component)]
struct ExampleShape;

fn rotate_shape_system(mut query: Query<&mut Transform, With<ExampleShape>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

fn change_draw_mode_system(mut query: Query<(&mut Fill, &mut Stroke)>, time: Res<Time>) {
    let hue = (time.elapsed_seconds_f64() * 50.0) % 360.0;
    let outline_width = 2.0 + time.elapsed_seconds_f64().sin().abs() * 10.0;

    for (mut fill_mode, mut stroke_mode) in query.iter_mut() {
        fill_mode.color = Color::hsl(hue as f32, 1.0, 0.5);
        stroke_mode.options.line_width = outline_width as f32;
    }
}

fn change_number_of_sides(mut query: Query<&mut Path>, time: Res<Time>) {
    let sides = ((time.elapsed_seconds_f64() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(200.0),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 10.0),
        ExampleShape,
    ));
}
