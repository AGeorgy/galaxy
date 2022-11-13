use bevy::prelude::*;

use super::app_plugin::galaxy_setting_component::GalaxySettings;
use super::components::StarDownButtonTag;
use super::components::StarUpButtonTag;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn update_stars_count(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            AnyOf<(With<StarDownButtonTag>, With<StarUpButtonTag>)>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut galaxy_settings: ResMut<GalaxySettings>,
) {
    for (interaction, mut color, down_or_up) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if down_or_up.0.is_some() {
                    galaxy_settings.count_stars = (galaxy_settings.count_stars - 1000).max(0);
                }
                if down_or_up.1.is_some() {
                    galaxy_settings.count_stars += 1000;
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
