use super::{
    starship_sprite_bundle::StarshipSpriteBundle, transform_component::TransformComponent,
};
use crate::{
    assets::images::faction_starships::starships::StarshipSprite, resources::constants::TILE_SIZE,
};
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
#[allow(dead_code)]
pub struct Starship {
    pub starship_sprite_bundle: StarshipSpriteBundle,
    pub transform: TransformComponent,
}

impl Starship {
    #[allow(dead_code)]
    pub fn new(starship_sprite: StarshipSprite) -> Starship {
        Self {
            starship_sprite_bundle: StarshipSpriteBundle::new(starship_sprite),
            transform: TransformComponent {
                size: Vec2::new(TILE_SIZE, TILE_SIZE),
                z_index: 4.0,
            },
        }
    }
}
