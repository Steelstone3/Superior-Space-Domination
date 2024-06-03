use bevy::{ecs::component::Component, math::Vec2};

use crate::{
    assets::images::space_facility_sprite::SpaceFacilitySprite, resources::constants::STATION_SIZE,
};

use super::size_component::SizeComponent;

#[allow(dead_code)]
const SPACE_FACILITY_SIZE: Vec2 = Vec2::new(STATION_SIZE, STATION_SIZE);

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct SpaceFacility {
    pub sprite_path: SpaceFacilitySprite,
    pub size_component: SizeComponent,
}

#[allow(dead_code)]
impl SpaceFacility {
    pub fn new(sprite_path: SpaceFacilitySprite) -> Self {
        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_FACILITY_SIZE,
                z_index: 3.0,
            },
        }
    }
}
