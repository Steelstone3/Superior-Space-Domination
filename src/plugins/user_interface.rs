use bevy::app::{Plugin, Startup, Update};

use crate::systems::user_interface::{
    interactions::{
        select_starship_spawn_button::select_starship_spawn_button,
        select_starship_spawn_menu_button::select_starship_spawn_menu_button,
    },
    layouts::{
        despawn_spawn_sub_menu::despawn_sub_menus, spawn_menu::spawn_menu,
        spawn_starship_menu::spawn_starship_sub_menu,
    },
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_menu);
        app.add_systems(Update, spawn_starship_sub_menu);
        app.add_systems(
            Update,
            (
                select_starship_spawn_menu_button,
                select_starship_spawn_button,
                despawn_sub_menus,
            ),
        );
    }
}
