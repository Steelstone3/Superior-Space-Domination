use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, Val},
};

use crate::{
    assets::user_interace::icons::starship_icons::StarshipIcon,
    components::user_interface::SpawnSubMenuButton,
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SpawnSubMenuQuery,
    resources::spawn_menu_selection::SpawnMenuSelection,
    systems::user_interface::{
        interactions::spawn_selection::Selection,
        styles::{create_starship_button_bundle, create_starship_button_icon},
    },
};

pub fn spawn_starship_sub_menu(
    selected_item: Res<SpawnMenuSelection>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SpawnSubMenuQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.selection == Selection::StarshipConstructionYard {
        for _ in user_interface_events.read() {
            // Remove UI
            if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                commands.entity(sub_menu_query.entity).despawn_recursive();
            }

            // Spawn UI
            commands
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                        grid_template_rows: vec![
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                        ],
                        width: Val::Px(64.0 * 2.0),
                        height: Val::Px(64.0 * 4.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(64.0),
                        top: Val::Percent(0.0),
                        ..Default::default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SpawnSubMenuButton)
                // // Support Ship
                // .with_children(|parent| {
                //     parent
                //         .spawn(create_starship_button_bundle(
                //             StarshipIcon::AtarkSupportShip,
                //         ))
                //         .with_children(|parent| {
                //             parent.spawn(create_starship_button_icon(
                //                 &asset_server,
                //                 StarshipIcon::AtarkSupportShip,
                //             ));
                //         });
                // })
                // // Scout
                // .with_children(|parent| {
                //     parent
                //         .spawn(create_starship_button_bundle(StarshipIcon::AtarkScout))
                //         .with_children(|parent| {
                //             parent.spawn(create_starship_button_icon(
                //                 &asset_server,
                //                 StarshipIcon::AtarkScout,
                //             ));
                //         });
                // })
                // Fighter
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(StarshipIcon::AtarkFighter))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkFighter,
                            ));
                        });
                })
                // Torpedo Ship
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(
                            StarshipIcon::AtarkTorpedoShip,
                        ))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkTorpedoShip,
                            ));
                        });
                })
                // Bomber
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(StarshipIcon::AtarkBomber))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkBomber,
                            ));
                        });
                })
                // Frigate
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(StarshipIcon::AtarkFrigate))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkFrigate,
                            ));
                        });
                })
                // Battlecruiser
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(
                            StarshipIcon::AtarkBattlecruiser,
                        ))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkBattlecruiser,
                            ));
                        });
                })
                // Dreadnought
                .with_children(|parent| {
                    parent
                        .spawn(create_starship_button_bundle(
                            StarshipIcon::AtarkDreadnought,
                        ))
                        .with_children(|parent| {
                            parent.spawn(create_starship_button_icon(
                                &asset_server,
                                StarshipIcon::AtarkDreadnought,
                            ));
                        });
                });
        }
    }
}
