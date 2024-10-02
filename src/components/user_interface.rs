use super::starship::Starship;
use bevy::prelude::Component;

#[derive(Component)]
pub struct SelectStarshipSpawnMenuButton;

#[derive(Component)]
pub struct SelectStarshipSpawnButton {
    pub starship: Starship,
}

#[derive(Component)]
pub struct SelectFacilitySpawnMenuButton;

#[derive(Component)]
pub struct SelectFacilitySpawnButton {
    pub starship: Starship,
}
