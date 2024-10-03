use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
    transform::components::Transform,
    utils::default,
};

#[derive(PartialEq)]
pub enum SpriteType {
    Static,
    Animated,
}

#[derive(Event)]
pub struct SpawnSpriteEvent2 {
    pub sprite_type: SpriteType,
    pub spawn_sprite: SpawnSprite,
    pub spawn_animated_sprite: SpawnAnimatedSprite,
    pub transform_dependent: TransformDependency,
}

impl SpawnSpriteEvent2 {
    pub fn spawn_sprite(spawn_sprite: SpawnSprite) -> Self {
        let transform = spawn_sprite.transform;

        Self {
            sprite_type: SpriteType::Static,
            spawn_sprite,
            spawn_animated_sprite: default(),
            transform_dependent: TransformDependency::new(transform),
        }
    }

    pub fn spawn_animated_sprite(
        spawn_sprite: SpawnSprite,
        spawn_animated_sprite: SpawnAnimatedSprite,
    ) -> Self {
        let transform = spawn_sprite.transform;

        Self {
            sprite_type: SpriteType::Animated,
            spawn_sprite,
            spawn_animated_sprite,
            transform_dependent: TransformDependency::new(transform),
        }
    }

    pub fn add_transform_dependency(
        spawn_sprite_event: SpawnSpriteEvent2,
        transform: Transform,
    ) -> Self {
        Self {
            sprite_type: spawn_sprite_event.sprite_type,
            spawn_sprite: spawn_sprite_event.spawn_sprite,
            spawn_animated_sprite: spawn_sprite_event.spawn_animated_sprite,
            transform_dependent: TransformDependency::new_with_dependency(transform),
        }
    }
}

pub struct SpawnSprite {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}

pub struct SpawnAnimatedSprite {
    pub sprite_tile_size: u32,
    pub frame_timing: f32,
    pub frame_count: usize,
}

impl Default for SpawnAnimatedSprite {
    fn default() -> Self {
        Self {
            sprite_tile_size: Default::default(),
            frame_timing: Default::default(),
            frame_count: Default::default(),
        }
    }
}

pub struct TransformDependency {
    pub is_transform_dependent: bool,
    pub entity_transform: Transform,
}

impl TransformDependency {
    pub fn new(transform: Transform) -> Self {
        Self {
            is_transform_dependent: false,
            entity_transform: transform,
        }
    }

    pub fn new_with_dependency(transform: Transform) -> Self {
        Self {
            is_transform_dependent: true,
            entity_transform: transform,
        }
    }
}
