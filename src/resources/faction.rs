use crate::assets::images::faction_starship_sprite::starship_sprite::StarshipSprite;
use bevy::prelude::Resource;

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerFaction {
    pub player_faction: Faction,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Faction {
    Atark,
    #[default]
    Karcan,
    Noozler,
    Garnok,
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
            StarshipSprite::GarnokBattleCruiser => Faction::Garnok,
            StarshipSprite::GarnokBomber => Faction::Garnok,
            StarshipSprite::GarnokDreadnought => Faction::Garnok,
            StarshipSprite::GarnokFighter => Faction::Garnok,
            StarshipSprite::GarnokFrigate => Faction::Garnok,
            StarshipSprite::GarnokScout => Faction::Garnok,
            StarshipSprite::GarnokSupportShip => Faction::Garnok,
            StarshipSprite::GarnokTorpedoShip => Faction::Garnok,
        }
    }
}
