use super::size_component::SizeComponent;
use crate::{
    assets::{
        images::space_facility_sprite::SpaceFacilitySprite,
        user_interface::icons::space_facility_icons::SpaceFacilityIcon,
    },
    resources::constants::SPACE_TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2};

const SIZE: f32 = SPACE_TILE_SIZE / 2.0;
const SPACE_FACILITY_SIZE: Vec2 = Vec2::new(SIZE, SIZE);

#[derive(Component, Clone, Copy)]
pub struct SpaceFacility {
    pub sprite_path: SpaceFacilitySprite,
    pub size_component: SizeComponent,
}

impl SpaceFacility {
    pub fn new(space_facility_sprite: SpaceFacilitySprite) -> SpaceFacility {
        Self {
            sprite_path: space_facility_sprite,
            size_component: SizeComponent {
                size: SPACE_FACILITY_SIZE,
                z_index: 3.0,
            },
        }
    }

    pub fn new_from_icon(space_facility_icon: SpaceFacilityIcon) -> SpaceFacility {
        let sprite_path = SpaceFacilitySprite::sprite_convert_from(space_facility_icon);

        Self {
            sprite_path,
            size_component: SizeComponent {
                size: SPACE_FACILITY_SIZE,
                z_index: 3.0,
            },
        }
    }
}
