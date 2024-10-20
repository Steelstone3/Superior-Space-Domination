use bevy::{
    ecs::query::QueryData,
    prelude::{Entity, Transform},
    sprite::Sprite,
};

use crate::components::user_interface::{Selectable, SelectedSprite};

#[derive(QueryData)]
pub struct SelectableQuery {
    pub transform: &'static Transform,
    pub sprite: &'static Sprite,
    pub entity: Entity,
    pub selectable: &'static Selectable,
}

// TODO Make selected optional?
#[derive(QueryData)]
pub struct SelectionQuery {
    pub entity: Option<Entity>,
    pub selected: &'static SelectedSprite,
}
