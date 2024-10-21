use super::size_component::SizeComponent;
use crate::{
    assets::images::space_facility_sprite::SpaceStationSprite,
    resources::constants::SPACE_TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2};

// TODO ships heal in a radius 4 times the size of the base (mechanic)

const SIZE: f32 = SPACE_TILE_SIZE / 2.0;
const SPACE_STATION_SIZE: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Copy)]
pub struct SpaceStation {
    pub sprite_path: SpaceStationSprite,
    pub size_component: SizeComponent,
}

impl SpaceStation {
    pub fn new(sprite_path: SpaceStationSprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_STATION_SIZE,
                z_index: 3.0,
            },
        }
    }
}
