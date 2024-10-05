use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    resources::spawn_menu_selection::SpawnMenuSelection,
};

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        tracing::info!("All De-Selected");

        SpawnMenuSelection::reset(&mut selected_item);

        user_interface_event.send(UserInterfaceEvent {});
    }
}
