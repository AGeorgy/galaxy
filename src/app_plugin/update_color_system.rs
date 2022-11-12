use bevy::prelude::*;

use super::star_component::{self, Star};

pub fn update_color(
    mut commands: Commands,
    mut star_query: Query<
        (
            Entity,
            &Star,
            &mut Sprite,
            AnyOf<(
                &star_component::StarTag,
                &star_component::DustTag,
                &star_component::DustFilamentsTag,
                &star_component::H2Tag,
                &star_component::H2CoreTag,
            )>,
        ),
        Added<Sprite>,
    >,
) {
    for (entity, star, mut sprite, star_type) in &mut star_query {
        if star_type.0.is_some() {
            let mut color = color_from_temperature_hrd(star.temp) * star.mag;
            color.set_a(1.);
            sprite.color = color;
        }

        if star_type.1.is_some() {
            let color = color_from_temperature_hrd(star.temp) * star.mag;
            sprite.color = color;
            commands
                .entity(entity)
                .insert(star_component::Alpha(color.a()));
        }

        if star_type.2.is_some() {
            let color = color_from_temperature_hrd(star.temp) * star.mag;
            sprite.color = color;
            commands
                .entity(entity)
                .insert(star_component::Alpha(color.a()));
        }

        if star_type.3.is_some() {
            let color =
                color_from_temperature_hrd(star.temp) * star.mag * Vec4::new(2.0, 0.5, 0.5, 1.0);
            sprite.color = color;
            commands
                .entity(entity)
                .insert(star_component::Alpha(color.a()));
        }

        if star_type.4.is_some() {
            let color = Color::WHITE;
            sprite.color = color;
            commands
                .entity(entity)
                .insert(star_component::Alpha(color.a()));
        }
    }
}

