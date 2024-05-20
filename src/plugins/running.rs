use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    events::spawn_planet_event::SpawnPlanetEvent,
    systems::{
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
        spawning::spawn_resource_planets::spawn_resource_planets,
    },
};

use super::run_conditions::event_called;

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
            ),
        )
        .add_systems(
            Update,
            spawn_resource_planets.run_if(event_called::<SpawnPlanetEvent>),
        );
    }
}
