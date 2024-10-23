use super::faction_starship_sprite::starship_sprite::StarshipSprite;
use crate::{
    assets::user_interface::icons::starship_icons::StarshipIcon, resources::faction::Faction,
};
use std::fmt::Display;

#[derive(PartialEq)]
pub enum StarshipType {
    SupportShip,
    Scout,
    Fighter,
    TorpedoShip,
    Bomber,
    Frigate,
    BattleCruiser,
    Dreadnought,
}

impl Display for StarshipType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipType::SupportShip => write!(formatter, "Support"),
            StarshipType::Scout => write!(formatter, "Scout"),
            StarshipType::Fighter => write!(formatter, "Fighter"),
            StarshipType::TorpedoShip => write!(formatter, "Torpedo"),
            StarshipType::Bomber => write!(formatter, "Bomber"),
            StarshipType::Frigate => write!(formatter, "Frigate"),
            StarshipType::BattleCruiser => write!(formatter, "BattleCruiser"),
            StarshipType::Dreadnought => write!(formatter, "Dreadnought"),
        }
    }
}

impl StarshipType {
    pub fn icon_convert_from(&self, faction: Faction) -> StarshipIcon {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipIcon::AtarkSupportShip,
                StarshipType::Scout => StarshipIcon::AtarkScout,
                StarshipType::Fighter => StarshipIcon::AtarkFighter,
                StarshipType::TorpedoShip => StarshipIcon::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipIcon::AtarkBomber,
                StarshipType::Frigate => StarshipIcon::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipIcon::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipIcon::KarcanSupportShip,
                StarshipType::Scout => StarshipIcon::KarcanScout,
                StarshipType::Fighter => StarshipIcon::KarcanFighter,
                StarshipType::TorpedoShip => StarshipIcon::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipIcon::KarcanBomber,
                StarshipType::Frigate => StarshipIcon::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipIcon::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipIcon::NoozlerSupportShip,
                StarshipType::Scout => StarshipIcon::NoozlerScout,
                StarshipType::Fighter => StarshipIcon::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipIcon::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipIcon::NoozlerBomber,
                StarshipType::Frigate => StarshipIcon::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipIcon::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::NoozlerDreadnought,
            },
            Faction::Garnok => match self {
                StarshipType::SupportShip => StarshipIcon::GarnokSupportShip,
                StarshipType::Scout => StarshipIcon::GarnokScout,
                StarshipType::Fighter => StarshipIcon::GarnokFighter,
                StarshipType::TorpedoShip => StarshipIcon::GarnokTorpedoShip,
                StarshipType::Bomber => StarshipIcon::GarnokBomber,
                StarshipType::Frigate => StarshipIcon::GarnokFrigate,
                StarshipType::BattleCruiser => StarshipIcon::GarnokBattleCruiser,
                StarshipType::Dreadnought => StarshipIcon::GarnokDreadnought,
            },
        }
    }

    pub fn sprite_convert_from(&self, faction: Faction) -> StarshipSprite {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipSprite::AtarkSupportShip,
                StarshipType::Scout => StarshipSprite::AtarkScout,
                StarshipType::Fighter => StarshipSprite::AtarkFighter,
                StarshipType::TorpedoShip => StarshipSprite::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipSprite::AtarkBomber,
                StarshipType::Frigate => StarshipSprite::AtarkFrigate,
                StarshipType::BattleCruiser => StarshipSprite::AtarkBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipSprite::KarcanSupportShip,
                StarshipType::Scout => StarshipSprite::KarcanScout,
                StarshipType::Fighter => StarshipSprite::KarcanFighter,
                StarshipType::TorpedoShip => StarshipSprite::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipSprite::KarcanBomber,
                StarshipType::Frigate => StarshipSprite::KarcanFrigate,
                StarshipType::BattleCruiser => StarshipSprite::KarcanBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipSprite::NoozlerSupportShip,
                StarshipType::Scout => StarshipSprite::NoozlerScout,
                StarshipType::Fighter => StarshipSprite::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipSprite::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipSprite::NoozlerBomber,
                StarshipType::Frigate => StarshipSprite::NoozlerFrigate,
                StarshipType::BattleCruiser => StarshipSprite::NoozlerBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::NoozlerDreadnought,
            },
            Faction::Garnok => match self {
                StarshipType::SupportShip => StarshipSprite::GarnokSupportShip,
                StarshipType::Scout => StarshipSprite::GarnokScout,
                StarshipType::Fighter => StarshipSprite::GarnokFighter,
                StarshipType::TorpedoShip => StarshipSprite::GarnokTorpedoShip,
                StarshipType::Bomber => StarshipSprite::GarnokBomber,
                StarshipType::Frigate => StarshipSprite::GarnokFrigate,
                StarshipType::BattleCruiser => StarshipSprite::GarnokBattleCruiser,
                StarshipType::Dreadnought => StarshipSprite::GarnokDreadnought,
            },
        }
    }
}
