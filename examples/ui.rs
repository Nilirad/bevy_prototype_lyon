//! Shows how to use shapes for the UI camera.

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{RegularPolygon, RegularPolygonFeature},
};

use lyon_tessellation::path::{path::Builder, Path};

// TODO: Death animation

// TODO: Dedicated system for timers?
// TODO: Refactor keeping in mind character states?

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Playing,
    GameOver,
}

struct HealthChangeEvent {
    from: f32,
    to: f32,
}

// Credits: https://commons.wikimedia.org/wiki/File:Octicons-heart.svg
const SVG_HEART: &str = "M19.2 43.2c19.95-15.7 19.2-21.25 19.2-25.6s-3.6-9.6-9.6-9.6s-9.6 6.4-9.6 6.4s-3.6-6.4-9.6-6.4s-9.6 5.25-9.6 9.6s-.75 9.9 19.2 25.6z";

const MAX_HEALTH: f32 = 10.0;
const MAX_LIVES: u32 = 3;
const SPAWN_X_POSITION: f32 = -300.0;
const CHARACHTER_TICK_X_DISPLACEMENT: f32 = 5.0;
const DAMAGE_TRESHOLD_X_POSITION: f32 = -100.0;
const DAMAGE_AMOUNT: f32 = 3.0;
const DAMAGE_ANIMATION_SECS: f32 = 0.4;
const CHARACTER_DAMAGE_ANGLE: f32 = -std::f32::consts::PI / 16.0;
const CHARACTER_DEAD_ANGLE: f32 = -std::f32::consts::FRAC_PI_2;
const HEALTH_CHANGE_ANIMATION_SECS: f32 = 0.25;
const LIFE_LOST_ANIMATION_SECS: f32 = 0.1;
const DEATH_ANIMATION_SECS: f32 = 0.7;
const DAMAGE_COOLDOWN_SECS: f32 = 0.5;
const HEALTH_BAR_WIDTH: f32 = 300.0;

struct Character;
struct Health(f32);
struct Lives(u32);
struct DamageCooldown {
    never_damaged: bool,
    timer: Timer,
}
struct HealthBar;
struct Heart(u32);
struct Animation {
    timer: Timer,
    initial_value: f32,
    final_value: f32,
}
struct DeathAnimationTimer(Timer);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_state(GameState::Playing)
        .add_event::<HealthChangeEvent>()
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
                .with_system(animate_hp_bar_system.label("animate_hp_bar"))
                .with_system(
                    update_health_bar_system
                        .after("animate_hp_bar")
                        .label("update_health_bar"),
                )
                .with_system(update_hearts_system.after("update_health_bar"))
                .with_system(damage_animation_system)
                .with_system(character_death_animation_system),
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
            width: HEALTH_BAR_WIDTH,
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
            parent
                .spawn_bundle(hp_bar_foreground)
                .insert(HealthBar)
                .insert(Animation {
                    timer: Timer::from_seconds(HEALTH_CHANGE_ANIMATION_SECS, false),
                    initial_value: 0.0,
                    final_value: MAX_HEALTH,
                });
        });

    let mut life_lost_timer = Timer::from_seconds(LIFE_LOST_ANIMATION_SECS, false);
    life_lost_timer.pause();

    commands
        .spawn_bundle(heart1)
        .insert(Heart(1))
        .insert(life_lost_timer.clone());
    commands
        .spawn_bundle(heart2)
        .insert(Heart(2))
        .insert(life_lost_timer.clone());
    commands
        .spawn_bundle(heart3)
        .insert(Heart(3))
        .insert(life_lost_timer.clone());
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

    let character = GeometryBuilder::build_as(
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
        Transform::from_xyz(SPAWN_X_POSITION, 0.0, 0.0),
    );

    let mut damage_animation_timer = Timer::from_seconds(DAMAGE_ANIMATION_SECS, false);
    damage_animation_timer.tick(std::time::Duration::from_secs_f32(DAMAGE_ANIMATION_SECS));

    let mut death_animation_timer = Timer::from_seconds(DEATH_ANIMATION_SECS, false);
    death_animation_timer.pause();

    commands.spawn_bundle(lava_pool);
    commands
        .spawn_bundle(character)
        .insert(Character)
        .insert(Health(MAX_HEALTH))
        .insert(Lives(MAX_LIVES))
        .insert(DamageCooldown {
            never_damaged: true,
            timer: Timer::from_seconds(DAMAGE_COOLDOWN_SECS, false),
        })
        .insert(Animation {
            timer: damage_animation_timer,
            initial_value: CHARACTER_DAMAGE_ANGLE,
            final_value: 0.0,
        })
        .insert(DeathAnimationTimer(death_animation_timer));
}

