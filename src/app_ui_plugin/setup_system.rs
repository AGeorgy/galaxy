use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn(
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
    );
}
