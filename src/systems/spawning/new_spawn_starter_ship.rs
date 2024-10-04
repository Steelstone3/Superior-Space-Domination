use bevy::prelude::{Commands, EventWriter, Query, Transform, With};

use crate::{
    assets::images::faction_starships::starships::StarshipSprite,
    components::{selectable::Selectable, space_station::SpaceStation, starship::Starship},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
};

pub fn new_spawn_starter_ship(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    space_station_query: Query<&Transform, With<SpaceStation>>,
) {
    for transform in space_station_query.iter() {
        let starship = Starship::new(StarshipSprite::AtarkBattleCruiser);

        let mut starship_transform = *transform;
        starship_transform.translation.z = starship.size_component.z_index;

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite_bundle.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform: starship_transform,
            entity: commands.spawn(starship).insert(Selectable).id(),
        }));
    }
}
