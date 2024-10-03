// TODO AH (IDEA) A builder pattern for all sprite spawning events
// TODO Create a constructor method that can handle different senarios such as animated, static, needing a position of another entity, being selectable
// TODO Maybe an enum passed down that determines the type with a match statement in the event reader
pub mod event_handlers;
pub mod mouse_click_event;
pub mod spawn_animated_sprite_event;
// TODO try to make this generic with spawn animated sprite
pub mod spawn_planet_event;
pub mod spawn_sprite_event;
pub mod spawn_startership_event;
pub mod user_interface_event;
pub mod spawn_sprite_event_2;