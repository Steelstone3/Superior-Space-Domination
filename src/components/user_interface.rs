use crate::assets::user_interace::{
    icons::starship_icons::StarshipIcon, team::TeamSelectionSprite,
};
use bevy::prelude::Component;

#[derive(Component)]
pub struct SpawnMenu;

#[derive(Component)]
pub struct SpawnSubMenuButton;

#[derive(Component)]
pub struct SpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectStarshipSpawnButton {
    pub icon: StarshipIcon,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnButton {
    pub icon: StarshipIcon, // TODO AH Change to facility later
}

// In Game Selection

#[derive(Component, Clone, Copy)]
pub struct Selectable;

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct SelectedSprite {
    pub sprite_path: TeamSelectionSprite,
}

impl SelectedSprite {
    pub fn new(sprite_path: TeamSelectionSprite) -> Self {
        Self { sprite_path }
    }
}
