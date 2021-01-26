use crate::demo_camera_plugin::Page;
use bevy::prelude::*;

/// Tags the main text.
struct TitleText;
/// Tags the subtitle text.
struct SubtitleText;
/// Text visible only on a certain page.
struct BodyText(isize);

pub struct DemoUiPlugin;

impl Plugin for DemoUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_title_text.system())
            .add_system(update_subtitle_text.system())
            .add_system(body_text_visibility.system());
    }
}

#[allow(clippy::too_many_lines)] // Too homogeneous to split
fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    // UI camera
    commands.spawn(CameraUiBundle::default());

    // Title text
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                margin: Rect {
                    left: Val::Px(30.0),
                    top: Val::Px(30.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
                ..Text::default()
            },
            ..TextBundle::default()
        })
        .with(TitleText);

    // Subtitle text
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(30.0),
                    top: Val::Px(100.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
                ..Text::default()
            },
            ..TextBundle::default()
        })
        .with(SubtitleText);

    // Welcome text
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(150.0),
                    left: Val::Px(250.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 90.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
                value: "Welcome to the\nbevy_prototype_lyon\ndemo!".to_string(),
            },
            ..TextBundle::default()
        })
        .with(BodyText(0));

    // Welcome text description
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(500.0),
                    left: Val::Px(350.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
                value: "Use the AD keys to move across pages".to_string(),
            },
            ..TextBundle::default()
        })
        .with(BodyText(0));

    // Closing text
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(150.0),
                    left: Val::Px(250.0),
                    ..Rect::default()
                },
                ..Style::default()
            },
            text: Text {
                font,
                style: TextStyle {
                    font_size: 90.0,
                    color: Color::WHITE,
                    ..TextStyle::default()
                },
                value: "Thanks for using\nbevy_prototype_lyon!".to_string(),
            },
            ..TextBundle::default()
        })
        .with(BodyText(5));
}

fn update_title_text(mut query: Query<&mut Text, With<TitleText>>, page: ChangedRes<Page>) {
    for mut text in query.iter_mut() {
        text.value = match page.0 {
            1 => "Easy physics integration",
            2 => "Draw paths like you do in <canvas>",
            3 => "Draw perfect regular polygons",
            4 => "This is a single entity",
            _ => "",
        }
        .to_owned();
    }
}

fn update_subtitle_text(mut query: Query<&mut Text, With<SubtitleText>>, page: ChangedRes<Page>) {
    for mut text in query.iter_mut() {
        text.value = match page.0 {
            1 => "Also look at the transparency effect :3",
            2 => "Has still room for improvement",
            3 => "Use the inspector to tweak the properties!",
            4 => "Try to move it",
            _ => "",
        }
        .to_owned();
    }
}

fn body_text_visibility(mut query: Query<(&mut Visible, &BodyText)>, slide: ChangedRes<Page>) {
    for (mut visible, body_text) in query.iter_mut() {
        visible.is_visible = body_text.0 == slide.0;
    }
}
