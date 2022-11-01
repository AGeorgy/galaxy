use bevy::prelude::*;

mod systems;

pub(crate) struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(systems::setup);
        //.add_system(some_system);
    }
}
