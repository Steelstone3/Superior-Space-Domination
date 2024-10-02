use bevy::ecs::system::{ResMut, Resource};

use crate::{
    assets::images::faction_starships::starships::StarshipSprite,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

#[derive(Resource)]
pub struct SpawnMenuSelection {
    pub selection: SpawnSelection,
    pub ships_selection: StarshipSprite,
    // pub facilities_selection: Facility,
}

impl Default for SpawnMenuSelection {
    fn default() -> Self {
        Self {
            selection: SpawnSelection::None,
            ships_selection: StarshipSprite::None,
        }
    }
}

impl SpawnMenuSelection {
    pub fn reset(selected_item: &mut ResMut<'_, SpawnMenuSelection>) {
        selected_item.selection = SpawnSelection::None;
        selected_item.ships_selection = StarshipSprite::None;
    }
}
