use bevy::prelude::*;

const NUM_SLIDES: isize = 6;
const WINDOW_WIDTH: f32 = 1280.0;

/// The current viewed slide.
pub struct Page(pub isize);
/// Tags the main camera.
struct WorldCamera;

/// Component for smooth camera movement.
#[derive(Debug, Default)]
struct CameraTarget(f32);

pub struct DemoCameraPlugin;

impl Plugin for DemoCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Page(0))
            .add_startup_system(camera_setup.system())
            .add_system(camera_controls.system())
            .add_system(move_camera.system());
    }
}

fn camera_setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with(WorldCamera)
        .with(CameraTarget::default());
}

fn camera_controls(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut CameraTarget, With<WorldCamera>>,
    mut slide: ResMut<Page>,
) {
    let mut movement = 0;
    if input.just_pressed(KeyCode::D) {
        movement += 1;
    }
    if input.just_pressed(KeyCode::A) {
        movement -= 1;
    }
    if movement != 0 {
        slide.0 += movement;
        slide.0 = slide.0.max(0).min(NUM_SLIDES - 1);

        for mut target in query.iter_mut() {
            target.0 = slide.0 as f32 * WINDOW_WIDTH;
        }
    }
}

fn move_camera(
    mut query: Query<(&mut Transform, &CameraTarget), With<WorldCamera>>,
    time: Res<Time>,
) {
    for (mut camera_transform, target) in query.iter_mut() {
        let camera_x = &mut camera_transform.translation.x;

        let remaining_distance = target.0 - *camera_x;
        let displacement = 5.0 * time.delta_seconds() * remaining_distance;
        *camera_x = if remaining_distance.abs() < 0.1 {
            target.0
        } else {
            *camera_x + displacement
        };
    }
}
