use bevy::{
    color::palettes::css::{GREY, YELLOW},
    prelude::{EventWriter, Query, ResMut},
    ui::Interaction,
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectStarshipSpawnMenuButtonQuery},
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::interactions::spawn_selection::SpawnSelection,
};

#[allow(dead_code)]
pub fn select_animal_button(
    mut select_starship_spawn_menu_button_queries: Query<
        SelectStarshipSpawnMenuButtonQuery,
        ButtonFilters,
    >,
    mut selected_item: ResMut<SpawnMenuSelection>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_starship_spawn_menu_button_query) =
        select_starship_spawn_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_starship_spawn_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Ships");

            SpawnMenuSelection::reset(&mut selected_item);

            selected_item.selection = SpawnSelection::Ships;

            *select_starship_spawn_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Ships");

            *select_starship_spawn_menu_button_query.border_color = YELLOW.into();
        }
        Interaction::None => {
            *select_starship_spawn_menu_button_query.border_color = GREY.into();
        }
    }
}
