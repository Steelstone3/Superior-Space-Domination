use bevy::app::{Plugin, Startup};

use crate::systems::user_interface::layouts::spawn_menu::spawn_menu;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_menu);
        // app.add_systems(Update, spawn_starship_sub_menu);
        // app.add_systems(
        //     Update,
        //     (
        //         select_starship_spawn_menu_button,
        //         select_starship_spawn_button,
        //     ),
        // );
    }
}
