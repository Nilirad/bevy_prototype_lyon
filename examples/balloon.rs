//! This example shows how you can draw arbitrary shapes using paths. Here is
//! shown a balloon taken from MDN HTML canvas tutorial.

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut builder = PathBuilder::new();
    builder.move_to(Vec2::new(75.0, 25.0));
    builder.quadratic_bezier_to(Vec2::new(25.0, 25.0), Vec2::new(25.0, 62.5));
    builder.quadratic_bezier_to(Vec2::new(25.0, 100.0), Vec2::new(50.0, 100.0));
    builder.quadratic_bezier_to(Vec2::new(50.0, 120.0), Vec2::new(30.0, 125.0));
    builder.quadratic_bezier_to(Vec2::new(60.0, 120.0), Vec2::new(65.0, 100.0));
    builder.quadratic_bezier_to(Vec2::new(125.0, 100.0), Vec2::new(125.0, 62.5));
    builder.quadratic_bezier_to(Vec2::new(125.0, 25.0), Vec2::new(75.0, 25.0));
    let path = builder.build();

    let fill_material = materials.add(ColorMaterial::color(Color::WHITE));
    let stroke_material = materials.add(ColorMaterial::color(Color::BLACK));
    let fill_mode = TessellationMode::Fill(FillOptions::default());
    let stroke_mode = TessellationMode::Stroke(
        StrokeOptions::default()
            .with_line_width(5.0)
            .with_line_join(LineJoin::Round),
    );
    let transform = Transform {
        translation: Vec3::new(-250.0, 250.0, 0.0),
        scale: Vec3::new(3.0, -3.0, 3.0),
        ..Default::default()
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(draw_path(
            &path,
            fill_material.clone(),
            fill_mode,
            transform,
        ))
        .spawn(draw_path(
            &path,
            stroke_material.clone(),
            stroke_mode,
            transform,
        ));
}
