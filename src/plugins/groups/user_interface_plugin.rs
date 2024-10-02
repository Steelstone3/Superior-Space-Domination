use bevy::app::{Plugin, Startup, Update};

use crate::systems::user_interface::layouts::{
    spawn_menu::spawn_menu, spawn_starship::spawn_starship,
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_menu);
        // app.add_systems(Update, spawn_starship);
    }
}
