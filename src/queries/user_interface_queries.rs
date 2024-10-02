use bevy::{
    ecs::query::{Changed, QueryData, QueryFilter},
    ui::{BorderColor, Interaction},
};

use crate::components::user_interface::{SelectStarshipSpawnButton, SelectStarshipSpawnMenuButton};

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
