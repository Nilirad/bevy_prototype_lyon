use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .run();
}

fn startup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let rect = Rectangle {
        width: 100.0,
        height: 100.0,
        origin: RectangleOrigin::Center,
    };

    let mut tessellator = Tessellator::new();

    commands
        .spawn(Camera2dBundle::default())
        .spawn(rect.generate_sprite(
            materials.add(ColorMaterial::color(Color::RED)),
            &mut meshes,
            &mut tessellator,
            &TessellationMode::Stroke(&StrokeOptions::default()),
            Transform::default(),
        ));

    commands.spawn(primitive(
        materials.add(ColorMaterial::color(Color::BLUE)),
        &mut meshes,
        ShapeType::Circle(10.0),
        TessellationMode::Fill(&FillOptions::default()),
        Vec3::new(0.0, 0.0, 0.0),
    ));
}
