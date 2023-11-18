use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HitNumber {
    pub number: u32,
    pub color: Color,
}

const FADING_SPEED: f32 = 10.0;

impl HitNumber {
    pub fn get_bundle(
        number: u32,
        color: Color,
        transform: Transform,
    ) -> (Text2dBundle, HitNumber) {
        let text_bundle = Text2dBundle {
            text: Text::from_section(
                number.to_string(),
                TextStyle {
                    font_size: 10.0,
                    color,
                    ..default()
                },
            ),
            transform,
            ..default()
        };
        let hit_number = HitNumber { number, color };
        (text_bundle, hit_number)
    }
}

pub fn update_hit_number(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Text), With<HitNumber>>,
    time: Res<Time>,
) {
    for (entity, mut text) in query.iter_mut() {
        text.style.opacity -= 0.01 * FADING_SPEED * time.delta_seconds();
        if text.style.opacity <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