fn move_player_system(mut query: Query<&mut Transform, With<Character>>) {
    let mut transform = query.single_mut().unwrap();
    transform.translation.x = (transform.translation.x + CHARACHTER_TICK_X_DISPLACEMENT).min(0.0);
}

fn damage_player_system(
    mut query: Query<(&mut Health, &mut DamageCooldown, &Transform), With<Character>>,
    time: Res<Time>,
    mut health_change_event_writer: EventWriter<HealthChangeEvent>,
) {
    let (mut health, mut damage_cooldown, transform) = query.single_mut().unwrap();
    let pos_x = transform.translation.x;

    if !damage_cooldown.never_damaged {
        damage_cooldown.timer.tick(time.delta());
    }

    // TODO: Refactor. See in particular if you can simply use timer without the bool.
    if pos_x > DAMAGE_TRESHOLD_X_POSITION {
        if damage_cooldown.never_damaged {
            damage_cooldown.never_damaged = false;
            let initial_health = health.0;
            let damage = calculate_damage(health.0, DAMAGE_AMOUNT);
            health.0 -= damage;
            health_change_event_writer.send(HealthChangeEvent {
                from: initial_health,
                to: health.0,
            });
        } else if damage_cooldown.timer.finished() {
            let initial_health = health.0;
            let damage = calculate_damage(health.0, DAMAGE_AMOUNT);
            health.0 -= damage;
            health_change_event_writer.send(HealthChangeEvent {
                from: initial_health,
                to: health.0,
            });
            damage_cooldown.timer.reset();
        }
    }
}

fn calculate_damage(cur_health: f32, desired_damage: f32) -> f32 {
    let new_health = (cur_health - desired_damage).max(0.0);
    cur_health - new_health
}

fn update_health_bar_system(mut health_bar_query: Query<(&mut Path, &Animation), With<HealthBar>>) {
    let (mut path, animation) = health_bar_query.single_mut().unwrap();

    let animation_progress = animation.timer.percent();
    let animated_health_value = animation.initial_value
        + animation_progress * (animation.final_value - animation.initial_value);

    let mut b = Builder::new();
    let newrect = shapes::Rectangle {
        width: HEALTH_BAR_WIDTH * animated_health_value / MAX_HEALTH,
        height: 40.0,
        origin: shapes::RectangleOrigin::TopLeft,
    };
    newrect.add_geometry(&mut b);

    *path = b.build();
}

fn update_hearts_system(
    mut hearts_query: Query<(&mut ShapeColors, &mut DrawMode, &Heart, &mut Timer)>,
    player_query: Query<&Lives, With<Character>>,
    time: Res<Time>,
) {
    let player_lives = player_query.single().unwrap().0;
    for (mut colors, mut draw_mode, heart, mut timer) in hearts_query.iter_mut() {
        timer.tick(time.delta());
        set_heart(
            &mut colors,
            &mut draw_mode,
            &mut timer,
            player_lives >= heart.0,
        );
    }
}

