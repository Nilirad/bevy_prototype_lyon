//! This is the example that goes to the README.md file. The README.md should be
//! updated before every release.

use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands.spawn(
        ShapeBuilder::with(&shape)
            .fill(DARK_CYAN)
            .stroke((BLACK, 10.0))
            .build(),
    );
}
