use bevy::{color, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(Startup, setup_system)
        .run();
}

// The tolerance for tesselating curves, in the same units as the svg path
const CURVE_TOLERANCE: f32 = 0.01;

fn chat_icon() -> Shape {
    // Equivalent to the "svg viewport"
    let svg_doc_size_in_px = Vec2::new(24., 24.);

    let icon_fill = Fill {
        options: FillOptions::tolerance(CURVE_TOLERANCE),
        color: color::palettes::tailwind::GRAY_50.into(),
    };

    ShapeBuilder::with(&shapes::SvgPathShape {
        svg_path_string: CHAT_CODE.to_owned(),
        svg_doc_size_in_px,
    })
    .fill(icon_fill)
    .build()
}

// A composite shape made of multiple SVG paths
fn planet_icon() -> impl Scene {
    // Equivalent to the "svg viewport"
    let svg_doc_size_in_px = Vec2::new(24., 24.);

    let icon_fill = Fill {
        options: FillOptions::tolerance(CURVE_TOLERANCE),
        color: color::palettes::tailwind::GRAY_50.into(),
    };

    let icon_fill_grey = Fill {
        options: FillOptions::tolerance(CURVE_TOLERANCE),
        color: color::palettes::tailwind::GRAY_400.into(),
    };

    let planet_shape = ShapeBuilder::with(&shapes::SvgPathShape {
        svg_path_string: PLANET.to_owned(),
        svg_doc_size_in_px,
    })
    .fill(icon_fill)
    .build();

    let planet_mid_shape = ShapeBuilder::with(&shapes::SvgPathShape {
        svg_path_string: PLANET_MID.to_owned(),
        svg_doc_size_in_px,
    })
    .fill(icon_fill_grey)
    .build();

    bsn! {
        template_value(planet_shape)
        Children [
            template_value(planet_mid_shape)
        ]
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Sample4));

    let scale = Vec2::splat(10.).extend(1.);

    commands.spawn((
        chat_icon(),
        Transform {
            translation: Vec3::new(-200., 0., 0.),
            scale,
            ..Default::default()
        },
    ));

    commands.spawn_scene(bsn! {
        planet_icon()
        Transform {
            translation: Vec3::new(200., 0., 0.),
            scale,
        }
    });
}

// SVG paths by 480 Design under the CC BY 4.0 license
// https://www.figma.com/community/file/1166831539721848736/solar-icons-set
const CHAT_CODE: &str = "m13.087 21.388l.542-.916c.42-.71.63-1.066.968-1.262c.338-.197.763-.204 1.613-.219c1.256-.021 2.043-.098 2.703-.372a5 5 0 0 0 2.706-2.706C22 14.995 22 13.83 22 11.5v-1c0-3.273 0-4.91-.737-6.112a5 5 0 0 0-1.65-1.651C18.41 2 16.773 2 13.5 2h-3c-3.273 0-4.91 0-6.112.737a5 5 0 0 0-1.651 1.65C2 5.59 2 7.228 2 10.5v1c0 2.33 0 3.495.38 4.413a5 5 0 0 0 2.707 2.706c.66.274 1.447.35 2.703.372c.85.015 1.275.022 1.613.219c.337.196.548.551.968 1.262l.542.916c.483.816 1.69.816 2.174 0M14.97 7.299a.75.75 0 0 1 1.06 0l.209.209c.635.635 1.165 1.165 1.529 1.642c.384.503.654 1.035.654 1.68c0 .644-.27 1.176-.654 1.68c-.364.476-.894 1.006-1.53 1.642l-.208.208a.75.75 0 1 1-1.06-1.06l.171-.172c.682-.682 1.139-1.141 1.434-1.528c.283-.37.347-.586.347-.77s-.064-.4-.347-.77c-.295-.388-.752-.847-1.434-1.529l-.171-.171a.75.75 0 0 1 0-1.06m-.952-1.105a.75.75 0 1 0-1.449-.388l-2.588 9.66a.75.75 0 1 0 1.45.387zM9.03 7.3a.75.75 0 0 1 0 1.06l-.171.172c-.682.682-1.139 1.141-1.434 1.529c-.283.37-.347.585-.347.77c0 .184.064.4.347.77c.295.387.752.846 1.434 1.528l.171.171a.75.75 0 1 1-1.06 1.06l-.172-.17l-.037-.037c-.635-.636-1.165-1.165-1.529-1.643c-.384-.503-.654-1.035-.654-1.68c0-.644.27-1.176.654-1.68c.364-.476.894-1.006 1.53-1.641l.036-.037l.172-.172a.75.75 0 0 1 1.06 0";
const PLANET: &str = "M21.206 15.912a41 41 0 0 0-.711.3l-.01.005c-.487.21-1.045.45-1.654.674c-1.226.454-2.693.86-4.322.86c-1.813 0-3.203-.486-4.317-1.02c-.43-.206-.829-.425-1.18-.617l-.272-.15c-.43-.232-.764-.399-1.062-.493a16.4 16.4 0 0 0-3.59-.677a16 16 0 0 0-1.453-.048l-.077.003h-.021l-.152.008a10.005 10.005 0 0 0 18.821 1.155M3.237 7.179l.297.302l.003.004l.019.018l.086.081c.079.072.2.18.36.31c.32.26.795.61 1.404.96c1.219.704 2.949 1.396 5.03 1.396c1.374 0 2.426-.394 3.318-.86c.355-.186.675-.377.993-.567l.275-.163c.392-.232.81-.468 1.234-.614a15 15 0 0 1 3.391-.743a11 11 0 0 1 1.155-.052A10 10 0 0 0 12 2a10 10 0 0 0-8.763 5.179";
const PLANET_MID: &str = "M21.775 14.118Q22 13.092 22 12a10 10 0 0 0-.525-3.206l-.527-.038h-.011l-.051-.003a10 10 0 0 0-1.096.043a13.4 13.4 0 0 0-3.047.67c-.263.09-.563.252-.958.485l-.248.148c-.322.193-.69.413-1.088.62c-1.03.539-2.323 1.031-4.012 1.031c-2.418 0-4.407-.803-5.78-1.596a12 12 0 0 1-1.6-1.096a9 9 0 0 1-.48-.415a10.1 10.1 0 0 0-.498 4.628l.385-.02h.011l.027-.001a9 9 0 0 1 .45-.006c.303.002.733.014 1.253.055c1.037.08 2.447.277 3.923.742c.45.141.899.373 1.327.605l.299.163c.346.19.697.383 1.087.57c.98.47 2.144.871 3.668.871c1.383 0 2.662-.344 3.802-.766c.571-.21 1.099-.437 1.591-.65l.018-.007c.475-.204.937-.403 1.343-.538z";
