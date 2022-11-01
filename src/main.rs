use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        //.add_startup_system(hello_world)
        //.add_system(hello_world)
        .run();
}
