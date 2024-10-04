use bevy::{
    app::{Plugin, Startup},
    prelude::IntoSystemConfigs,
};

use crate::systems::spawning::{
    new_spawn_starter_ship::new_spawn_starter_ship, spawn_player_base::spawn_space_stations,
};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, new_spawn_starter_ship.after(spawn_space_stations));
    }
}
