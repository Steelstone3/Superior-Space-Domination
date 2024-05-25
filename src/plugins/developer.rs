use bevy::app::{Plugin, Startup, Update};

use crate::systems::{
    selecting::updated_select_space_station::select_selectable,
    spawning::updated_spawn_player_base::updated_spawn_space_stations,
};

pub struct DeveloperPlugin;

impl Plugin for DeveloperPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, select_selectable)
            .add_systems(Startup, updated_spawn_space_stations);
    }
}
