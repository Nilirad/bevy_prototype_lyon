//! This example shows how you can use [`Multishape`] to add multiple shapes to
//! a single mesh.

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
    let circle = shapes::Circle {
        radius: 100.0,
        ..Default::default()
    };
    let square = shapes::RegularPolygon {
        sides: 4,
        center: Vec2::new(0.0, 200.0),
        feature: shapes::RegularPolygonFeature::SideLength(200.0),
    };
    let mut multishape = Multishape::new();
    multishape.add(circle).add(square);

    let material = materials.add(ColorMaterial::color(Color::CRIMSON));
    let mode = TessellationMode::Stroke(StrokeOptions::default().with_line_width(3.0));

    commands
        .spawn(Camera2dBundle::default())
        .spawn(multishape.build(material, mode, Transform::default()));
}
