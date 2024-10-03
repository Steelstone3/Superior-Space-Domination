use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::events::event_handlers::spawn_sprite_2::spawn_sprite_2;

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_sprite_2);
    }
}
