use bevy::prelude::{App, Plugin};

use crate::events::{
    mouse_click_event::MouseClickEvent, spawn_planet_event::SpawnPlanetEvent, spawn_sprite_event_2::SpawnSpriteEvent2, spawn_startership_event::SpawnStarterShipEvent, user_interface_event::UserInterfaceEvent
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent2>();
        app.add_event::<SpawnPlanetEvent>();
        app.add_event::<MouseClickEvent>();
        app.add_event::<UserInterfaceEvent>();
        // TODO DEVELOPMENT EVENT
        app.add_event::<SpawnStarterShipEvent>();
    }
}
