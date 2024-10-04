use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use rand::random;

use crate::{
    components::sun::Sun,
    events::spawn_sprite_event::{SpawnAnimatedSprite, SpawnSprite, SpawnSpriteEvent},
};

pub fn spawn_suns(mut commands: Commands, mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let sun = Sun::new(random());

    let sun_transform = Transform {
        translation: Vec3::new(0.0, 0.0, sun.size_component.z_index),
        ..Default::default()
    };

    spawn_sprite_event.send(SpawnSpriteEvent::spawn_animated_sprite(
        SpawnSprite {
            sprite_path: sun.sprite_path.to_string(),
            size: sun.size_component.size,
            transform: sun_transform,
            entity: commands.spawn(sun).insert(sun_transform).id(),
        },
        SpawnAnimatedSprite {
            sprite_tile_size: 200,
            frame_timing: 0.1,
            frame_count: 50,
        },
    ));
}
