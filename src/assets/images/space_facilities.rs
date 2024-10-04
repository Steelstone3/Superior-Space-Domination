use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceStationSprite {
    AtarkSpaceStation,
    KarcanSpaceStation,
    NoozlerSpaceStation,
}

impl Display for SpaceStationSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceStationSprite::AtarkSpaceStation => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_space_station.png"
                )
            }
            SpaceStationSprite::KarcanSpaceStation => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_space_station.png"
                )
            }
            SpaceStationSprite::NoozlerSpaceStation => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_space_station.png"
                )
            }
        }
    }
}

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum SpaceFacilitySprite {
    AtarkSpaceShipConstructionYard,
    KarcanSpaceShipConstructionYard,
    NoozlerSpaceShipConstructionYard,
}

impl Display for SpaceFacilitySprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceFacilitySprite::AtarkSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/atark/space_facilities/atark_spaceship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::KarcanSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/karcan/space_facilities/karcan_spaceship_construction_yard.png"
                )
            }
            SpaceFacilitySprite::NoozlerSpaceShipConstructionYard => {
                write!(
                    formatter,
                    "images/factions/noozler/space_facilities/noozler_spaceship_construction_yard.png"
                )
            }
        }
    }
}
