use bevy::prelude::*;

use super::lod_setting_resource;
use super::star_component;

pub fn update_stars_visibility(
    mut lod_settings: ResMut<lod_setting_resource::LodSetting>,
    projection: Query<&OrthographicProjection>,
    mut star_query: Query<&mut Visibility, With<star_component::StarTag>>,
) {
    let is_stars_visibile = lod_settings.is_stars_visibile;
    let scale = projection.single().scale;

    if !is_stars_visibile
        && scale > lod_settings.stars_visibility.x
        && scale < lod_settings.stars_visibility.y
    {
        lod_settings.is_stars_visibile = change_visibility(&mut star_query, true);
    }

    if is_stars_visibile
        && (scale <= lod_settings.stars_visibility.x || scale >= lod_settings.stars_visibility.y)
    {
        lod_settings.is_stars_visibile = change_visibility(&mut star_query, false);
    }

    fn change_visibility(
        star_query: &mut Query<&mut Visibility, With<star_component::StarTag>>,
        is_visible: bool,
    ) -> bool {
        for mut visibility in star_query {
            visibility.is_visible = is_visible;
        }
        is_visible
    }
}

pub fn update_other_visibility(
    mut lod_settings: ResMut<lod_setting_resource::LodSetting>,
    projection: Query<&OrthographicProjection>,
    mut star_query: Query<
        &mut Visibility,
        AnyOf<(
            &star_component::DustTag,
            &star_component::DustFilamentsTag,
            &star_component::H2Tag,
            &star_component::H2CoreTag,
        )>,
    >,
) {
    let is_stars_visibile = lod_settings.is_other_visibile;
    let scale = projection.single().scale;

    if !is_stars_visibile
        && scale > lod_settings.other_visibility.x
        && scale < lod_settings.other_visibility.y
    {
        lod_settings.is_other_visibile = change_visibility(&mut star_query, true);
    }

    if is_stars_visibile
        && (scale <= lod_settings.other_visibility.x || scale >= lod_settings.other_visibility.y)
    {
        lod_settings.is_other_visibile = change_visibility(&mut star_query, false);
    }

    fn change_visibility(
        star_query: &mut Query<
            &mut Visibility,
            AnyOf<(
                &star_component::DustTag,
                &star_component::DustFilamentsTag,
                &star_component::H2Tag,
                &star_component::H2CoreTag,
            )>,
        >,
        is_visible: bool,
    ) -> bool {
        for mut visibility in star_query {
            visibility.is_visible = is_visible;
        }
        is_visible
    }
}
