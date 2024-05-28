use bevy::app::{Plugin, Update};

use crate::{
    events::event_handlers::spawn_animated_sprite::spawn_animated_sprite,
    systems::animation::animate::animate_sprites,
};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        _app.add_systems(Update, (animate_sprites, spawn_animated_sprite));
    }
}
