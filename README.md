# Bevy + Lyon = ❤

`bevy_prototype_lyon` enables [**Bevy**](https://bevyengine.org) users to draw 2D shapes and paths, like triangles, circles, rectangles, lines, arcs and beziers.

## How does it work?

Currently Bevy does not support drawing custom shapes in an easy way. This crate uses Bevy's [`SpriteBundle`](https://docs.rs/bevy/0.4.0/bevy/prelude/struct.SpriteBundle.html) bundle and replaces its default quad mesh with a custom mesh.

Here the [**lyon**](https://docs.rs/lyon/0.16.2/lyon/) crate is used to generate that custom mesh.

### Changelog

#### `master` branch
- New API based on `trait ShapeSprite` instead of the old `primitive` function.

#### 0.1.5
- updated dependency to `lyon_tessellation v0.17`
- with `lyon_tessellation v0.17`, unfortunately rectangles with rounded borders are no longer supported.
- `Quad`, `Triangle` and `Polyline` have been substituted by a general-purpose `Polygon` shape.

#### 0.1.4
- adds support to `bevy 0.4.0`.

#### 0.1.3
- adds support to `bevy 0.3.0`.

## Usage

Add the following line in your `cargo.toml` manifest file, under the `[dependencies]` section:

```TOML
bevy_prototype_lyon = "0.1.5"
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
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());

    commands.spawn(Camera2dBundle::default()).spawn(primitive(
        material.clone(),
        &mut meshes,
        ShapeType::Circle(60.0),
        TessellationMode::Fill(&FillOptions::default()),
        Vec3::new(0.0, 0.0, 0.0).into(),
    ));
}
```

Don't forget to check out the [examples](examples/) to learn more!
