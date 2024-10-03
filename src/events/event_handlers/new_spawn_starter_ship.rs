use bevy::prelude::{Commands, EventReader, EventWriter};

use crate::{
    assets::images::faction_starships::starships::StarshipSprite,
    components::{selectable::Selectable, starship::Starship},
    events::{
        spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
        spawn_transform_dependent_sprite_events::SpawnStarterSpaceshipEvent,
    },
};

pub fn new_spawn_starter_ship(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    mut spawn_starter_spaceship_event_reader: EventReader<SpawnStarterSpaceshipEvent>,
) {
    for event in spawn_starter_spaceship_event_reader.read() {
        let starship = Starship::new(StarshipSprite::AtarkBattleCruiser);

        let mut starship_transform = event.starbase_transform;
        starship_transform.translation.z = starship.size_component.z_index;

        let sprite = starship.starship_sprite_bundle.starship_sprite.to_string();
        let size = starship.size_component.size;
        let entity = commands.spawn(starship).insert(Selectable).id();

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: sprite,
            size,
            transform: starship_transform,
            entity,
        }));
    }
}
