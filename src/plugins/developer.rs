use bevy::{
    app::{Plugin, Startup},
    prelude::IntoSystemConfigs,
};

use crate::systems::spawning::{
    new_spawn_starter_ship::spawn_starter_spaceship, spawn_player_base::spawn_space_stations,
};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
    }
}
