use bevy::{ecs::event::Event, transform::components::Transform};

#[derive(Event)]
pub struct SpawnedSunEvent {
    pub sun_transform: Transform,
}

#[derive(Event)]
pub struct SpawnedStarbaseEvent {
    pub starbase_transform: Transform,
}
