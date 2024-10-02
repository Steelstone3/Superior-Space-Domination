use bevy::app::{Plugin, Startup};

use crate::systems::user_interface::layouts::spawn_menu::spawn_menu;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_menu);
    }
}
