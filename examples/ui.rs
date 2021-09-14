//! Shows how to use shapes for the UI camera.

use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{RegularPolygon, RegularPolygonFeature},
};
use lyon_tessellation::path::{path::Builder, Path};

// TODO: Use states to reset player data in initialization or after death?

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum CharacterState {
    Normal,
    Hurt,
    Dying,
    GameOver,
}

struct HealthChangedEvent {
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
const DAMAGE_ANIMATION_TIME: Duration = Duration::from_millis(400);
const CHARACTER_DAMAGE_ANGLE: f32 = -PI / 16.0;
const CHARACTER_DEAD_ANGLE: f32 = -FRAC_PI_2;
const HEALTH_CHANGED_ANIMATION_TIME: Duration = Duration::from_millis(250);
const HEART_LOST_ANIMATION_TIME: Duration = Duration::from_millis(100);
const DEATH_ANIMATION_TIME: Duration = Duration::from_millis(700);
const DAMAGE_COOLDOWN_TIME: Duration = Duration::from_millis(500);
const HEALTH_BAR_WIDTH: f32 = 300.0;

struct Character;
struct Health(f32);
struct Lives(u32);
struct DamageCooldown(Timer);
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
        .add_state(CharacterState::Normal)
        .add_event::<HealthChangedEvent>()
        .add_startup_system(setup_ui_system)
        .add_startup_system(setup_gameplay_system)
        .add_system(tick_timers_system)
        .add_system_set(
            SystemSet::on_update(CharacterState::Normal)
                .with_system(move_character_system.label("move_character"))
                .with_system(damage_character_system.after("move_character")),
        )
        .add_system_set(
            SystemSet::on_update(CharacterState::Hurt)
                .with_system(move_character_system)
                .with_system(damage_animation_system),
        )
        .add_system_set(
            SystemSet::on_update(CharacterState::Dying)
                .with_system(manage_character_death_system.label("manage_character_death"))
                .with_system(character_death_animation_system.after("manage_character_death")),
        )
        .add_system_set(
            SystemSet::new()
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

    commands
        .spawn_bundle(hp_bar_background)
        .with_children(|parent| {
            parent
                .spawn_bundle(hp_bar_foreground)
                .insert(HealthBar)
                .insert(Animation {
                    timer: Timer::new(HEALTH_CHANGED_ANIMATION_TIME, false),
                    initial_value: 0.0,
                    final_value: MAX_HEALTH,
                });
        });

    let mut life_lost_timer = Timer::new(HEART_LOST_ANIMATION_TIME, false);
    life_lost_timer.pause();

    for i in 1..=3 {
        let offset = 50.0 * (i - 1) as f32;
        let heart = GeometryBuilder::build_ui_as(
            &heart_shape,
            ShapeColors::outlined(Color::RED, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(30.0 + offset),
                    right: Val::Auto,
                    top: Val::Px(80.0),
                    bottom: Val::Auto,
                },
                ..Default::default()
            },
        );

        commands
            .spawn_bundle(heart)
            .insert(Heart(i))
            .insert(life_lost_timer.clone());
    }
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

    let mut damage_cooldown_timer = Timer::new(DAMAGE_COOLDOWN_TIME, false);
    damage_cooldown_timer.tick(DAMAGE_COOLDOWN_TIME);

    let mut damage_animation_timer = Timer::new(DAMAGE_ANIMATION_TIME, false);
    damage_animation_timer.tick(DAMAGE_ANIMATION_TIME);

    let mut death_animation_timer = Timer::new(DEATH_ANIMATION_TIME, false);
    death_animation_timer.pause();

    commands.spawn_bundle(lava_pool);
    commands
        .spawn_bundle(character)
        .insert(Character)
        .insert(Health(MAX_HEALTH))
        .insert(Lives(MAX_LIVES))
        .insert(DamageCooldown(damage_cooldown_timer))
        .insert(Animation {
            timer: damage_animation_timer,
            initial_value: CHARACTER_DAMAGE_ANGLE,
            final_value: 0.0,
        })
        .insert(DeathAnimationTimer(death_animation_timer));
}

fn move_character_system(mut query: Query<&mut Transform, With<Character>>) {
    let mut transform = query.single_mut();
    transform.translation.x = (transform.translation.x + CHARACHTER_TICK_X_DISPLACEMENT).min(0.0);
}

fn damage_character_system(
    mut query: Query<(&mut Health, &Transform, &mut DamageCooldown), With<Character>>,
    mut character_state: ResMut<State<CharacterState>>,
    mut health_changed_event_writer: EventWriter<HealthChangedEvent>,
) {
    let (mut health, transform, mut damage_cooldown) = query.single_mut();

    let mut damage_applied = false;

    if transform.translation.x > DAMAGE_TRESHOLD_X_POSITION && damage_cooldown.0.finished() {
        damage_cooldown.0.reset();
        let initial_health = health.0;
        health.0 -= calculate_damage(health.0, DAMAGE_AMOUNT);
        health_changed_event_writer.send(HealthChangedEvent {
            from: initial_health,
            to: health.0,
        });
        damage_applied = true;
    }

    if damage_applied {
        character_state
            .set(if health.0 > 0.0 {
                CharacterState::Hurt
            } else {
                CharacterState::Dying
            })
            .unwrap();
    }
}

