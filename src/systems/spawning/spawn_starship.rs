use crate::{
    assets::user_interace::icons::starship_icons::StarshipIcon,
    components::{selectable::Selectable, starship::Starship},
    events::spawn_sprite_event::{SpawnSprite, SpawnSpriteEvent},
    queries::{
        camera_queries::CameraTransformOrthographicProjectionQuery, window_queries::WindowQuery,
    },
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::{
        controller::get_location::get_cursor_location,
        user_interface::interactions::spawn_selection::SpawnSelection,
    },
};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, Query, ResMut},
    },
    input::{mouse::MouseButton, ButtonInput},
    transform::components::Transform,
    utils::tracing,
};

pub fn spawn_starship(
    mut commands: Commands,
    selected_item: ResMut<SpawnMenuSelection>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_query: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.selection == SpawnSelection::None {
        return;
    }

    let Ok(window_query) = windows_query.get_single() else {
        return;
    };

    let Ok(camera_query) = camera_queries.get_single() else {
        return;
    };

    if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
        return;
    }

    let mut transform = Transform::default();
    transform.translation.z = 999.0;

    if let Some(position) = window_query.window.cursor_position() {
        get_cursor_location(&mut transform, position, window_query, camera_query);
    } else {
        return;
    }

    tracing::info!("starship at {:?}", transform.translation);

    if selected_item.starship_selection != StarshipIcon::None {
        let starship = Starship::new_from_icon(selected_item.starship_selection);

        spawn_sprite_event.send(SpawnSpriteEvent::spawn_sprite(SpawnSprite {
            sprite_path: starship.starship_sprite_bundle.starship_sprite.to_string(),
            size: starship.size_component.size,
            transform,
            entity: commands.spawn(starship).insert(Selectable).id(),
        }));
    }
}
