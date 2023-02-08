use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        //Added msaa to reduce aliasing
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .run();
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct BlacksmithMarker;

#[derive(Component)]
struct ToolShackMarker;

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn((
            Name("Blacksmith".to_owned()),
            BlacksmithMarker,
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
                ..default()
            },
        ))
        //we split our art in this example to two children because our art is made out of 2 paths,
        //one path who's width is 4,
        //and another whose width is 2.5
        //the art style was approximated from https://www.kenney.nl/assets/cartography-pack
        .with_children(|parent| {
            let svg_doc_size = Vec2::new(512., 512.);
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                        svg_path_string: BLACKSMITH_OUTLINE.to_owned(),
                        svg_doc_size_in_px: svg_doc_size.to_owned(),
                    }),
                    ..default()
                },
                Stroke::new(Color::BLACK, 4.0),
            ));
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                        svg_path_string: BLACKSMITH_DETAIL.to_owned(),
                        svg_doc_size_in_px: svg_doc_size.to_owned(),
                    }),
                    ..default()
                },
                Stroke::new(Color::BLACK, 2.5),
            ));
        });

    commands
        .spawn((
            Name("Shack".to_owned()),
            ToolShackMarker,
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(375., 0., 0.),
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..Default::default()
                },
                ..default()
            },
        ))
        //we split our art in this example to two children because our art is made out of 2 paths,
        //one path who's width is 4,
        //and another whose width is 2.5
        //the art style was approximated from https://www.kenney.nl/assets/cartography-pack
        .with_children(|parent| {
            let svg_doc_size = Vec2::new(1000., 1000.);
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                        svg_path_string: SHACK.to_owned(),
                        svg_doc_size_in_px: svg_doc_size.to_owned(),
                    }),
                    ..default()
                },
                Stroke::new(Color::BLACK, 20.0),
            ));

            // shack walls
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                        svg_path_string: SHACK_WALLS.to_owned(),
                        svg_doc_size_in_px: svg_doc_size.to_owned(),
                    }),
                    ..default()
                },
                Stroke::new(Color::BLACK, 17.5),
            ));
        });
}

const BLACKSMITH_OUTLINE: &str = "m
210.49052,219.61666
c
-54.97575,-3.12045
-153.83891,-43.5046
-181.900067,-79.34483
41.944976,3.29834
143.100787,1.42313
185.138697,1.61897
l
6e-5,-0.003
c
41.78023,-0.87477
200.563,-0.4537
261.24529,0
0.085,7.05106
0.79737,22.71244
1.07386,32.86306
-42.04814,8.31883
-101.90702,24.33338
-128.45794,63.97855
-10.53308,31.59203
39.6912,45.827
74.62215,55.19132
1.14898,12.80889
2.62233,32.62936
2.46309,44.71853
-75.4682,-0.86499
-141.64601,-1.07063
-209.86695,-1.35786
-10.81491,-1.77566
-6.66734,-23.1495
-4.31819,-32.38456
5.44628,-16.65332
38.03788,-18.20507
28.06768,-83.12367
-7.29786,-2.58188
-23.92259,-1.83114
-28.06768,-2.15756";

const BLACKSMITH_DETAIL: &str = "m 213.72921,141.88787 -4e-5,80.1576";

const SHACK: &str = "m
254.47507,533.90714
28.03554,-31.1502
29.07393,-32.18938
30.11225,-26.99742
29.07391,-30.11185
28.03556,-34.26547
29.07391,-25.95885
28.03556,-29.0741
q
13.49859,-16.61388
21.80543,-21.80524
l
25.95885,-17.65243
q
20.76708,9.34498
26.9972,26.99742
6.2297,18.68994
25.95885,35.30382
l
34.26568,29.07411
31.15062,24.9205
26.9972,23.88213
24.92049,29.07412
28.03556,37.38075
q
12.46024,18.69016
22.84378,21.80522
11.4219,4.15218
28.03556,20.76687
m
-332.27326,332.27305
2.07692,-44.64881
v
-40.496
l
-6.23054,-39.45766
-3.11527,-42.57209
1.03835,-35.30383
6.23054,-46.72655
44.64922,-3.1161
38.4191,1.03627
30.11226,-1.03627
52.95605,3.1161
q
5.19218,20.76749
-2.0767,43.61128
-6.22972,22.84357
1.03835,41.53437
7.26806,18.68995
3.11527,39.45682
l
-6.23054,46.72656
q
-1.03836,25.95884
1.03835,35.30381
l
3.11527,42.5721
m
164.05971,-83.0681
-33.22711,-1.03629
-47.76428,1.03629
-4.15362,-32.18855
4.15362,-50.87956
34.26567,1.03628
48.80264,-1.03628
m
-498.40988,-83.06873
30.11226,4.15217
52.95606,-4.15217
3.11505,33.22774
q
-3.11505,11.42189
-3.11505,49.84099
l
-28.03557,1.03628
-55.03275,-1.03628";

const SHACK_WALLS: &str = "m
254.47507,866.18019
q
18.69037,-88.25945
8.30683,-113.17996
-9.34519,-24.92049
-8.30683,-52.95625
11.42188,-69.57013
0,-83.06873
v
-83.06811
l
-34.26568,42.57292
q
-8.30684,13.49862
-48.80263,40.49519
l
-49.841,-39.45683
Q
99.760328,557.78928
88.33844,533.90714
99.760328,499.64167
136.10271,475.75953
l
67.49301,-53.99462
57.10946,-62.30123
q
28.03557,-33.22712
57.10947,-53.9946
29.07391,-20.76688
55.03276,-58.14762
26.9972,-36.34218
62.30124,-59.18595
36.34239,-21.80524
47.76428,-45.68738
12.46024,-23.88276
20.76708,-23.88276
17.65201,12.46025
43.61086,52.95626
25.95885,40.49601
65.4163,66.45486
l
72.68478,55.03235
57.10946,58.14822
60.22453,60.22453
q
36.34259,31.1502
47.76427,60.22432
11.42191,29.07412
34.26569,45.68736
23.88214,17.65244
34.26589,16.61387
l
-43.61088,41.53437
-39.45764,41.53374
q
-17.65203,-26.99657
-38.4191,-40.49519
l
-44.64922,-42.57292
v
60.22453
105.91231
83.06811
h
-2.07671
q
-7.26826,4.15217
2.07671,83.0681";
