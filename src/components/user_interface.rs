use crate::assets::user_interace::icons::starships::atark_icons::AtarkIcon;
use bevy::prelude::Component;

#[derive(Component)]
pub struct SpawnMenu;

#[derive(Component)]
pub struct SelectStarshipSpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectStarshipSpawnButton {
    pub icon: AtarkIcon,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnMenuButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct SelectFacilitySpawnButton {
    pub icon: AtarkIcon,
}
