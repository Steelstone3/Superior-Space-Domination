use bevy::{
    ecs::{
        event::EventReader,
        system::{Commands, Query},
    },
    hierarchy::DespawnRecursiveExt,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SpawnSubMenuQuery,
};

pub fn despawn_sub_menus(
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SpawnSubMenuQuery>,
    mut commands: Commands,
) {
    for _ in user_interface_events.read() {
        if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
            commands.entity(sub_menu_query.entity).despawn_recursive();
        }
    }
}
