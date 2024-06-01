use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Clone, Copy)]
pub struct TransformComponent {
    pub size: Vec2,
    pub z_index: f32,
}
