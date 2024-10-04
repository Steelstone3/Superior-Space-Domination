use crate::{
    assets::images::faction_starships::starships::StarshipSprite,
    components::{selectable::Selectable, starship::Starship},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    queries::faction_queries::SpaceStationQuery,
};
use bevy::prelude::{Commands, EventWriter, Query};

pub fn spawn_starter_spaceship(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    space_station_queries: Query<SpaceStationQuery>,
) {
    for space_station_query in space_station_queries.iter() {
        let starship = Starship::new(StarshipSprite::AtarkBattleCruiser);

        let mut starship_transform = *space_station_query.transform;
        starship_transform.translation.z = starship.size_component.z_index;

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite_bundle.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform: starship_transform,
            entity: commands.spawn(starship).insert(Selectable).id(),
        }));
    }
}
