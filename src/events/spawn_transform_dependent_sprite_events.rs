use bevy::{ecs::event::Event, transform::components::Transform};

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub sun_transform: Transform,
}

#[derive(Event)]
pub struct SpawnStarterSpaceshipEvent {
    pub starbase_transform: Transform,
}
