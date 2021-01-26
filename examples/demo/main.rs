//! This demo shows the main features of the `bevy_prototype_lyon` library.

// rustc
#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, unused)]
#![allow(elided_lifetimes_in_paths)]
// clippy
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::multiple_crate_versions)] // dependency problem
#![allow(clippy::needless_pass_by_value)] // interferes with Bevy system creation

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{
    physics::{RapierConfiguration, RapierPhysicsPlugin},
    rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, InteractionGroups},
    },
};
use demo_camera_plugin::DemoCameraPlugin;
use demo_inspector_plugin::DemoInspectorPlugin;
use demo_ui_plugin::DemoUiPlugin;
use rand::Rng;

mod demo_camera_plugin;
mod demo_inspector_plugin;
mod demo_ui_plugin;

const WINDOW_WIDTH: f32 = 1280.0;

/// Tags the multishape entity.
pub struct MultishapeTag;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(DemoCameraPlugin)
        .add_plugin(DemoUiPlugin)
        .add_plugin(DemoInspectorPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = bevy_rapier2d::na::Vector2::new(0.0, 0.0);

    physics_showoff(commands, &mut materials);
    path_builder_usage(commands, &mut materials);
    show_multishape(commands, &mut materials);
}

fn physics_showoff(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    const TRANSPARENT_RED: Color = Color::rgba_linear(1.0, 0.0, 0.0, 0.5);
    const TRANSPARENT_GREEN: Color = Color::rgba_linear(0.0, 1.0, 0.0, 0.5);
    const TRANSPARENT_BLUE: Color = Color::rgba_linear(0.0, 0.0, 1.0, 0.5);
    const CIRCLE_RADIUS: f32 = 100.0;
    const PI_2: f32 = 2.0 * std::f32::consts::PI;
    let circle = shapes::Circle {
        radius: CIRCLE_RADIUS,
        ..shapes::Circle::default()
    };

    physics_static_geometry(commands);

    let mut rng = rand::thread_rng();
    let circle_interaction_groups = InteractionGroups::new(0x0002, 0x0001);
    let colors = vec![TRANSPARENT_RED, TRANSPARENT_GREEN, TRANSPARENT_BLUE];
    let num_colors = colors.len();
    for (i, color) in colors.into_iter().enumerate() {
        let x = CIRCLE_RADIUS * 1.5 * (PI_2 / num_colors as f32 * i as f32).cos();
        let y = CIRCLE_RADIUS * 1.5 * (PI_2 / num_colors as f32 * i as f32).sin();

        let dir_x = (rng.gen::<f32>() - 0.5) * 2.0; // rand number in [-1.0, 1.0]
        let dir_y = (rng.gen::<f32>() - 0.5) * 2.0;
        let speed = (rng.gen::<f32>() * 75.0).max(25.0);
        let vel = Vec2::new(dir_x, dir_y).normalize() * speed;

        let rigid_body = RigidBodyBuilder::new_dynamic()
            .translation(x + WINDOW_WIDTH, y)
            .linvel(vel.x, vel.y);
        let collider = ColliderBuilder::ball(CIRCLE_RADIUS)
            .friction(0.0)
            .restitution(1.0)
            .collision_groups(circle_interaction_groups);

        commands
            .spawn(GeometryBuilder::build_as(
                &circle,
                materials.add(ColorMaterial::color(color)),
                TessellationMode::Fill(FillOptions::default()),
                Transform::default(), // Rapier overrides transform.
            ))
            .with(rigid_body)
            .with(collider);
    }
}

fn physics_static_geometry(commands: &mut Commands) {
    let wall_interaction_groups = InteractionGroups::new(0x0001, 0x0002);

    // screen bottom
    let rigid_body = RigidBodyBuilder::new_static().translation(1280.0, -360.5);
    let collider = ColliderBuilder::cuboid(640.0, 1.0)
        .friction(0.0)
        .restitution(1.0)
        .collision_groups(wall_interaction_groups);
    commands.spawn((rigid_body, collider));
    // screen top
    let rigid_body = RigidBodyBuilder::new_static().translation(1280.0, 360.5);
    let collider = ColliderBuilder::cuboid(640.0, 1.0)
        .friction(0.0)
        .restitution(1.0)
        .collision_groups(wall_interaction_groups);
    commands.spawn((rigid_body, collider));
    // screen_left
    let rigid_body = RigidBodyBuilder::new_static().translation(639.5, 0.0);
    let collider = ColliderBuilder::cuboid(1.0, 360.0)
        .friction(0.0)
        .restitution(1.0)
        .collision_groups(wall_interaction_groups);
    commands.spawn((rigid_body, collider));
    // screen right
    let rigid_body = RigidBodyBuilder::new_static().translation(1920.5, 0.0);
    let collider = ColliderBuilder::cuboid(1.0, 360.0)
        .friction(0.0)
        .restitution(1.0)
        .collision_groups(wall_interaction_groups);
    commands.spawn((rigid_body, collider));
}

fn path_builder_usage(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
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
        translation: Vec3::new(WINDOW_WIDTH.mul_add(2.0, -250.0), 250.0, 0.0),
        scale: Vec3::new(3.0, -3.0, 3.0),
        ..Transform::default()
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(GeometryBuilder::build_as(
            &path,
            fill_material,
            fill_mode,
            transform,
        ))
        .spawn(GeometryBuilder::build_as(
            &path,
            stroke_material,
            stroke_mode,
            transform,
        ));
}

fn show_multishape(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    const SIDE_LENGTH: shapes::RegularPolygonFeature =
        shapes::RegularPolygonFeature::SideLength(200.0);
    const PI_2: f32 = 2.0 * std::f32::consts::PI;

    let house_walls = shapes::RegularPolygon {
        sides: 4,
        feature: SIDE_LENGTH,
        ..shapes::RegularPolygon::default()
    };
    let roof = shapes::RegularPolygon {
        sides: 3,
        center: Vec2::new(0.0, 158.0),
        feature: SIDE_LENGTH,
    };
    let mut person = PathBuilder::new();
    // head
    person.arc(Vec2::new(300.0, 100.0), Vec2::new(20.0, 20.0), PI_2, 0.0);
    // torso
    person.move_to(Vec2::new(300.0, 80.0));
    person.line_to(Vec2::new(300.0, 0.0));
    // arms
    person.move_to(Vec2::new(300.0, 70.0));
    person.line_to(Vec2::new(270.0, 60.0));
    person.move_to(Vec2::new(300.0, 70.0));
    person.line_to(Vec2::new(330.0, 60.0));
    // legs
    person.move_to(Vec2::new(300.0, 0.0));
    person.line_to(Vec2::new(280.0, -50.0));
    person.move_to(Vec2::new(300.0, 0.0));
    person.line_to(Vec2::new(320.0, -50.0));
    let person = person.build();

    let mut geometry = GeometryBuilder::new();
    geometry.add(&house_walls).add(&roof).add(&person);

    commands
        .spawn(geometry.build(
            materials.add(ColorMaterial::color(Color::FUCHSIA)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(3.0)),
            Transform {
                translation: Vec3::new(4.0 * WINDOW_WIDTH, 0.0, 0.0),
                ..Transform::default()
            },
        ))
        .with(MultishapeTag);
}
