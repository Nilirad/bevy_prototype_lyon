use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(change_color)
        .add_system(change_stroke)
        .add_system(rotate)
        .run();
}

struct ExampleShape;

fn rotate(mut query: Query<&mut Transform, With<ExampleShape>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let delta = time.delta_seconds();

        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

fn change_color(mut query: Query<&mut ShapeColors>, time: Res<Time>) {
    for mut colors in query.iter_mut() {
        let h = (time.seconds_since_startup() * 50.0) % 360.0;

        *colors = ShapeColors {
            main: Color::hsl(h as f32, 1.0, 0.5),
            outline: Color::BLACK,
        };
    }
}

fn change_stroke(mut query: Query<&mut DrawMode>, time: Res<Time>) {
    for mut draw_mode in query.iter_mut() {
        let w = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

        *draw_mode = DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(w as f32),
        }
    }
}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::TEAL, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(10.0),
            },
            Transform::default(),
        ))
        .insert(ExampleShape);
}