fn calculate_damage(cur_health: f32, desired_damage: f32) -> f32 {
    let new_health = (cur_health - desired_damage).max(0.0);
    cur_health - new_health
}

fn update_health_bar_system(
    mut health_bar_query: Query<(&mut Path, &mut Animation), With<HealthBar>>,
    mut health_changed_event_reader: EventReader<HealthChangedEvent>,
) {
    let (mut path, mut animation) = health_bar_query.single_mut();

    for health_changed in health_changed_event_reader.iter() {
        if animation.timer.finished() {
            animation.timer.reset();
            animation.initial_value = health_changed.from;
        }
        animation.final_value = health_changed.to;
    }

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
    character_query: Query<&Lives, With<Character>>,
) {
    let character_lives = character_query.single().0;
    for (mut colors, mut draw_mode, heart, mut timer) in hearts_query.iter_mut() {
        set_heart(
            &mut colors,
            &mut draw_mode,
            &mut timer,
            character_lives >= heart.0,
        );
    }
}

fn set_heart(colors: &mut ShapeColors, draw_mode: &mut DrawMode, timer: &mut Timer, filled: bool) {
    if filled {
        *colors = ShapeColors::outlined(Color::RED, Color::BLACK);
    } else {
        if timer.paused() {
            timer.unpause();
        }

        if timer.finished() {
            *colors = ShapeColors::new(Color::BLACK);
        } else {
            *colors = ShapeColors::outlined(Color::WHITE, Color::BLACK);
        }
    }

    *draw_mode = DrawMode::Outlined {
        fill_options: FillOptions::default(),
        outline_options: StrokeOptions::default().with_line_width(5.0),
    }
}

fn damage_animation_system(
    mut health_changed_event_reader: EventReader<HealthChangedEvent>,
    mut query: Query<(&mut Animation, &mut Transform, &mut ShapeColors), With<Character>>,
    mut character_state: ResMut<State<CharacterState>>,
) {
    let (mut animation, mut transform, mut shape_colors) = query.single_mut();

    for health_changed in health_changed_event_reader.iter() {
        if health_changed.to < health_changed.from && health_changed.to != 0.0 {
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

    if animation.timer.finished() {
        character_state.set(CharacterState::Normal).unwrap();
    }
}

fn manage_character_death_system(
    mut query: Query<
        (
            &mut DeathAnimationTimer,
            &mut Lives,
            &mut Transform,
            &mut Health,
            &mut DamageCooldown,
        ),
        With<Character>,
    >,
    mut character_state: ResMut<State<CharacterState>>,
    mut health_changed_event_writer: EventWriter<HealthChangedEvent>,
) {
    let (mut death_animation_timer, mut lives, mut transform, mut health, mut damage_cooldown) =
        query.single_mut();

    if death_animation_timer.0.paused() {
        death_animation_timer.0.unpause();
    } else if death_animation_timer.0.finished() {
        lives.0 -= 1;
        if lives.0 > 0 {
            transform.translation.x = SPAWN_X_POSITION;
            health_changed_event_writer.send(HealthChangedEvent {
                from: 0.0,
                to: MAX_HEALTH,
            });
            health.0 = MAX_HEALTH;
            transform.translation.x = SPAWN_X_POSITION;
            damage_cooldown.0.tick(DAMAGE_COOLDOWN_TIME);
            death_animation_timer.0.pause();
            death_animation_timer.0.reset();
            character_state.set(CharacterState::Normal).unwrap();
        } else {
            character_state.set(CharacterState::GameOver).unwrap();
        }
    }
}

fn character_death_animation_system(
    mut query: Query<(&mut Transform, &mut ShapeColors, &DeathAnimationTimer), With<Character>>,
) {
    let (mut transform, mut shape_colors, death_animation_timer) = query.single_mut();
    let animation_progress = death_animation_timer.0.percent();

    transform.rotation = Quat::from_axis_angle(Vec3::Z, CHARACTER_DEAD_ANGLE * animation_progress);
    shape_colors.main.set_a(1.0 - animation_progress);
    shape_colors.outline.set_a(1.0 - animation_progress);
}

fn tick_timers_system(
    mut timer_query: Query<&mut Timer>,
    mut damage_cooldown_query: Query<&mut DamageCooldown>,
    mut animation_query: Query<&mut Animation>,
    mut death_animation_timer_query: Query<&mut DeathAnimationTimer>,
    time: Res<Time>,
) {
    let delta = time.delta();

    for mut timer in timer_query.iter_mut() {
        timer.tick(delta);
    }
    for mut damage_cooldown in damage_cooldown_query.iter_mut() {
        damage_cooldown.0.tick(delta);
    }
    for mut animation in animation_query.iter_mut() {
        animation.timer.tick(delta);
    }
    for mut death_animation_timer in death_animation_timer_query.iter_mut() {
        death_animation_timer.0.tick(delta);
    }
}
