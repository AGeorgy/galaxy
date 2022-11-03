use bevy::prelude::*;

mod components;
mod density_wave_resource;
mod systems;

pub(crate) struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(systems::setup)
            .add_startup_system(systems::create_stars);
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //.add_system(some_system);
    }
}
