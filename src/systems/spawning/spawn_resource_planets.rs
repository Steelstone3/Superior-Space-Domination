use std::cmp;

use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    prelude::{Query, Transform, With},
};
use rand::{random, Rng};

use crate::{
    components::{planet::Planet, sun::Sun},
    events::spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
    resources::constants::PLANET_CLOSEST_DISTANCE_TO_SUN,
};

pub fn spawn_resource_planets(
    mut commands: Commands,
    mut spawn_animated_sprite_event: EventWriter<SpawnSpriteEvent>,
    sun_transform_query: Query<&Transform, With<Sun>>,
) {
    let Ok(sun_transform) = sun_transform_query.get_single() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let number_of_planets: usize = rng.gen_range(3..6);

    for _ in 0..number_of_planets {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(0.0..360.0);
        let planet = Planet::new(random());
        let mut transform = sun_transform.with_rotation(Quat::from_rotation_z(angle.to_radians()));

        transform.translation +=
            transform.up() * PLANET_CLOSEST_DISTANCE_TO_SUN * rng.gen_range(1.0..3.0)
                + (cmp::max(
                    planet.size_component.size.x as u32,
                    planet.size_component.size.y as u32,
                ) * 2) as f32;

        spawn_animated_sprite_event.send(SpawnSpriteEvent::spawn_animated_sprite(
            SpawnSprite {
                sprite_path: planet.sprite_path.to_string(),
                size: planet.size_component.size,
                transform,
                entity: commands.spawn(planet).id(),
            },
            SpawnAnimatedSprite {
                sprite_tile_size: 100,
                frame_timing: 0.1,
                frame_count: 50,
            },
        ));
    }
}
