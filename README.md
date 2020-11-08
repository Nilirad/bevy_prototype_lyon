# Bevy + Lyon = ❤

`bevy_prototype_lyon` enables [**Bevy**](https://bevyengine.org) users to draw 2D shapes and paths, like triangles, circles, rectangles, lines, arcs and beziers.

## How does it work?

Currently Bevy does not support drawing custom shapes in an easy way. This crate uses Bevy's [`SpriteComponents`](https://docs.rs/bevy/0.1.3/bevy/prelude/struct.SpriteComponents.html) bundle and replaces its default quad mesh with a custom mesh.

Here the [**lyon**](https://docs.rs/lyon/0.16.0/lyon/) crate is used to generate that custom mesh.

### What's new in 0.1.3

This version of `bevy_prototype_lyon` adds support to `bevy 0.3.0`.

## Usage

Add the following line in your `cargo.toml` manifest file, under the `[dependencies]` section:

```TOML
bevy_prototype_lyon = "0.1.3"
```

Then, you can start by drawing simple shapes:

```rust
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());

    commands
        .spawn(Camera2dComponents::default())
        .spawn(primitive(
            material.clone(),
            &mut meshes,
            ShapeType::Circle(60.0),
            TessellationMode::Fill(&FillOptions::default()),
            Vec3::new(0.0, 0.0, 0.0).into(),
        ));
}
```

Don't forget to try out the examples to learn more!
