use crate::{assets::images::planets::PlanetSprite, resources::constants::PLANET_SIZE};
use bevy::{ecs::component::Component, math::Vec2};

const PLANET_SPRITE_SIZE: Vec2 = Vec2::new(PLANET_SIZE, PLANET_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Planet {
    pub sprite_path: PlanetSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl Planet {
    pub fn new(sprite_path: PlanetSprite) -> Self {
        Self {
            sprite_path,
            size: PLANET_SPRITE_SIZE,
            z_index: 0.0,
        }
    }
}
