use super::space_facility_sprite::SpaceStationSprite;
use crate::resources::faction::Faction;

pub enum SpaceStationType {
    SpaceStation,
}

impl SpaceStationType {
    pub fn sprite_convert_from(&self, faction: Faction) -> SpaceStationSprite {
        match faction {
            Faction::Atark => match self {
                SpaceStationType::SpaceStation => SpaceStationSprite::AtarkSpaceStation,
            },
            Faction::Karcan => match self {
                SpaceStationType::SpaceStation => SpaceStationSprite::KarcanSpaceStation,
            },
            Faction::Noozler => match self {
                SpaceStationType::SpaceStation => SpaceStationSprite::NoozlerSpaceStation,
            },
            Faction::Granok => match self {
                SpaceStationType::SpaceStation => SpaceStationSprite::GranokSpaceStation,
            },
        }
    }
}
