use crate::components::user_interface::{
    SelectFacilitySpawnButton, SelectStarshipSpawnButton, SpawnMenuButton,
};
use bevy::{
    ecs::query::{Changed, QueryData, QueryFilter},
    ui::{BorderColor, Interaction},
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectSpawnMenuButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub button: &'static SpawnMenuButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectStarshipSpawnButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub button: &'static SelectStarshipSpawnButton,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SelectSpaceFacilitySpawnButtonQuery {
    pub interaction: &'static Interaction,
    pub border_color: &'static mut BorderColor,
    pub button: &'static SelectFacilitySpawnButton,
}

#[derive(QueryFilter)]
pub struct ButtonFilters {
    changed_interaction: Changed<Interaction>,
}
