use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        //Added msaa to reduce aliasing
        .add_resource(Msaa{samples:8})
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup_system.system())
        .run();
}
struct Name(String);
struct BlacksmithMarker;
struct HouseOfPrayerMarker;
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
        transform:Transform::from_translation(Vec3::new(-50.,0.,0.)),
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
                svg_path_string:"m 210.49052,219.61666 c -54.97575,-3.12045 -153.83891,-43.5046 -181.900067,-79.34483 41.944976,3.29834 143.100787,1.42313 185.138697,1.61897 l 6e-5,-0.003 c 41.78023,-0.87477 200.563,-0.4537 261.24529,0 0.085,7.05106 0.79737,22.71244 1.07386,32.86306 -42.04814,8.31883 -101.90702,24.33338 -128.45794,63.97855 -10.53308,31.59203 39.6912,45.827 74.62215,55.19132 1.14898,12.80889 2.62233,32.62936 2.46309,44.71853 -75.4682,-0.86499 -141.64601,-1.07063 -209.86695,-1.35786 -10.81491,-1.77566 -6.66734,-23.1495 -4.31819,-32.38456 5.44628,-16.65332 38.03788,-18.20507 28.06768,-83.12367 -7.29786,-2.58188 -23.92259,-1.83114 -28.06768,-2.15756".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(4.)),
            Transform::default()
        ))
        .with(GlobalTransform::default())
        .spawn(GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"m 213.72921,141.88787 -4e-5,80.1576".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(2.5)),
            Transform::default()
        ))
        .with(GlobalTransform::default());
    });
    commands.spawn(BuildingBundle {
        name: Name("House Of Prayer".to_owned()),
        transform:Transform::from_translation(Vec3::new(375.,0.,0.)),
        global_transform:GlobalTransform::default()
    })
    .with(HouseOfPrayerMarker)
    //we split our art in this example to two children because our art is made out of 2 paths, 
    //one path who's width is 4, 
    //and another whose width is 2.5
    //the art style was approximated from https://www.kenney.nl/assets/cartography-pack 
    .with_children(|parent|{
        let svg_doc_size = Vec2::new(512.,512.);
        parent.spawn(
            GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"m 451.08823,183.34138 v 1.23527 l 32.93058,-1.23527 C 449.61188,131.857 418.92615,69.387224 385.22702,18.688274 347.37606,79.500094 318.96905,124.78068 286.43521,183.34138 h 32.93062 m 131.7224,1.23527 c -1.235,97.55653 -0.82797,200.67401 2.2e-4,295.14045 M 319.36583,216.27223 c -1.37219,-1.37232 -4.80245,-1.37232 -10.29081,0 -97.52186,34.53072 -190.38406,59.53117 -286.084619,98.79176 7.134941,12.0746 12.760607,17.83761 16.876939,17.2887 4.116305,-0.55022 7.546573,2.74437 10.290808,9.87915 l 2.469641,-1.64596 c 1.097791,0.28059 2.19551,2.74437 3.293049,7.40919 2.351405,48.53927 -2.559249,92.5245 0,131.72262 M 319.36569,216.27282 c -1.37219,-17.01407 -1.37219,-27.9909 0,-32.93059".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(4.)),
            Transform::default()
        ))
        .with(GlobalTransform::default())
        .spawn(GeometryBuilder::build_as(
            &shapes::SvgPathShape{
                svg_path_string:"M 319.36583,183.34138 C 343.33899,145.4125 364.63234,109.93103 382.3456,71.788704 l 2.8813,-3.70451 c 23.4704,40.721146 42.63396,73.872316 61.74487,110.729226 l 4.11637,4.52796 m -1.235,98.7918 -13.17225,-0.41051 -18.52347,0.41051 c 2.74409,5.48846 3.15591,9.60474 1.235,12.34888 -1.64632,3.0181 -2.05801,9.87915 -1.235,20.58157 l 10.29087,-1.64596 22.63976,1.64596 m -65.86103,-98.7918 13.99549,-11.93711 6.58613,-9.46787 12.34896,-11.52547 -13.17221,-11.93747 -8.23267,-8.2326 c -11.81354,-15.42301 -12.23497,-12.07317 -25.10955,0.41051 l -6.58613,6.99751 -12.76064,12.76078 c 3.29309,6.31174 7.13498,10.70256 11.52572,13.17216 l 8.6443,7.40955 12.7606,12.34874 m -332.599069,124.31311 4.527956,-3.29292 12.348983,-2.47058 15.230396,-4.11655 15.642024,-6.17424 16.46532,-3.29327 15.23039,-5.35133 15.2304,-8.64424 15.64203,-6.58587 14.40714,-5.35123 15.64206,-4.11655 18.52345,-4.11628 c 6.03729,-1.6461 10.83966,-3.84255 14.40714,-6.58592 3.8419,-2.47059 8.50709,-3.70487 13.99552,-3.70487 5.48844,0 10.70244,-1.64609 15.64203,-4.93973 18.92671,-12.50988 42.50014,-13.60179 63.80304,-22.63953 l 3.70469,-21.40502 c 0.27378,-3.56737 -0.96054,-7.40929 -3.70469,-11.52556 m 0,32.93058 c 4.67089,22.68942 -0.40415,42.79932 3.2e-4,65.86131 l 18.93508,-0.41051 13.99552,0.41051 -2.0583,20.58134 c -0.82328,2.19668 -0.13687,6.31174 2.0583,12.34911 l -19.34677,1.23532 -13.58383,-1.23532 c 1.07316,49.43391 -1.86756,86.9026 9e-5,131.7223 m 0,-164.65275 c -0.5489,3.56741 -0.68603,6.99778 -0.41164,10.29079 l 0.41164,22.63966 m 32.93057,131.7223 c 0.24585,-33.44016 -0.087,-69.45583 0,-98.7918 21.47192,0.45467 46.52051,0.0979 65.86119,0 0.78263,36.13255 0.48729,71.43814 0,98.7918 m -288.14271,-124.72411 -8.23267,9.46751 2.46964,18.11175 -0.41164,12.34874 -0.82328,17.2887 -1.23497,16.46543 c -0.82329,4.66519 -0.82329,10.7022 0,18.11171 l 18.11185,-0.41051 14.81876,0.41051 0.82329,-20.58171 -2.0583,-14.8187 c -1.64632,-5.4885 -2.05797,-10.83969 -1.23497,-16.05379 l 3.29305,-14.81875 c 1.09779,-4.66518 0.82328,-10.01615 -0.82328,-16.05338 l -7.82101,-9.05624 -8.6443,-7.40919 -8.23264,6.99792 m 123.48975,9.46751 c -1.37219,7.68356 -1.23497,13.03488 0.41164,16.05338 1.92076,3.29292 1.92076,8.36974 0,15.23039 -1.64632,6.86064 -1.64632,12.76087 0,17.70047 1.64632,4.93955 1.78354,8.9186 0.41164,11.93711 -1.37219,3.293 -1.64663,10.42806 -0.82328,21.40498 l -13.58386,1.23531 -19.34674,-1.23531 -2.46963,-16.87702 c -0.82328,-5.76287 -0.41164,-11.93715 1.23497,-18.52339 l 2.0583,-16.46543 -0.41164,-10.70219 -0.41163,-19.7583 c 5.21396,-4.11632 8.36981,-7.68405 9.46752,-10.70256 1.37219,-2.74436 3.7048,-4.66519 6.99772,-5.76287 5.48844,0.82476 8.2327,3.29323 8.2327,7.40919 0.27378,4.39065 3.01853,7.40956 8.23265,9.05624 m -197.583659,49.39588 11.525746,0.82476 21.404872,-0.82476 c 2.744077,5.21396 3.43034,9.33024 2.058297,12.34874 -1.372192,3.01828 -2.058297,9.87929 -2.058297,20.58171 l -13.995486,0.41051 -18.935132,-0.41051".to_owned(),
                svg_doc_size_in_px:svg_doc_size.to_owned()
            },

            materials.add(ColorMaterial::color(Color::BLACK)),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(2.5)),
            Transform::default()
        ))
        .with(GlobalTransform::default());
    });
}
