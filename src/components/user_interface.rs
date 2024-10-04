use crate::assets::user_interace::icons::starship_icons::StarshipIcon;
use bevy::prelude::Component;

#[derive(Component)]
pub struct SpawnMenu;

#[derive(Component)]
pub struct SpawnSubMenu;

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
