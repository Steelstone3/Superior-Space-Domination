use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    utils::tracing,
};

use crate::{
    assets::user_interace::icons::starships::atark_icons::AtarkIcon,
    events::user_interface_event::UserInterfaceEvent,
    resources::spawn_menu_selection::SpawnMenuSelection,
};

use super::spawn_selection::SpawnSelection;

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        tracing::info!("All De-Selected");

        selected_item.selection = SpawnSelection::None;
        selected_item.starship_selection = AtarkIcon::None;

        user_interface_event.send(UserInterfaceEvent {});
    }
}
