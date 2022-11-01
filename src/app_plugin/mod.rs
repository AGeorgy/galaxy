use bevy::prelude::*;

mod systems;

pub(crate) struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup);
        //.add_system(some_system);
    }
}
