use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::{FRAC_PI_6, PI};

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
    // Just making some colors...
    let red = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
    let blue = materials.add(Color::rgb(0.1, 0.4, 0.5).into());

    // To draw a path, you have to create a `PathBuilder` first. The initial point
    // is set at (0.0, 0.0).
    let mut builder = PathBuilder::new();
    // Using that builder, you can build any shape:
    builder.line_to(point(20.0, 70.0));
    builder.quadratic_bezier_to(point(70.0, 70.0), point(120.0, 20.0));
    builder.cubic_bezier_to(point(130.0, -20.0), point(0.0, -70.0), point(-70.0, -200.0));
    builder.close(); // This draws a line to (0.0, 0.0)

    // Calling `PathBuilder::move_to` will change the initial position, such that
    // calling `PathBuilder::close` will draw a line to the new position
    builder.move_to(point(-200.0, -200.0));
    builder.line_to(point(-450.0, -300.0));
    builder.line_to(point(-450.0, -200.0));
    builder.close(); // This draws a line to (-200.0, -200.0).

    // Finally, let's draw an arc. A line is drawn if the current position is
    // outside the arc.
    builder.move_to(point(-400.0, 300.0));
    builder.arc(
        point(-200.0, 100.0),
        300.0,
        150.0,
        -PI, // use negative angles for a clockwise arc.
        FRAC_PI_6,
    );

    // Calling `PathBuilder::build` will return a `Path` ready to be used to create
    // Bevy entities.
    let path = builder.build();

    commands
        .spawn(Camera2dComponents::default())
        // Let's draw the path by calling `Path::stroke`.
        .spawn(
            path.stroke(
                red,
                &mut meshes,
                Vec3::new(0.0, 0.0, 0.0),
                &StrokeOptions::default()
                    .with_line_width(5.0)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            ),
        )
        // You can also fill the path using `Path::fill`.
        .spawn(path.fill(
            blue,
            &mut meshes,
            Vec3::new(400.0, 0.0, 0.0),
            &FillOptions::default(),
        ));
    // Calling `Path::stroke` or `Path::fill`, returns a `SpriteComponents`
    // bundle, which can be fed into Bevy's ECS system as `Entities`.
}
