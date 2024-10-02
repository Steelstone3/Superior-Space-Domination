use super::starship::Starship;
use bevy::prelude::Component;

#[derive(Component)]
pub struct SelectStarshipSpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectStarshipSpawnButton {
    pub starship: Starship,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnButton {
    pub starship: Starship,
}
