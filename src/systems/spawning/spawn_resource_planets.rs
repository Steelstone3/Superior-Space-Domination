use std::cmp;

use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Commands,
    },
    math::Quat,
};
use rand::{random, Rng};

use crate::{
    components::planet::Planet,
    events::{
        spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
        spawn_transform_dependent_sprite_events::SpawnPlanetEvent,
    },
    resources::constants::PLANET_CLOSEST_DISTANCE_TO_SUN,
};

pub fn spawn_resource_planets(
    mut commands: Commands,
    mut spawn_animated_sprite_event: EventWriter<SpawnSpriteEvent>,
    mut spawn_planet_event_reader: EventReader<SpawnPlanetEvent>,
) {
    for spawn_planet_event in spawn_planet_event_reader.read() {
        let mut rng = rand::thread_rng();
        let angle: f32 = rng.gen_range(0.0..360.0);
        let planet = Planet::new(random());
        let mut transform = spawn_planet_event
            .sun_transform
            .with_rotation(Quat::from_rotation_z(angle.to_radians()));

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
