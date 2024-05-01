use bevy::app::{Plugin, Startup};

use crate::systems::{camera::spawn_camera::spawn_camera, spawning::spawn_space::spawn_space};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (spawn_space, spawn_camera));
    }
}
