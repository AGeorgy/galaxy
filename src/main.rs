use bevy::prelude::*;
// use bevy_egui::EguiPlugin;
// use bevy_inspector_egui::WorldInspectorPlugin;

mod app_plugin;
mod pan_cam;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(pan_cam::PanCamPlugin)
        // .add_plugin(EguiPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(app_plugin::AppPlugin)
        .run();
}
