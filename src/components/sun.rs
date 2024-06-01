use bevy::{ecs::component::Component, math::Vec2};

use crate::{assets::images::suns::SunSprite, resources::constants::SUN_TILE_SIZE};

use super::transform_component::TransformComponent;

const SUN_SPRITE_SIZE: Vec2 = Vec2::new(SUN_TILE_SIZE, SUN_TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Sun {
    pub sprite_path: SunSprite,
    pub transform: TransformComponent,
}

impl Sun {
    pub fn new(sprite_path: SunSprite) -> Self {
        Self {
            sprite_path,
            transform: TransformComponent {
                size: SUN_SPRITE_SIZE,
                z_index: 1.0,
            },
        }
    }
}