fn color_from_temperature_hrd(temp: f32) -> Color {
    let min_temp = 1000.;
    let max_temp = 10000.;
    let col_num = 200.;

    let col: Vec<Color> = vec![
        Color::rgba(1.0, -0.00987248, -0.0166818, 1.0),
        Color::rgba(1.0, 0.000671682, -0.0173831, 1.0),
        Color::rgba(1.0, 0.0113477, -0.0179839, 1.0),
        Color::rgba(1.0, 0.0221357, -0.0184684, 1.0),
        Color::rgba(1.0, 0.0330177, -0.0188214, 1.0),
        Color::rgba(1.0, 0.0439771, -0.0190283, 1.0),
        Color::rgba(1.0, 0.0549989, -0.0190754, 1.0),
        Color::rgba(1.0, 0.0660696, -0.0189496, 1.0),
        Color::rgba(1.0, 0.0771766, -0.0186391, 1.0),
        Color::rgba(1.0, 0.0883086, -0.0181329, 1.0),
        Color::rgba(1.0, 0.0994553, -0.017421, 1.0),
        Color::rgba(1.0, 0.110607, -0.0164945, 1.0),
        Color::rgba(1.0, 0.121756, -0.0153455, 1.0),
        Color::rgba(1.0, 0.132894, -0.0139671, 1.0),
        Color::rgba(1.0, 0.144013, -0.0123534, 1.0),
        Color::rgba(1.0, 0.155107, -0.0104993, 1.0),
        Color::rgba(1.0, 0.166171, -0.0084008, 1.0),
        Color::rgba(1.0, 0.177198, -0.00605465, 1.0),
        Color::rgba(1.0, 0.188184, -0.00345843, 1.0),
        Color::rgba(1.0, 0.199125, -0.000610485, 1.0),
        Color::rgba(1.0, 0.210015, 0.00249014, 1.0),
        Color::rgba(1.0, 0.220853, 0.00584373, 1.0),
        Color::rgba(1.0, 0.231633, 0.00944995, 1.0),
        Color::rgba(1.0, 0.242353, 0.0133079, 1.0),
        Color::rgba(1.0, 0.25301, 0.0174162, 1.0),
        Color::rgba(1.0, 0.263601, 0.021773, 1.0),
        Color::rgba(1.0, 0.274125, 0.0263759, 1.0),
        Color::rgba(1.0, 0.284579, 0.0312223, 1.0),
        Color::rgba(1.0, 0.294962, 0.0363091, 1.0),
        Color::rgba(1.0, 0.305271, 0.0416328, 1.0),
        Color::rgba(1.0, 0.315505, 0.0471899, 1.0),
        Color::rgba(1.0, 0.325662, 0.0529765, 1.0),
        Color::rgba(1.0, 0.335742, 0.0589884, 1.0),
        Color::rgba(1.0, 0.345744, 0.0652213, 1.0),
        Color::rgba(1.0, 0.355666, 0.0716707, 1.0),
        Color::rgba(1.0, 0.365508, 0.078332, 1.0),
        Color::rgba(1.0, 0.375268, 0.0852003, 1.0),
        Color::rgba(1.0, 0.384948, 0.0922709, 1.0),
        Color::rgba(1.0, 0.394544, 0.0995389, 1.0),
        Color::rgba(1.0, 0.404059, 0.106999, 1.0),
        Color::rgba(1.0, 0.41349, 0.114646, 1.0),
        Color::rgba(1.0, 0.422838, 0.122476, 1.0),
        Color::rgba(1.0, 0.432103, 0.130482, 1.0),
        Color::rgba(1.0, 0.441284, 0.138661, 1.0),
        Color::rgba(1.0, 0.450381, 0.147005, 1.0),
        Color::rgba(1.0, 0.459395, 0.155512, 1.0),
        Color::rgba(1.0, 0.468325, 0.164175, 1.0),
        Color::rgba(1.0, 0.477172, 0.172989, 1.0),
        Color::rgba(1.0, 0.485935, 0.181949, 1.0),
        Color::rgba(1.0, 0.494614, 0.19105, 1.0),
        Color::rgba(1.0, 0.503211, 0.200288, 1.0),
        Color::rgba(1.0, 0.511724, 0.209657, 1.0),
        Color::rgba(1.0, 0.520155, 0.219152, 1.0),
        Color::rgba(1.0, 0.528504, 0.228769, 1.0),
        Color::rgba(1.0, 0.536771, 0.238502, 1.0),
        Color::rgba(1.0, 0.544955, 0.248347, 1.0),
        Color::rgba(1.0, 0.553059, 0.2583, 1.0),
        Color::rgba(1.0, 0.561082, 0.268356, 1.0),
        Color::rgba(1.0, 0.569024, 0.27851, 1.0),
        Color::rgba(1.0, 0.576886, 0.288758, 1.0),
        Color::rgba(1.0, 0.584668, 0.299095, 1.0),
        Color::rgba(1.0, 0.592372, 0.309518, 1.0),
        Color::rgba(1.0, 0.599996, 0.320022, 1.0),
        Color::rgba(1.0, 0.607543, 0.330603, 1.0),
        Color::rgba(1.0, 0.615012, 0.341257, 1.0),
        Color::rgba(1.0, 0.622403, 0.35198, 1.0),
        Color::rgba(1.0, 0.629719, 0.362768, 1.0),
        Color::rgba(1.0, 0.636958, 0.373617, 1.0),
        Color::rgba(1.0, 0.644122, 0.384524, 1.0),
        Color::rgba(1.0, 0.65121, 0.395486, 1.0),
        Color::rgba(1.0, 0.658225, 0.406497, 1.0),
        Color::rgba(1.0, 0.665166, 0.417556, 1.0),
        Color::rgba(1.0, 0.672034, 0.428659, 1.0),
        Color::rgba(1.0, 0.678829, 0.439802, 1.0),
        Color::rgba(1.0, 0.685552, 0.450982, 1.0),
        Color::rgba(1.0, 0.692204, 0.462196, 1.0),
        Color::rgba(1.0, 0.698786, 0.473441, 1.0),
        Color::rgba(1.0, 0.705297, 0.484714, 1.0),
        Color::rgba(1.0, 0.711739, 0.496013, 1.0),
        Color::rgba(1.0, 0.718112, 0.507333, 1.0),
        Color::rgba(1.0, 0.724417, 0.518673, 1.0),
        Color::rgba(1.0, 0.730654, 0.53003, 1.0),
        Color::rgba(1.0, 0.736825, 0.541402, 1.0),
        Color::rgba(1.0, 0.742929, 0.552785, 1.0),
        Color::rgba(1.0, 0.748968, 0.564177, 1.0),
        Color::rgba(1.0, 0.754942, 0.575576, 1.0),
        Color::rgba(1.0, 0.760851, 0.586979, 1.0),
        Color::rgba(1.0, 0.766696, 0.598385, 1.0),
        Color::rgba(1.0, 0.772479, 0.609791, 1.0),
        Color::rgba(1.0, 0.778199, 0.621195, 1.0),
        Color::rgba(1.0, 0.783858, 0.632595, 1.0),
        Color::rgba(1.0, 0.789455, 0.643989, 1.0),
        Color::rgba(1.0, 0.794991, 0.655375, 1.0),
        Color::rgba(1.0, 0.800468, 0.666751, 1.0),
        Color::rgba(1.0, 0.805886, 0.678116, 1.0),
        Color::rgba(1.0, 0.811245, 0.689467, 1.0),
        Color::rgba(1.0, 0.816546, 0.700803, 1.0),
        Color::rgba(1.0, 0.82179, 0.712122, 1.0),
        Color::rgba(1.0, 0.826976, 0.723423, 1.0),
        Color::rgba(1.0, 0.832107, 0.734704, 1.0),
        Color::rgba(1.0, 0.837183, 0.745964, 1.0),
        Color::rgba(1.0, 0.842203, 0.757201, 1.0),
        Color::rgba(1.0, 0.847169, 0.768414, 1.0),
        Color::rgba(1.0, 0.852082, 0.779601, 1.0),
        Color::rgba(1.0, 0.856941, 0.790762, 1.0),
        Color::rgba(1.0, 0.861748, 0.801895, 1.0),
        Color::rgba(1.0, 0.866503, 0.812999, 1.0),
        Color::rgba(1.0, 0.871207, 0.824073, 1.0),
        Color::rgba(1.0, 0.87586, 0.835115, 1.0),
        Color::rgba(1.0, 0.880463, 0.846125, 1.0),
        Color::rgba(1.0, 0.885017, 0.857102, 1.0),
        Color::rgba(1.0, 0.889521, 0.868044, 1.0),
        Color::rgba(1.0, 0.893977, 0.878951, 1.0),
        Color::rgba(1.0, 0.898386, 0.889822, 1.0),
        Color::rgba(1.0, 0.902747, 0.900657, 1.0),
        Color::rgba(1.0, 0.907061, 0.911453, 1.0),
        Color::rgba(1.0, 0.91133, 0.922211, 1.0),
        Color::rgba(1.0, 0.915552, 0.932929, 1.0),
        Color::rgba(1.0, 0.91973, 0.943608, 1.0),
        Color::rgba(1.0, 0.923863, 0.954246, 1.0),
        Color::rgba(1.0, 0.927952, 0.964842, 1.0),
        Color::rgba(1.0, 0.931998, 0.975397, 1.0),
        Color::rgba(1.0, 0.936001, 0.985909, 1.0),
        Color::rgba(1.0, 0.939961, 0.996379, 1.0),
        Color::rgba(0.993241, 0.9375, 1.0, 1.0),
        Color::rgba(0.983104, 0.931743, 1.0, 1.0),
        Color::rgba(0.973213, 0.926103, 1.0, 1.0),
        Color::rgba(0.963562, 0.920576, 1.0, 1.0),
        Color::rgba(0.954141, 0.915159, 1.0, 1.0),
        Color::rgba(0.944943, 0.909849, 1.0, 1.0),
        Color::rgba(0.935961, 0.904643, 1.0, 1.0),
        Color::rgba(0.927189, 0.899538, 1.0, 1.0),
        Color::rgba(0.918618, 0.894531, 1.0, 1.0),
        Color::rgba(0.910244, 0.88962, 1.0, 1.0),
        Color::rgba(0.902059, 0.884801, 1.0, 1.0),
        Color::rgba(0.894058, 0.880074, 1.0, 1.0),
        Color::rgba(0.886236, 0.875434, 1.0, 1.0),
        Color::rgba(0.878586, 0.87088, 1.0, 1.0),
        Color::rgba(0.871103, 0.86641, 1.0, 1.0),
        Color::rgba(0.863783, 0.862021, 1.0, 1.0),
        Color::rgba(0.856621, 0.857712, 1.0, 1.0),
        Color::rgba(0.849611, 0.853479, 1.0, 1.0),
        Color::rgba(0.84275, 0.849322, 1.0, 1.0),
        Color::rgba(0.836033, 0.845239, 1.0, 1.0),
        Color::rgba(0.829456, 0.841227, 1.0, 1.0),
        Color::rgba(0.823014, 0.837285, 1.0, 1.0),
        Color::rgba(0.816705, 0.83341, 1.0, 1.0),
        Color::rgba(0.810524, 0.829602, 1.0, 1.0),
        Color::rgba(0.804468, 0.825859, 1.0, 1.0),
        Color::rgba(0.798532, 0.82218, 1.0, 1.0),
        Color::rgba(0.792715, 0.818562, 1.0, 1.0),
        Color::rgba(0.787012, 0.815004, 1.0, 1.0),
        Color::rgba(0.781421, 0.811505, 1.0, 1.0),
        Color::rgba(0.775939, 0.808063, 1.0, 1.0),
        Color::rgba(0.770561, 0.804678, 1.0, 1.0),
        Color::rgba(0.765287, 0.801348, 1.0, 1.0),
        Color::rgba(0.760112, 0.798071, 1.0, 1.0),
        Color::rgba(0.755035, 0.794846, 1.0, 1.0),
        Color::rgba(0.750053, 0.791672, 1.0, 1.0),
        Color::rgba(0.745164, 0.788549, 1.0, 1.0),
        Color::rgba(0.740364, 0.785474, 1.0, 1.0),
        Color::rgba(0.735652, 0.782448, 1.0, 1.0),
        Color::rgba(0.731026, 0.779468, 1.0, 1.0),
        Color::rgba(0.726482, 0.776534, 1.0, 1.0),
        Color::rgba(0.722021, 0.773644, 1.0, 1.0),
        Color::rgba(0.717638, 0.770798, 1.0, 1.0),
        Color::rgba(0.713333, 0.767996, 1.0, 1.0),
        Color::rgba(0.709103, 0.765235, 1.0, 1.0),
        Color::rgba(0.704947, 0.762515, 1.0, 1.0),
        Color::rgba(0.700862, 0.759835, 1.0, 1.0),
        Color::rgba(0.696848, 0.757195, 1.0, 1.0),
        Color::rgba(0.692902, 0.754593, 1.0, 1.0),
        Color::rgba(0.689023, 0.752029, 1.0, 1.0),
        Color::rgba(0.685208, 0.749502, 1.0, 1.0),
        Color::rgba(0.681458, 0.747011, 1.0, 1.0),
        Color::rgba(0.67777, 0.744555, 1.0, 1.0),
        Color::rgba(0.674143, 0.742134, 1.0, 1.0),
        Color::rgba(0.670574, 0.739747, 1.0, 1.0),
        Color::rgba(0.667064, 0.737394, 1.0, 1.0),
        Color::rgba(0.663611, 0.735073, 1.0, 1.0),
        Color::rgba(0.660213, 0.732785, 1.0, 1.0),
        Color::rgba(0.656869, 0.730528, 1.0, 1.0),
        Color::rgba(0.653579, 0.728301, 1.0, 1.0),
        Color::rgba(0.65034, 0.726105, 1.0, 1.0),
        Color::rgba(0.647151, 0.723939, 1.0, 1.0),
        Color::rgba(0.644013, 0.721801, 1.0, 1.0),
        Color::rgba(0.640922, 0.719692, 1.0, 1.0),
        Color::rgba(0.637879, 0.717611, 1.0, 1.0),
        Color::rgba(0.634883, 0.715558, 1.0, 1.0),
        Color::rgba(0.631932, 0.713531, 1.0, 1.0),
        Color::rgba(0.629025, 0.711531, 1.0, 1.0),
        Color::rgba(0.626162, 0.709557, 1.0, 1.0),
        Color::rgba(0.623342, 0.707609, 1.0, 1.0),
        Color::rgba(0.620563, 0.705685, 1.0, 1.0),
        Color::rgba(0.617825, 0.703786, 1.0, 1.0),
        Color::rgba(0.615127, 0.701911, 1.0, 1.0),
        Color::rgba(0.612469, 0.70006, 1.0, 1.0),
        Color::rgba(0.609848, 0.698231, 1.0, 1.0),
        Color::rgba(0.607266, 0.696426, 1.0, 1.0),
        Color::rgba(0.60472, 0.694643, 1.0, 1.0),
    ];

    let mut idx = ((temp - min_temp) / (max_temp - min_temp) * col_num).floor();
    idx = idx.min(col_num - 1.);
    idx = idx.max(0.);
    return col[idx as usize];
}
