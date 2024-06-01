use crate::{assets::images::space::SpaceSprite, resources::constants::SPACE_TILE_SIZE};
use bevy::{ecs::component::Component, math::Vec2};

use super::transform_component::TransformComponent;

const SPACE_SIZE: Vec2 = Vec2::new(SPACE_TILE_SIZE, SPACE_TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Space {
    pub sprite_path: SpaceSprite,
    pub transform: TransformComponent,
}

impl Space {
    pub fn new(sprite_path: SpaceSprite) -> Self {
        Self {
            sprite_path,
            transform: TransformComponent {
                size: SPACE_SIZE,
                z_index: 0.0,
            },
        }
    }
}
