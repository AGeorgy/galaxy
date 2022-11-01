use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

mod app_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(app_plugin::AppPlugin)
        .run();
}
