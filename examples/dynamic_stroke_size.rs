//! Demonstrate surgical changes on the fields of `Shape` (path, fill, stroke).
//! The triangle changes fill color; the hexagon changes stroke width; and
//! the node positions of all shapes are extracted from the path to apply different
//! rotations on size.

use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, redraw_line_width)
        .add_systems(Update, redraw_fill)
        .add_systems(Update, rotate_shape_by_size)
        .run();
}

// Marker traits to uniquely identify entities.
#[derive(Component)]
struct HexagonShape;
#[derive(Component)]
struct TriangleShape;

/// Walk Path to get the maximum x coordinate.
fn get_max_x(shape: &Shape) -> f32 {
    shape
        .path
        .iter()
        .map(|p| p.to().x)
        .chain(shape.path.iter().map(|p| p.from().x))
        .fold(0f32, |acc, x| if x - acc > 1e-8 { x } else { acc })
}

/// Over time, rotate smaller shapes faster.
fn rotate_shape_by_size(mut query: Query<(&mut Transform, &Shape)>, time: Res<Time>) {
    let delta = time.delta_secs();

    for (mut transform, shape) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(200.0 / get_max_x(shape) * delta));
    }
}

/// Change line width of the hexagon over time.
fn redraw_line_width(mut query: Query<&mut Shape, With<HexagonShape>>, time: Res<Time>) {
    let outline_width = 2.0 + time.elapsed_secs_f64().sin().abs() * 10.0;

    let mut shape = query.single_mut();
    shape.stroke = shape.stroke.map(|mut s| {
        s.options.line_width = outline_width as f32;
        s
    });
}

/// Change fill color of the triangle over time.
fn redraw_fill(mut query: Query<&mut Shape, With<TriangleShape>>, time: Res<Time>) {
    let hue = (time.elapsed_secs_f64() * 50.0) % 360.0;
    let color = Color::hsl(hue as f32, 1.0, 0.5);

    let mut shape = query.single_mut();
    shape.fill = shape.fill.map(|mut f| {
        f.color = color;
        f
    });
}

fn setup_system(mut commands: Commands) {
    let triangle = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(100.0),
        ..shapes::RegularPolygon::default()
    };
    let hexagon = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };
    let big_square = shapes::RegularPolygon {
        sides: 4,
        feature: shapes::RegularPolygonFeature::Radius(300.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn((
        ShapeBuilder::with(&triangle)
            .fill(DARK_CYAN)
            .stroke((BLACK, 10.0))
            .build(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, 2.0)),
        TriangleShape,
    ));
    commands.spawn((
        ShapeBuilder::with(&hexagon)
            .fill(DARK_CYAN)
            .stroke((BLACK, 10.0))
            .build(),
        Transform::default().with_translation(Vec3::new(0.0, 0.0, 1.0)),
        HexagonShape,
    ));
    commands.spawn((ShapeBuilder::with(&big_square)
        .fill(ORANGE)
        .stroke((BLACK, 10.0))
        .build(),));
}
