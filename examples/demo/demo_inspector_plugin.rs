use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContext,
};
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, TessellationMode},
    shapes::{self, RegularPolygonFeature},
};
use lyon_tessellation::FillOptions;

use crate::{demo_camera_plugin::Page, MultishapeTag, WINDOW_WIDTH};

#[allow(clippy::suboptimal_flops)] // can't use mul_add in consts.
const X_OFFSET: f32 = 3.0 * WINDOW_WIDTH + WINDOW_WIDTH / 5.0;

/// Tags the regular polygon entity.
pub struct PolygonTag;

#[derive(Debug)]
struct PolygonInspector {
    pos_x: f32,
    pos_y: f32,
    rotation: f32,
    color: egui::Color32,
    sides: usize,
    feature: RegularPolygonFeature,
}

impl Default for PolygonInspector {
    fn default() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            rotation: 0.0,
            color: egui::Color32::GOLD,
            sides: 3,
            feature: RegularPolygonFeature::Radius(100.0),
        }
    }
}

#[derive(Debug)]
struct MultishapeInspector {
    pos_x: f32,
    pos_y: f32,
    rotation: f32,
    scale: f32,
}

impl Default for MultishapeInspector {
    fn default() -> Self {
        Self {
            pos_x: 0.0,
            pos_y: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

pub struct DemoInspectorPlugin;

impl Plugin for DemoInspectorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PolygonInspector::default())
            .insert_resource(MultishapeInspector::default())
            .add_system(update_polygon_inspector.system())
            .add_system(replace_polygon.system())
            .add_system(update_multishape_inspector.system());
    }
}

fn update_polygon_inspector(
    page: Res<Page>,
    mut inspector: ResMut<PolygonInspector>,
    mut egui_context: ResMut<EguiContext>,
) {
    if page.0 == 3 {
        let ctx = &mut egui_context.ctx;

        egui::Window::new("Polygon Inspector")
            .fixed_pos([50.0, 200.0])
            .show(ctx, |ui| {
                polygon_bevy_properties(ui, &mut inspector);
                ui.separator();
                polygon_shape_properties(ui, &mut inspector);
            });
    }
}

fn polygon_bevy_properties(ui: &mut Ui, inspector: &mut ResMut<PolygonInspector>) {
    ui.horizontal(|ui| {
        ui.label("x");
        ui.add(egui::widgets::DragValue::f32(&mut inspector.pos_x));
    });
    ui.horizontal(|ui| {
        ui.label("y");
        ui.add(egui::widgets::DragValue::f32(&mut inspector.pos_y));
    });
    ui.horizontal(|ui| {
        ui.label("rotation");
        ui.drag_angle(&mut inspector.rotation);
    });
    ui.horizontal(|ui| {
        ui.label("color");
        ui.color_edit_button_srgba(&mut inspector.color);
    });
}

fn polygon_shape_properties(ui: &mut Ui, inspector: &mut ResMut<PolygonInspector>) {
    ui.horizontal(|ui| {
        ui.label("sides");
        ui.add(egui::widgets::Slider::usize(&mut inspector.sides, 3..=10));
        ui.label(inspector.sides.to_string());
    });
    egui::containers::combo_box_with_label(
        ui,
        "Polygon feature",
        format!("{:?}", inspector.feature),
        |ui| {
            ui.selectable_value(
                &mut inspector.feature,
                RegularPolygonFeature::Radius(100.0),
                "Radius",
            );
            ui.selectable_value(
                &mut inspector.feature,
                RegularPolygonFeature::Apothem(100.0),
                "Apothem",
            );
            ui.selectable_value(
                &mut inspector.feature,
                RegularPolygonFeature::SideLength(100.0),
                "Side length",
            );
        },
    );
    match inspector.feature {
        RegularPolygonFeature::Radius(ref mut r) => {
            ui.add(egui::widgets::Slider::f32(r, 10.0..=200.0));
        }
        RegularPolygonFeature::Apothem(ref mut a) => {
            ui.add(egui::widgets::Slider::f32(a, 10.0..=200.0));
        }
        RegularPolygonFeature::SideLength(ref mut s) => {
            ui.add(egui::widgets::Slider::f32(s, 10.0..=200.0));
        }
    }
}

fn replace_polygon(
    commands: &mut Commands,
    query: Query<Entity, With<PolygonTag>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    inspector: ChangedRes<PolygonInspector>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }
    let fill_mode = TessellationMode::Fill(FillOptions::default());

    let polygon = shapes::RegularPolygon {
        sides: inspector.sides,
        feature: inspector.feature,
        center: Vec2::new(0.0, 0.0),
    };

    let r = f32::from(inspector.color.r()) / 255.0;
    let g = f32::from(inspector.color.g()) / 255.0;
    let b = f32::from(inspector.color.b()) / 255.0;
    let a = f32::from(inspector.color.a()) / 255.0;

    commands
        .spawn(GeometryBuilder::build_as(
            &polygon,
            materials.add(ColorMaterial::color(Color::rgba_linear(r, g, b, a))),
            fill_mode,
            Transform {
                translation: Vec3::new(X_OFFSET + inspector.pos_x, inspector.pos_y, 0.0),
                rotation: Quat::from_axis_angle(Vec3::unit_z(), inspector.rotation),
                ..Transform::default()
            },
        ))
        .with(PolygonTag);
}

fn update_multishape_inspector(
    mut query: Query<&mut Transform, With<MultishapeTag>>,
    page: Res<Page>,
    mut inspector: ResMut<MultishapeInspector>,
    mut egui_context: ResMut<EguiContext>,
) {
    if page.0 == 4 {
        let ctx = &mut egui_context.ctx;

        egui::Window::new("Multishape Inspector")
            .fixed_pos([50.0, 200.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(egui::widgets::DragValue::f32(&mut inspector.pos_x));
                });
                ui.horizontal(|ui| {
                    ui.label("y");
                    ui.add(egui::widgets::DragValue::f32(&mut inspector.pos_y));
                });
                ui.horizontal(|ui| {
                    ui.label("rotation");
                    ui.drag_angle(&mut inspector.rotation);
                });
                ui.horizontal(|ui| {
                    ui.label("scale");
                    ui.add(
                        egui::widgets::DragValue::f32(&mut inspector.scale)
                            .clamp_range(-1.5..=1.5)
                            .speed(0.1),
                    );
                });

                for mut transform in query.iter_mut() {
                    transform.translation.x = WINDOW_WIDTH.mul_add(4.0, inspector.pos_x);
                    transform.translation.y = inspector.pos_y;
                    transform.rotation = Quat::from_axis_angle(Vec3::unit_z(), inspector.rotation);
                    transform.scale = Vec3::splat(inspector.scale);
                }
            });
    }
}
