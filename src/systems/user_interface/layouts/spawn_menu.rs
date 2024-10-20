use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::user_interace::icons::commander_icons::CommanderIcon,
    components::user_interface::{SpawnMenu, SpawnMenuButton},
    resources::{
        constants::TILE_SIZE,
        faction::{PlayerFaction, SpaceFacilityType, StarshipType},
    },
    systems::user_interface::styles::{
        create_space_facility_button_bundle, create_space_facility_button_icon,
        create_starship_button_bundle, create_starship_button_icon,
    },
};

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    faction: Res<PlayerFaction>,
) {
    const ICON_SIZE: f32 = TILE_SIZE * 2.0;

    // Commander Button
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::flex(1.0)],
                width: Val::Px(ICON_SIZE * 1.0),
                height: Val::Px(ICON_SIZE * 1.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(SpawnMenu)
        .with_children(|parent| {
            // Starships
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border: UiRect::new(
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                                Val::Px(2.0),
                            ),

                            ..Default::default()
                        },
                        border_color: Color::srgb(189.0, 189.0, 189.0).into(),
                        ..Default::default()
                    },
                    SpawnMenuButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(
                            asset_server.load(
                                CommanderIcon::convert_from(faction.player_faction).to_string(),
                            ),
                        ),
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
        });

    let columns = 2.0;
    let rows = 7.0;

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
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                    GridTrack::flex(1.0),
                ],
                width: Val::Px(ICON_SIZE * columns),
                height: Val::Px(ICON_SIZE * rows),
                position_type: PositionType::Absolute,
                left: Val::Px(ICON_SIZE),
                top: Val::Percent(0.0),
                ..Default::default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(SpawnMenu)
        // Support Ship
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::SupportShip,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::SupportShip,
                        faction.player_faction,
                    ));
                });
        })
        // Scout
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::Scout,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::Scout,
                        faction.player_faction,
                    ));
                });
        })
        // Fighter
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::Fighter,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::Fighter,
                        faction.player_faction,
                    ));
                });
        })
        // Torpedo Ship
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::TorpedoShip,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::TorpedoShip,
                        faction.player_faction,
                    ));
                });
        })
        // Bomber
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::Bomber,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::Bomber,
                        faction.player_faction,
                    ));
                });
        })
        // Frigate
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::Frigate,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::Frigate,
                        faction.player_faction,
                    ));
                });
        })
        // Battlecruiser
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::BattleCruiser,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::BattleCruiser,
                        faction.player_faction,
                    ));
                });
        })
        // Dreadnought
        .with_children(|parent| {
            parent
                .spawn(create_starship_button_bundle(
                    StarshipType::Dreadnought,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_starship_button_icon(
                        &asset_server,
                        StarshipType::Dreadnought,
                        faction.player_faction,
                    ));
                });
        })
        // Starship Construction Yard
        .with_children(|parent| {
            parent
                .spawn(create_space_facility_button_bundle(
                    SpaceFacilityType::SpaceShipConstructionYard,
                    faction.player_faction,
                ))
                .with_children(|parent| {
                    parent.spawn(create_space_facility_button_icon(
                        &asset_server,
                        SpaceFacilityType::SpaceShipConstructionYard,
                        faction.player_faction,
                    ));
                });
        });
}
