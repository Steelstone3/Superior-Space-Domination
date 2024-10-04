use bevy::prelude::Resource;

use crate::assets::{
    images::faction_starships::starship_sprite::StarshipSprite,
    user_interace::icons::starship_icons::StarshipIcon,
};

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFaction {
    #[allow(dead_code)]
    pub player_faction: Faction,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Faction {
    #[default]
    Atark,
    Karcan,
    Noozler,
}

impl Faction {
    pub fn determine_faction(starship_sprite: StarshipSprite) -> Faction {
        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Faction::Atark,
            StarshipSprite::AtarkBomber => Faction::Atark,
            StarshipSprite::AtarkDreadnought => Faction::Atark,
            StarshipSprite::AtarkFighter => Faction::Atark,
            StarshipSprite::AtarkFrigate => Faction::Atark,
            StarshipSprite::AtarkScout => Faction::Atark,
            StarshipSprite::AtarkSupportShip => Faction::Atark,
            StarshipSprite::AtarkTorpedoShip => Faction::Atark,
            StarshipSprite::KarcanBattleCruiser => Faction::Karcan,
            StarshipSprite::KarcanBomber => Faction::Karcan,
            StarshipSprite::KarcanDreadnought => Faction::Karcan,
            StarshipSprite::KarcanFighter => Faction::Karcan,
            StarshipSprite::KarcanFrigate => Faction::Karcan,
            StarshipSprite::KarcanScout => Faction::Karcan,
            StarshipSprite::KarcanSupportShip => Faction::Karcan,
            StarshipSprite::KarcanTorpedoShip => Faction::Karcan,
            StarshipSprite::NoozlerBattleCruiser => Faction::Noozler,
            StarshipSprite::NoozlerBomber => Faction::Noozler,
            StarshipSprite::NoozlerDreadnought => Faction::Noozler,
            StarshipSprite::NoozlerFighter => Faction::Noozler,
            StarshipSprite::NoozlerFrigate => Faction::Noozler,
            StarshipSprite::NoozlerScout => Faction::Noozler,
            StarshipSprite::NoozlerSupportShip => Faction::Noozler,
            StarshipSprite::NoozlerTorpedoShip => Faction::Noozler,
        }
    }
}

pub enum StarshipType {
    SupportShip,
    Scout,
    Fighter,
    TorpedoShip,
    Bomber,
    Frigate,
    Battlecruiser,
    Dreadnought,
}

impl StarshipType {
    pub fn convert_from(&self, faction: Faction) -> StarshipIcon {
        match faction {
            Faction::Atark => match self {
                StarshipType::SupportShip => StarshipIcon::AtarkSupportShip,
                StarshipType::Scout => StarshipIcon::AtarkScout,
                StarshipType::Fighter => StarshipIcon::AtarkFighter,
                StarshipType::TorpedoShip => StarshipIcon::AtarkTorpedoShip,
                StarshipType::Bomber => StarshipIcon::AtarkBomber,
                StarshipType::Frigate => StarshipIcon::AtarkFrigate,
                StarshipType::Battlecruiser => StarshipIcon::AtarkBattlecruiser,
                StarshipType::Dreadnought => StarshipIcon::AtarkDreadnought,
            },
            Faction::Karcan => match self {
                StarshipType::SupportShip => StarshipIcon::KarcanSupportShip,
                StarshipType::Scout => StarshipIcon::KarcanScout,
                StarshipType::Fighter => StarshipIcon::KarcanFighter,
                StarshipType::TorpedoShip => StarshipIcon::KarcanTorpedoShip,
                StarshipType::Bomber => StarshipIcon::KarcanBomber,
                StarshipType::Frigate => StarshipIcon::KarcanFrigate,
                StarshipType::Battlecruiser => StarshipIcon::KarcanBattlecruiser,
                StarshipType::Dreadnought => StarshipIcon::KarcanDreadnought,
            },
            Faction::Noozler => match self {
                StarshipType::SupportShip => StarshipIcon::NoozlerSupportShip,
                StarshipType::Scout => StarshipIcon::NoozlerScout,
                StarshipType::Fighter => StarshipIcon::NoozlerFighter,
                StarshipType::TorpedoShip => StarshipIcon::NoozlerTorpedoShip,
                StarshipType::Bomber => StarshipIcon::NoozlerBomber,
                StarshipType::Frigate => StarshipIcon::NoozlerFrigate,
                StarshipType::Battlecruiser => StarshipIcon::NoozlerBattlecruiser,
                StarshipType::Dreadnought => StarshipIcon::NoozlerDreadnought,
            },
        }
    }
}
