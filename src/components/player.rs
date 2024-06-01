use super::faction::Faction;
use bevy::ecs::component::Component;

#[derive(Component)]
#[allow(dead_code)]
pub struct Player {
    pub faction: Faction,
}
