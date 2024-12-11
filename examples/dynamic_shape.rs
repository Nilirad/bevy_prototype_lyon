use std::f64::consts::PI;

use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, redraw_shape)
        .add_systems(Update, rotate_shape_system)
        .run();
}

#[derive(Component)]
struct ExampleShape;

fn rotate_shape_system(mut query: Query<&mut Transform, With<ExampleShape>>, time: Res<Time>) {
    let delta = time.delta_secs();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

// NOTE: `Fill` and `Stroke` are no longer supposed to be components,
// therefore they should not be queried for.
// Instead, query the `Shape` component, and edit the `fill` and `stroke`
// fields. NOTE: Fuse `change_draw_mode_system` and `change_number_of_sides`
// systems into a single `redraw_shape` system.
// This new system uses `ShapePath` to redefine
fn redraw_shape(mut query: Query<&mut Shape, With<ExampleShape>>, time: Res<Time>) {
    let hue = (time.elapsed_secs_f64() * 50.0) % 360.0;
    let color = Color::hsl(hue as f32, 1.0, 0.5);
    let outline_width = 2.0 + time.elapsed_secs_f64().sin().abs() * 10.0;
    let sides = ((time.elapsed_secs_f64() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;
    let polygon = shapes::RegularPolygon {
        sides,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    let mut shape = query.single_mut();
    *shape = ShapeBuilder::with(&polygon)
        .fill(color)
        .stroke((BLACK, outline_width as f32))
        .build();
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        ShapeBuilder::with(&shape)
            .fill(DARK_CYAN)
            .stroke((BLACK, 10.0))
            .build(),
        ExampleShape,
    ));
}
