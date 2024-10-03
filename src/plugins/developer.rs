use bevy::app::{Plugin, Update};

use crate::events::event_handlers::new_spawn_starter_ship::new_spawn_starter_ship;

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, new_spawn_starter_ship);
    }
}
