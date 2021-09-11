//! Shows how to use shapes for the UI camera.

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{RegularPolygon, RegularPolygonFeature},
};

use lyon_tessellation::path::{path::Builder, Path};

// TODO: Damage animation
// TODO: HP bar animation
// TODO: Heart flash animation

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Playing,
    GameOver,
}

// Credits: https://commons.wikimedia.org/wiki/File:Octicons-heart.svg
const SVG_HEART: &str = "M19.2 43.2c19.95-15.7 19.2-21.25 19.2-25.6s-3.6-9.6-9.6-9.6s-9.6 6.4-9.6 6.4s-3.6-6.4-9.6-6.4s-9.6 5.25-9.6 9.6s-.75 9.9 19.2 25.6z";

struct Player;
struct Health(f32);
struct Lives(u32);
struct DamageCooldown {
    never_damaged: bool,
    timer: Timer,
}
struct HealthBar;
struct Heart(u32);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state(GameState::Playing)
        .add_startup_system(setup_ui_system)
        .add_startup_system(setup_gameplay_system)
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label("gameplay")
                .with_system(move_player_system)
                .with_system(damage_player_system)
                .with_system(handle_player_death_system),
        )
        .add_system_set(
            SystemSet::new()
                .after("gameplay")
                .with_system(update_health_bar_system)
                .with_system(update_hearts_system),
        )
        .run();
}

fn setup_ui_system(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    let hp_bar_background = GeometryBuilder::build_ui_as(
        &shapes::Rectangle {
            width: 310.0,
            height: 50.0,
            origin: shapes::RectangleOrigin::TopLeft,
        },
        ShapeColors::new(Color::BLACK),
        DrawMode::Fill(FillOptions::default()),
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(20.0),
                right: Val::Auto,
                top: Val::Px(20.0),
                bottom: Val::Auto,
            },
            ..Default::default()
        },
    );

    let hp_bar_foreground = GeometryBuilder::build_ui_as(
        &shapes::Rectangle {
            width: 300.0,
            height: 40.0,
            origin: shapes::RectangleOrigin::TopLeft,
        },
        ShapeColors::new(Color::GREEN),
        DrawMode::Fill(FillOptions::default()),
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(5.0),
                right: Val::Auto,
                top: Val::Px(5.0),
                bottom: Val::Auto,
            },
            ..Default::default()
        },
    );

    let heart_shape = shapes::SvgPathShape {
        svg_doc_size_in_px: Vec2::new(0.0, 0.0),
        svg_path_string: SVG_HEART.to_owned(),
    };

    let heart1 = GeometryBuilder::build_ui_as(
        &heart_shape,
        ShapeColors::outlined(Color::RED, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        },
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(30.0),
                right: Val::Auto,
                top: Val::Px(80.0),
                bottom: Val::Auto,
            },
            ..Default::default()
        },
    );

    let heart2 = GeometryBuilder::build_ui_as(
        &heart_shape,
        ShapeColors::outlined(Color::RED, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        },
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(80.0),
                right: Val::Auto,
                top: Val::Px(80.0),
                bottom: Val::Auto,
            },
            ..Default::default()
        },
    );

    let heart3 = GeometryBuilder::build_ui_as(
        &heart_shape,
        ShapeColors::outlined(Color::RED, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        },
        Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(130.0),
                right: Val::Auto,
                top: Val::Px(80.0),
                bottom: Val::Auto,
            },
            ..Default::default()
        },
    );

    commands
        .spawn_bundle(hp_bar_background)
        .with_children(|parent| {
            parent.spawn_bundle(hp_bar_foreground).insert(HealthBar);
        });
    commands.spawn_bundle(heart1).insert(Heart(1));
    commands.spawn_bundle(heart2).insert(Heart(2));
    commands.spawn_bundle(heart3).insert(Heart(3));
}

fn setup_gameplay_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let lava_pool = GeometryBuilder::build_as(
        &shapes::Rectangle {
            width: 200.0,
            height: 200.0,
            ..shapes::Rectangle::default()
        },
        ShapeColors::outlined(Color::ORANGE, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        },
        Transform::default(),
    );

    let player = GeometryBuilder::build_as(
        &RegularPolygon {
            sides: 3,
            feature: RegularPolygonFeature::Radius(30.0),
            ..RegularPolygon::default()
        },
        ShapeColors::outlined(Color::CYAN, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(3.0),
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
    );

    commands.spawn_bundle(lava_pool);
    commands
        .spawn_bundle(player)
        .insert(Player)
        .insert(Health(10.0))
        .insert(Lives(3))
        .insert(DamageCooldown {
            never_damaged: true,
            timer: Timer::from_seconds(0.5, false),
        });
}

fn move_player_system(mut query: Query<&mut Transform, With<Player>>) {
    let mut transform = query.single_mut().unwrap();
    transform.translation.x = (transform.translation.x + 5.0).min(0.0);
}

fn damage_player_system(
    mut query: Query<(&mut Health, &mut DamageCooldown, &Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut health, mut damage_cooldown, transform) = query.single_mut().unwrap();
    let pos_x = transform.translation.x;

    if !damage_cooldown.never_damaged {
        damage_cooldown.timer.tick(time.delta());
    }

    if pos_x > -100.0 {
        if damage_cooldown.never_damaged {
            damage_cooldown.never_damaged = false;
            health.0 = (health.0 - 3.0).max(0.0);
        } else if damage_cooldown.timer.finished() {
            health.0 = (health.0 - 3.0).max(0.0);
            damage_cooldown.timer.reset();
        }
    }
}

fn update_health_bar_system(
    mut health_bar_query: Query<&mut Path, With<HealthBar>>,
    player_query: Query<&Health, With<Player>>,
) {
    let player_health = player_query.single().unwrap().0;

    let mut b = Builder::new();
    let newrect = shapes::Rectangle {
        width: player_health * 30.0,
        height: 40.0,
        origin: shapes::RectangleOrigin::TopLeft,
    };
    newrect.add_geometry(&mut b);

    let mut health_bar = health_bar_query.single_mut().unwrap();
    *health_bar = b.build();
}

fn update_hearts_system(
    mut hearts_query: Query<(&mut ShapeColors, &mut DrawMode, &Heart)>,
    player_query: Query<&Lives, With<Player>>,
) {
    let player_lives = player_query.single().unwrap().0;
    for (mut colors, mut draw_mode, heart) in hearts_query.iter_mut() {
        set_heart(&mut colors, &mut draw_mode, player_lives >= heart.0);
    }
}

fn set_heart(colors: &mut ShapeColors, draw_mode: &mut DrawMode, filled: bool) {
    if filled {
        *colors = ShapeColors::outlined(Color::RED, Color::BLACK);
        *draw_mode = DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        }
    } else {
        *colors = ShapeColors::new(Color::BLACK);
        *draw_mode = DrawMode::Fill(FillOptions::default());
    }
}

fn handle_player_death_system(
    mut query: Query<(&mut Health, &mut Lives, &mut Transform, &mut DamageCooldown), With<Player>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (mut health, mut lives, mut transform, mut damage_cooldown) = query.single_mut().unwrap();

    if health.0 <= 0.0 {
        lives.0 -= 1;
        if lives.0 > 0 {
            health.0 = 10.0;
            transform.translation.x = -300.0;
            *damage_cooldown = DamageCooldown {
                never_damaged: true,
                timer: Timer::from_seconds(0.5, false),
            };
        } else {
            game_state.set(GameState::GameOver).unwrap();
        }
    }
}
