use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LodSetting {
    pub stars_visibility: Vec2,
    pub other_visibility: Vec2,

    pub is_stars_visibile: bool,
    pub is_other_visibile: bool,
}
