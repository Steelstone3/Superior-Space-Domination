use crate::{
    events::event_handlers::select::select,
    systems::{
        animation::animate::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        selecting::select_selectable::select_selectable,
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
                select,
                select_selectable,
                animate_sprites,
            ),
        );
    }
}
