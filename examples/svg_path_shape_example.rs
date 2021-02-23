use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_system.system())
        .run();
}
struct Name(String);
struct BlacksmithMarker;
#[derive(Bundle)]
struct BuildingBundle {
    name: Name,
    transform: Transform,
    global_transform: GlobalTransform,
}
fn startup_system(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BuildingBundle {
        name: Name("Blacksmith".to_owned()),
        transform:Transform::default(),
        global_transform:GlobalTransform::default()
    })
    .with(BlacksmithMarker)
    //we split our art in this example to two children because our art is made out of 2 paths, 
    //one path who's width is 4, 
    //and another whose width is 2.5
    //the art style was approximated from https://www.kenney.nl/assets/cartography-pack 
    .with_children(|parent|{
        let svg_doc_size = Vec2::new(512.,512.);
        parent.spawn(
            GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"M 210.49052,225.67758 C 155.51477,222.55713 56.65161,182.17298 28.590453,146.33275 70.535429,149.63109 171.69124,147.75588 213.72915,147.95172 L 213.72921,147.94879 C 255.50944,147.07402 414.29221,147.49509 474.9745,147.94879 475.0595,154.99985 475.77187,170.66123 476.04836,180.81185 434.00022,189.13068 374.14134,205.14523 347.59042,244.7904 337.05734,276.38243 387.28162,290.6174 422.21257,299.98172 423.36155,312.79061 424.8349,332.61108 424.67566,344.70025 349.20746,343.83526 283.02965,343.62962 214.80871,343.34239 203.9938,341.56673 208.14137,320.19289 210.49052,310.95783 215.9368,294.30451 248.5284,292.75276 238.5582,227.83416 231.26034,225.25228 214.63561,226.00302 210.49052,225.6766".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(4.)),
            Transform::default()
        ))
        .with(GlobalTransform::default())
        .spawn(GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"M 213.72921,147.94879 213.72917,228.10639".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(2.5)),
            Transform::default()
        ))
        .with(GlobalTransform::default());
    });
}
