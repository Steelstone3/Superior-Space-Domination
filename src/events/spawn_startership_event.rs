use bevy::{prelude::Event, transform::components::Transform};

#[derive(Event)]
pub struct SpawnStarterShipEvent {
    pub transform: Transform,
}
