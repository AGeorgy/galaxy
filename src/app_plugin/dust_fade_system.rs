use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use super::{lod_setting_resource, star_component};

pub fn update_dust_fade(
    mut bloom_settings_query: Query<&mut BloomSettings>,
    lod_settings: ResMut<lod_setting_resource::LodSetting>,
    projection_query: Query<&OrthographicProjection, Changed<OrthographicProjection>>,
    mut star_query: Query<
        (&mut star_component::Alpha, &mut Sprite),
        AnyOf<(
            &star_component::DustTag,
            &star_component::DustFilamentsTag,
            &star_component::H2Tag,
            &star_component::H2CoreTag,
        )>,
    >,
) {
    const KNEE_0: f32 = 0.4;
    const KNEE_1: f32 = 0.16;
    const KNEE_2: f32 = 0.35;
    for projection in &projection_query {
        if lod_settings.is_other_visibile {
            if projection.scale > lod_settings.other_visibility.x
                && projection.scale < lod_settings.other_visibility.y
            {
                bloom_settings_query.single_mut().knee = remap(
                    lod_settings.other_visibility.x,
                    lod_settings.other_visibility.y,
                    KNEE_1,
                    KNEE_2,
                    projection.scale,
                );

                for (alpha, mut sprite) in &mut star_query {
                    let mut color = sprite.color;
                    color.set_a(remap(
                        lod_settings.other_visibility.x,
                        lod_settings.other_visibility.y,
                        0.,
                        alpha.0,
                        projection.scale,
                    ));
                    sprite.color = color;
                }
            }
        }

        if projection.scale < lod_settings.other_visibility.x {
            bloom_settings_query.single_mut().knee = remap(
                0.,
                lod_settings.other_visibility.x,
                KNEE_0,
                KNEE_1,
                projection.scale,
            );
        }
    }
}

fn remap(src_start: f32, src_end: f32, dst_start: f32, dst_end: f32, x: f32) -> f32 {
    return lerp(dst_start, dst_end, unlerp(src_start, src_end, x));
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}

fn unlerp(start: f32, end: f32, t: f32) -> f32 {
    (t - start) / (end - start)
}
