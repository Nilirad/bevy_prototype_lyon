use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Let's create some color materials to give some life to everything.
    let red = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
    let green = materials.add(Color::rgb(0.3, 0.4, 0.3).into());
    let blue = materials.add(Color::rgb(0.1, 0.4, 0.5).into());

    // Now, let's create the shapes and put them into the ECS world. Here we use the
    // `basic_shapes::primitive` function, that returns a `SpriteComponents`, which
    // is very good even for drawing any kind of flat mesh.
    commands
        .spawn(Camera2dComponents::default())
        // Fill Circle
        .spawn(primitive(
            red,
            &mut meshes,
            ShapeType::Circle(60.0),
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(200.0, 0.0, 0.0).into(),
        ))
        // Stroke Circle
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::Circle(40.0),
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_width(8.0)
            ),
            Vec3::new(-150.0, 100.0, 0.0).into(),
        ))
        // Stroke Triangle
        .spawn(primitive(
            blue,
            &mut meshes,
            ShapeType::Triangle(
                (0.0, 0.0).into(),
                (-250.0, 100.0).into(),
                (-100.0, -100.0).into(),
            ),
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_width(5.0)
                .with_line_join(LineJoin::Round)
                .with_line_cap(LineCap::Round)
            ),
            Vec3::new(-100.0, -50.0, 0.0).into(),
        ))
        // Fill Quad
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::Quad(
                (-60.0, 30.0).into(),
                (-120.0, -30.0).into(),
                (60.0, -30.0).into(),
                (30.0, 60.0).into(),
            ),
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(250.0, 200.0, 0.0).into(),
        ))
        // Stroke Quad
        .spawn(primitive(
            blue,
            &mut meshes,
            ShapeType::Quad(
                (-50.0, 50.0).into(),
                (-50.0, -50.0).into(),
                (50.0, -50.0).into(),
                (50.0, 50.0).into(),
            ),
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(3.0)),
            Vec3::new(0.0, 0.0, 0.0).into(),
        ))
        // Fill Rectangle
        .spawn(primitive(
            blue,
            &mut meshes,
            ShapeType::Rectangle { width: 200.0, height: 125.0},
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(-500.0, -300.0, 0.0).into(),
        ))
        // Fill Rounded Rectangle
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::RoundedRectangle { width: 50.0, height: 70.0, border_radius: 15.0 },
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(200.0, -150.0, 0.0).into(),
        ))
        // Stroke Ellipse
        .spawn(primitive(
            blue,
            &mut meshes,
            ShapeType::Ellipse { radius_x: 150.0, radius_y: 50.0 },
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_width(30.0)
            ),
            Vec3::new(0.0, -150.0, 0.0).into(),
        ))
        // Stroke Quad
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::Quad(
                (0.0,0.0).into(),
                (60.0,-30.0).into(),
                (0.0,60.0).into(),
                (-60.0,-30.0).into(),
            ),
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_line_width(5.0)
            ),
            Vec3::new(300.0, -200.0, 0.0).into(),
        ))
        // Stroke Rectangle
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::Rectangle { width: 320.0, height: 180.0 },
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_width(3.0)
            ),
            Vec3::new(-500.0, 150.0, 0.0).into(),
        ))
        .spawn(primitive(
            red,
            &mut meshes,
            ShapeType::RoundedRectangle {
                width: 150.0 * 1.618,
                height: 150.0,
                border_radius: 15.0,
            },
            TessellationMode::Stroke(&StrokeOptions::default()
                .with_line_width(3.0),
            ),
            Vec3::new(-600.0, -100.0, 0.0).into(),
        ))
        // Stroke Polyline
        .spawn(primitive(
            red,
            &mut meshes,
            ShapeType::Polyline {
                points: vec![
                    (0.0, 0.0).into(),
                    (120.0, 30.0).into(),
                    (180.0, -30.0).into(),
                    (150.0, 90.0).into(),
                ],
                closed: false,
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(5.0)),
            Vec3::new(400.0, 0.0, 0.0).into(),
        ))
        // Fill Polyline
        .spawn(primitive(
            green,
            &mut meshes,
            ShapeType::Polyline {
                points: vec![
                    (0.0, 0.0).into(),
                    (30.0, -60.0).into(),
                    (-60.0, -120.0).into(),
                    (60.0, -90.0).into(),
                    (120.0, -150.0).into(),
                    (90.0, -60.0).into(),
                    (150.0, -60.0).into(),
                ],
                closed: false, // required by enum variant, but it is ignored by tessellator
            },
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(-100.0, 300.0, 0.0).into(),
        ))
        // END
        ;
}
