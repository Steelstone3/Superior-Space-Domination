use bevy::ecs::system::{ResMut, Resource};

use crate::{
    assets::user_interace::icons::starship_icons::StarshipIcon,
    systems::user_interface::interactions::spawn_selection::Selection,
};

// TODO AH Make generic current only works for Atark faction
#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: Selection,
    pub starship_selection: StarshipIcon,
    // pub facilities_selection: Facility,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: Selection::None,
            starship_selection: StarshipIcon::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset(selected_item: &mut ResMut<'_, SpawnMenuSelection>) {
        selected_item.selection = Selection::None;
        selected_item.starship_selection = StarshipIcon::None;
    }
}
