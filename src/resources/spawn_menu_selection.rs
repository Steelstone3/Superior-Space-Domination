use bevy::ecs::system::{ResMut, Resource};

use crate::{
    assets::user_interace::icons::starships::atark_icons::AtarkIcon,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

// TODO AH Make generic current only works for Atark faction
#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub starship_selection: AtarkIcon,
    // pub facilities_selection: Facility,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            starship_selection: AtarkIcon::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset(selected_item: &mut ResMut<'_, SpawnMenuSelection>) {
        selected_item.selection = SpawnSelection::None;
        selected_item.starship_selection = AtarkIcon::None;
    }
}
