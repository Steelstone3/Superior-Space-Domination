use crate::{
    events::event_handlers::select::handle_mouse_input,
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        controller::new_set_controllable_target::new_set_controllable_target,
        movement::{
            new_controllable_move_to_target::new_controllable_move_to_target,
            new_update_selection_sprite_location::new_update_selection_sprite_location,
        },
        user_interface::sprite_selection::sprite_selection,
    },
};
use bevy::app::{Plugin, Update};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                camera_zoom_keyboard,
                camera_zoom_mouse_and_touchpad,
                camera_movement,
                camera_position_reset,
                handle_mouse_input,
                sprite_selection,
                animate_sprites,
                new_set_controllable_target,
                new_controllable_move_to_target,
                new_update_selection_sprite_location,
            ),
        );
    }
}
