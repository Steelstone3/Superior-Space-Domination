use bevy::{
    ecs::query::{Changed, QueryData, QueryFilter},
    prelude::Entity,
    ui::{BorderColor, Interaction},
};

use crate::components::user_interface::{
    SelectStarshipSpawnButton, SelectStarshipSpawnMenuButton, SpawnSubMenu,
};

#[derive(QueryData)]
pub struct SpawnSubMenuQuery {
    pub entity: Entity,
    pub sub_menu: &'static SpawnSubMenu,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectStarshipSpawnMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub button: &'static SelectStarshipSpawnMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectStarshipSpawnButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub button: &'static SelectStarshipSpawnButton,
}

#[derive(QueryFilter)]
pub struct ButtonFilters {
    changed_interaction: Changed<Interaction>,
}
