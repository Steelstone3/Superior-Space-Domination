use bevy::ecs::component::Component;

use super::starship::Starship;

#[derive(Component)]
#[allow(dead_code)]
pub struct Faction {
    pub starships: [Starship; 8],
}
