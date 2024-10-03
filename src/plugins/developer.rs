use bevy::app::{Plugin, Update};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_systems(Update, new_spawn_starter_ship);
    }
}
