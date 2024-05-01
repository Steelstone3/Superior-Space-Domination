use bevy::prelude::{App, Plugin};

use crate::events::spawn_sprite_event::SpawnSpriteEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpriteEvent>();
    }
}
