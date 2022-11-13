use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::app_plugin::galaxy_setting_component::GalaxySettings;

pub fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        text.sections[1].value = format!("FPS: {:.1}", fps,);
    }
}

pub fn update_stars_count(galaxy_settings: Res<GalaxySettings>, mut query: Query<&mut Text>) {
    if !galaxy_settings.is_changed() {
        return;
    }
    for mut text in &mut query {
        text.sections[2].value = format!(
            "  StarsCount: {}, All objects count: {}",
            galaxy_settings.count_stars,
            galaxy_settings.get_count_all_objects()
        );
    }
}