fn set_heart(colors: &mut ShapeColors, draw_mode: &mut DrawMode, timer: &mut Timer, filled: bool) {
    if filled {
        *colors = ShapeColors::outlined(Color::RED, Color::BLACK);
        *draw_mode = DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(5.0),
        }
    } else {
        if timer.paused() {
            timer.unpause();
        }

        if timer.finished() {
            *colors = ShapeColors::new(Color::BLACK);
            *draw_mode = DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            }
        } else {
            *colors = ShapeColors::outlined(Color::WHITE, Color::BLACK);
            *draw_mode = DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            }
        }
    }
}

fn handle_player_death_system(
    mut query: Query<
        (
            &mut Health,
            &mut Lives,
            &mut Transform,
            &mut DamageCooldown,
            &mut DeathAnimationTimer,
        ),
        With<Character>,
    >,
    mut game_state: ResMut<State<GameState>>,
    mut health_change_event_writer: EventWriter<HealthChangeEvent>,
    time: Res<Time>,
) {
    let (mut health, mut lives, mut transform, mut damage_cooldown, mut death_animation_timer) =
        query.single_mut().unwrap();

    if health.0 <= 0.0 {
        death_animation_timer.0.tick(time.delta());
        if death_animation_timer.0.paused() {
            death_animation_timer.0.unpause();
            lives.0 -= 1;
        } else if death_animation_timer.0.finished() {
            if lives.0 > 0 {
                health_change_event_writer.send(HealthChangeEvent {
                    from: 0.0,
                    to: MAX_HEALTH,
                });
                health.0 = MAX_HEALTH;
                transform.translation.x = SPAWN_X_POSITION;
                *damage_cooldown = DamageCooldown {
                    never_damaged: true,
                    timer: Timer::from_seconds(DAMAGE_COOLDOWN_SECS, false),
                };
                death_animation_timer.0.pause();
                death_animation_timer.0.reset();
            } else {
                game_state.set(GameState::GameOver).unwrap();
            }
        }
    }
}

fn animate_hp_bar_system(
    mut health_change_event_reader: EventReader<HealthChangeEvent>,
    mut query: Query<&mut Animation, With<HealthBar>>,
    time: Res<Time>,
) {
    let mut animation = query.single_mut().unwrap();
    animation.timer.tick(time.delta());

    for health_change in health_change_event_reader.iter() {
        if animation.timer.finished() {
            animation.timer.reset();
            animation.initial_value = health_change.from;
        }
        animation.final_value = health_change.to;
    }
}

fn damage_animation_system(
    mut health_change_event_reader: EventReader<HealthChangeEvent>,
    mut query: Query<(&mut Animation, &mut Transform, &mut ShapeColors), With<Character>>,
    time: Res<Time>,
) {
    let (mut animation, mut transform, mut shape_colors) = query.single_mut().unwrap();
    animation.timer.tick(time.delta());
    for health_change in health_change_event_reader.iter() {
        if health_change.to < health_change.from && health_change.to != 0.0 {
            animation.timer.reset();
        }
    }

    transform.rotation = Quat::from_axis_angle(
        Vec3::Z,
        animation.initial_value * animation.timer.percent_left(),
    );

    let red = animation.timer.percent_left();
    let green_blue = animation.timer.percent();

    *shape_colors = ShapeColors::outlined(Color::rgb(red, green_blue, green_blue), Color::BLACK);
}

// TODO: Sometimes death animation doesn't work. Timers are working correctly. May it be
// a change detection issue?
fn character_death_animation_system(
    mut query: Query<(&mut Transform, &mut ShapeColors, &DeathAnimationTimer), With<Character>>,
) {
    let (mut transform, mut shape_colors, death_animation_timer) = query.single_mut().unwrap();
    let animation_progress = death_animation_timer.0.percent();

    if !death_animation_timer.0.paused() {
        transform.rotation =
            Quat::from_axis_angle(Vec3::Z, CHARACTER_DEAD_ANGLE * animation_progress);
        shape_colors.main.set_a(1.0 - animation_progress);
        shape_colors.outline.set_a(1.0 - animation_progress);
    }
}
