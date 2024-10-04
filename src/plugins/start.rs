use bevy::{
    app::{Plugin, Startup},
    prelude::IntoSystemConfigs,
};

use crate::systems::{
    camera::spawn_camera::spawn_camera,
    spawning::{
        new_spawn_starter_ship::spawn_starter_spaceship, spawn_player_base::spawn_space_stations,
        spawn_solar_system::spawn_sun, spawn_space::spawn_space,
    },
};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            (
                spawn_space,
                spawn_camera,
                spawn_sun,
                spawn_space_stations,
                spawn_starter_spaceship.after(spawn_space_stations),
            ),
        );
    }
}
