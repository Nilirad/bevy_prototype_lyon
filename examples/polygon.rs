//! This example shows the capabilities of
//! [`RegularPolygon`](shape::RegularPolygon).

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
    let shape_radius = shapes::RegularPolygonFeature::Radius(100.0);
    let color = materials.add(ColorMaterial::color(Color::ORANGE_RED));
    let fill_mode = TessellationMode::Fill(FillOptions::default());

    let triangle = shapes::RegularPolygon {
        sides: 3,
        feature: shape_radius,
        center: Vec2::new(-400.0, 0.0),
    };

    let square = shapes::RegularPolygon {
        sides: 4,
        feature: shape_radius,
        center: Vec2::new(-200.0, 0.0),
    };

    let pentagon = shapes::RegularPolygon {
        sides: 5,
        feature: shape_radius,
        center: Vec2::new(0.0, 0.0),
    };

    let hexagon = shapes::RegularPolygon {
        sides: 6,
        feature: shape_radius,
        center: Vec2::new(200.0, 0.0),
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(ShapeBuilder::build_as(
            &triangle,
            color.clone(),
            fill_mode,
            Transform::default(),
        ))
        .spawn(ShapeBuilder::build_as(
            &square,
            color.clone(),
            fill_mode,
            Transform::default(),
        ))
        .spawn(ShapeBuilder::build_as(
            &pentagon,
            color.clone(),
            fill_mode,
            Transform::default(),
        ))
        .spawn(ShapeBuilder::build_as(
            &hexagon,
            color.clone(),
            fill_mode,
            Transform::default(),
        ));
}
