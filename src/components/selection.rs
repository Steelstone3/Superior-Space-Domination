use crate::assets::user_interace::team::TeamSelectionSprite;
use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub struct Selection {
    pub sprite_path: TeamSelectionSprite,
}

impl Selection {
    pub fn new(sprite_path: TeamSelectionSprite) -> Self {
        Self { sprite_path }
    }
}
