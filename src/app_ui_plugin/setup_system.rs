use bevy::prelude::*;

use super::components::FpsTag;
use super::components::StarDownButtonTag;
use super::components::StarUpButtonTag;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Move by mouse draging. Zoome by scrolling\n",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " fps, ",
                TextStyle {
                    font: font.clone(),
                    font_size: 13.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                " stars ",
                TextStyle {
                    font: font.clone(),
                    font_size: 13.0,
                    color: Color::GREEN,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
        FpsTag,
    ));

    // Buttons
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(40.0)),
                    position: UiRect {
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(1., 1., 1., 0.3).into(),
                ..default()
            },
            StarUpButtonTag,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "+ 1000 stars",
                TextStyle {
                    font: font.clone(),
                    font_size: 13.0,
                    color: Color::WHITE,
                },
            ));
        });
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(40.0)),
                    position: UiRect {
                        top: Val::Px(10.0),
                        left: Val::Px(20.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(1., 1., 1., 0.3).into(),
                ..default()
            },
            StarDownButtonTag,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "- 1000 stars",
                TextStyle {
                    font: font.clone(),
                    font_size: 13.0,
                    color: Color::WHITE,
                },
            ));
        });
}
