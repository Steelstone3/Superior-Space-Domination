use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use rand::random;

use crate::{
    components::space::Space,
    events::spawn_sprite_event::SpawnSpriteEvent,
    resources::constants::{MAP_TILES, SPACE_TILE_SIZE},
};

pub fn spawn_space(mut commands: Commands, mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let space = Space::new(random());

    for x in MAP_TILES {
        for y in MAP_TILES {
            spawn_sprite_event.send(SpawnSpriteEvent {
                sprite_path: space.sprite_path.to_string(),
                size: space.transform.size,
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                        (y as f32 * SPACE_TILE_SIZE) + (SPACE_TILE_SIZE / 2.0),
                        space.transform.z_index,
                    ),
                    ..Default::default()
                },
                entity: commands.spawn(space).id(),
            });
        }
    }
}
