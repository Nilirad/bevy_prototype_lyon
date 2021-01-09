use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup.system())
        .run();
}

fn startup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material = materials.add(ColorMaterial::color(Color::CRIMSON));

    let shape = RectangleShape {
        width: 200.0,
        height: 150.0,
        origin: RectangleOrigin::Center,
    };

    commands.spawn(Camera2dBundle::default()).spawn(shape.draw(
        material,
        TessellationMode::Fill(FillOptions::default()),
        Transform {
            translation: Vec3::new(800.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}
