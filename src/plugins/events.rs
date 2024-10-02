use bevy::prelude::{App, Plugin};

use crate::events::{
    mouse_click_event::MouseClickEvent, spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
    spawn_planet_event::SpawnPlanetEvent, spawn_sprite_event::SpawnSpriteEvent,
    spawn_startership_event::SpawnStarterShipEvent, user_interface_event::UserInterfaceEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent>();
        app.add_event::<SpawnAnimatedSpriteEvent>();
        // TODO try and make this generic to use spawn animate sprite event
        app.add_event::<SpawnPlanetEvent>();
        app.add_event::<MouseClickEvent>();
        // TODO DEVELOPMENT EVENT
        app.add_event::<UserInterfaceEvent>();
        app.add_event::<SpawnStarterShipEvent>();
    }
}
