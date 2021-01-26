# Bevy + Lyon = ❤

`bevy_prototype_lyon` enables [**Bevy**](https://bevyengine.org) users to draw 2D shapes and paths, like triangles, circles, rectangles, lines, arcs and beziers.

![Regular polygon demo](docs/polygon_demo.webp)

## How does it work?

Currently Bevy does not support drawing custom shapes in an easy way. This crate uses a variation of Bevy's `SpriteBundle` with custom meshes to draw shapes. The [**lyon**](https://docs.rs/lyon_tessellation) crate is used to generate those custom mesh.

### Changelog

#### 0.2.0
- Complete API reworking
- Regular polygon support
- Extensible shape system through `Geometry` trait

#### 0.1.5
- updated dependency to `lyon_tessellation v0.17`
- with `lyon_tessellation v0.17`, unfortunately rectangles with rounded borders are no longer supported.
- `Quad`, `Triangle` and `Polyline` have been substituted by a general-purpose `Polygon` shape.

## Usage

Add the following line in your `cargo.toml` manifest file, under the `[dependencies]` section:

```TOML
bevy_prototype_lyon = "0.2"
```

Then, you can start by drawing simple shapes:

```rust
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let circle = shapes::Circle {
        radius: 100.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn(Camera2dBundle::default())
        .spawn(GeometryBuilder::build_as(
            &circle,
            materials.add(ColorMaterial::color(Color::AQUAMARINE)),
            TessellationMode::Fill(FillOptions::default()),
            Transform::default(),
        ));
}
```

Don't forget to check out the [examples](examples/) to learn more!
