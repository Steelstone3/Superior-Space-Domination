use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Quat,
    transform::components::Transform,
};
use rand::{random, Rng};

use crate::{
    components::{selectable::Selectable, space_station::SpaceStation},
    events::{
        spawn_sprite_event_2::{SpawnSprite, SpawnSpriteEvent2},
        spawn_startership_event::SpawnStarterShipEvent,
    },
    resources::constants::SPACE_STATION_DISTANCE_FROM_CENTRE,
};

pub fn spawn_space_stations(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent2>,
    mut spawn_startership_event: EventWriter<SpawnStarterShipEvent>,
) {
    let angle = 360.0 / rand::thread_rng().gen_range(1.0..4.0) as f32;

    let space_station = SpaceStation::new(random());
    let mut transform = Transform::from_xyz(0.0, 0.0, space_station.size_component.z_index)
        .with_rotation(Quat::from_rotation_z(angle.to_radians()));

    transform.translation += transform.up() * SPACE_STATION_DISTANCE_FROM_CENTRE;

    spawn_sprite_event.send(SpawnSpriteEvent2::spawn_sprite(SpawnSprite {
        sprite_path: space_station.sprite_path.to_string(),
        size: space_station.size_component.size,
        transform,
        entity: commands.spawn(space_station).insert(Selectable).id(),
    }));

    // TODO AH Spawn Events Part 2
    spawn_startership_event.send(SpawnStarterShipEvent { transform });
}
