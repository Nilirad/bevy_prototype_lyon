//! This example shows how you can make semitransparent shapes using the alpha
//! channel.

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const TRANSPARENT_RED: Color = Color::rgba_linear(1.0, 0.0, 0.0, 0.5);
const TRANSPARENT_GREEN: Color = Color::rgba_linear(0.0, 1.0, 0.0, 0.5);
const TRANSPARENT_BLUE: Color = Color::rgba_linear(0.0, 0.0, 1.0, 0.5);
const CIRCLE_RADIUS: f32 = 100.0;
const PI_2: f32 = 2.0 * std::f32::consts::PI;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_resource(ClearColor(Color::BLACK))
        .add_startup_system(startup.system())
        .run();
}

fn startup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let circle = shapes::Circle {
        radius: CIRCLE_RADIUS,
        ..Default::default()
    };

    let colors = vec![TRANSPARENT_RED, TRANSPARENT_GREEN, TRANSPARENT_BLUE];
    let num_colors = colors.len();
    for (i, color) in colors.into_iter().enumerate() {
        let x = CIRCLE_RADIUS / 2.0 * (PI_2 / num_colors as f32 * i as f32).cos();
        let y = CIRCLE_RADIUS / 2.0 * (PI_2 / num_colors as f32 * i as f32).sin();
        commands.spawn(circle.draw(
            materials.add(ColorMaterial::color(color)),
            TessellationMode::Fill(FillOptions::default()),
            Transform {
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
        ));
    }
}
