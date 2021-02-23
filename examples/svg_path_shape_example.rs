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
        parent.spawn(
            GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"m 0.0,0.0 c -13.112819,-0.74429 -36.693655,-10.37672 -43.386799,-18.92534 10.004713,0.78673 34.132393,0.33946 44.159274,0.38617 l 1.4e-5,-7.1e-4 c 9.965431,-0.20865 47.838295,-0.10822 62.312225,0 0.0203,1.68182 0.1902,5.41738 0.25613,7.83851 -10.02932,1.98419 -24.306866,5.80399 -30.639796,15.26016 -2.51234,7.53532 9.46716,10.93064 17.798896,13.16422 0.27404,3.05518 0.62549,7.78276 0.58749,10.66626 -18.000676,-0.20632 -33.785405,-0.25535 -50.057463,-0.32388 -2.579571,-0.42352 -1.590285,-5.52161 -1.029971,-7.72435 1.299045,-3.97216 9.07279,-4.34227 6.694701,-19.82666 -1.740686,-0.61583 -5.706016,-0.43676 -6.694701,-0.51462".to_owned()
                //               ^make sure that if you have an M/m componet like here it is set to 0.0, 0.0
                //this for example is the original path from the SVG file:
                //svg_path_string:"M 55.294594,65.720739 C 40.87105,64.902051 14.933126,54.306787 7.5709476,44.903676 18.575711,45.769037 45.115141,45.277053 56.144285,45.328434 l 1.7e-5,-7.7e-4 c 10.961541,-0.229507 52.620088,-0.119034 68.540808,0 0.0223,1.849929 0.2092,5.958879 0.28174,8.622016 -11.03183,2.182544 -26.736518,6.384152 -33.702468,16.785533 -2.76348,8.288546 10.413458,12.02326 19.578008,14.480099 0.30145,3.360566 0.688,8.560701 0.64622,11.732438 C 91.688632,96.720811 74.326097,96.666859 56.42752,96.591499 53.590101,96.125634 54.678265,90.517953 55.294594,88.095027 56.72349,83.725828 65.274282,83.31871 62.658483,66.286543 60.743802,65.609155 56.382106,65.80612 55.294594,65.720482".to_owned()
                //this will be offset by 55.294594,65.720739 from it's transform's translation, and you don't want that
                //and you can try replacing them to see the result
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(4.)),
            Transform::default()
        ))
        .with(GlobalTransform::default())
        .spawn(GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"m 0.849708,-20.393075 -1e-5,21.0303".to_owned()
                //^make sure that if your art is made of multiple paths their origins are relative to each other
                //this was the original path:
                //svg_path_string:"m 56.144302,45.327664 -1e-5,21.0303".to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(2.5)),
            Transform::default()
        ))
        .with(GlobalTransform::default());
    });
}
