use bevy::prelude::*;

mod app_plugin;
mod app_ui_plugin;
mod pan_cam;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(pan_cam::PanCamPlugin)
        .add_plugin(app_plugin::AppPlugin)
        .add_plugin(app_ui_plugin::AppUIPlugin)
        // Loging
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .run();
}
