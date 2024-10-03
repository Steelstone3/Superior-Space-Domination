use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use rand::{random, Rng};

use crate::{
    components::sun::Sun,
    events::{
        spawn_planet_event::SpawnPlanetEvent,
        spawn_sprite_event_2::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent2},
    },
    resources::constants::{NUMBER_OF_TILES, SPACE_TILE_SIZE},
};

pub fn spawn_sun(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent2>,
    mut spawn_planet_event: EventWriter<SpawnPlanetEvent>,
) {
    let mut rng = rand::thread_rng();
    let none_player_owned_stars: usize = rng.gen_range(1..3);

    for _ in 0..none_player_owned_stars {
        let sun = Sun::new(random());

        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 3) as f32)
                ..SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 3) as f32),
        );
        let y: f32 = rng.gen_range(
            -SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 3) as f32)
                ..SPACE_TILE_SIZE * ((NUMBER_OF_TILES - 3) as f32),
        );

        let sun_transform = Transform {
            translation: Vec3::new(x, y, sun.size_component.z_index),
            ..Default::default()
        };

        let mut rng = rand::thread_rng();
        let number_of_planets: usize = rng.gen_range(1..5);

        for _ in 0..number_of_planets {
            spawn_planet_event.send(SpawnPlanetEvent { sun_transform });
        }

        spawn_sprite_event.send(SpawnSpriteEvent2::spawn_animated_sprite(
            SpawnSprite {
                sprite_path: sun.sprite_path.to_string(),
                size: sun.size_component.size,
                transform: sun_transform,
                entity: commands.spawn(sun).id(),
            },
            SpawnAnimatedSprite {
                sprite_tile_size: 200,
                frame_timing: 0.1,
                frame_count: 50,
            },
        ));
    }
}
