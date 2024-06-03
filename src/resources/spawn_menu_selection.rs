use bevy::{
    ecs::system::{ResMut, Resource},
    prelude::Entity,
};

use crate::{
    assets::user_interace::icons::starship_icons::StarshipIcon,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

// TODO AH Make generic current only works for Atark faction
#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub selected_entity: Entity,
    pub starship_selection: StarshipIcon,
    // pub facilities_selection: Facility,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            starship_selection: StarshipIcon::None,
            selected_entity: Entity::PLACEHOLDER,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset(selected_item: &mut ResMut<'_, SpawnMenuSelection>) {
        selected_item.selection = SpawnSelection::None;
        selected_item.starship_selection = StarshipIcon::None;
        selected_item.selected_entity = Entity::PLACEHOLDER;
    }
}
