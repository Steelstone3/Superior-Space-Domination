use super::size_component::SizeComponent;
use crate::{
    assets::images::faction_starship_sprite::{
        starship_sprite::StarshipSprite, weapon_sprite::WeaponSprite,
    },
    resources::{constants::TILE_SIZE, faction::Faction},
};
use bevy::{ecs::component::Component, math::Vec2};

const SIZE: f32 = TILE_SIZE;

#[allow(dead_code)]
#[derive(Component)]
pub struct Weapon {
    weapon_sprite: WeaponSprite,
    size: SizeComponent,
    faction: Faction,
}
impl Weapon {
    pub fn new(starship_sprite: StarshipSprite) -> Self {
        let size = SizeComponent {
            size: Vec2::new(SIZE, SIZE),
            z_index: 4.0,
        };

        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser => Self {
                weapon_sprite: WeaponSprite::AtarkBattleCruiser,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkBomber => Self {
                weapon_sprite: WeaponSprite::AtarkBomber,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkDreadnought => Self {
                weapon_sprite: WeaponSprite::AtarkDreadnought,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkFighter => Self {
                weapon_sprite: WeaponSprite::AtarkFighter,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkFrigate => Self {
                weapon_sprite: WeaponSprite::AtarkFrigate,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkScout => Self {
                weapon_sprite: WeaponSprite::AtarkScout,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkSupportShip => Self {
                weapon_sprite: WeaponSprite::AtarkSupportShip,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::AtarkTorpedoShip => Self {
                weapon_sprite: WeaponSprite::AtarkTorpedoShip,
                size,
                faction: Faction::Atark,
            },
            StarshipSprite::KarcanBattleCruiser => Self {
                weapon_sprite: WeaponSprite::KarcanBattleCruiser,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanBomber => Self {
                weapon_sprite: WeaponSprite::KarcanBomber,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanDreadnought => Self {
                weapon_sprite: WeaponSprite::KarcanDreadnought,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanFighter => Self {
                weapon_sprite: WeaponSprite::KarcanFighter,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanFrigate => Self {
                weapon_sprite: WeaponSprite::KarcanFrigate,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanScout => Self {
                weapon_sprite: WeaponSprite::KarcanScout,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanSupportShip => Self {
                weapon_sprite: WeaponSprite::KarcanSupportShip,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::KarcanTorpedoShip => Self {
                weapon_sprite: WeaponSprite::KarcanTorpedoShip,
                size,
                faction: Faction::Karcan,
            },
            StarshipSprite::NoozlerBattleCruiser => Self {
                weapon_sprite: WeaponSprite::NoozlerBattleCruiser,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerBomber => Self {
                weapon_sprite: WeaponSprite::NoozlerBomber,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerDreadnought => Self {
                weapon_sprite: WeaponSprite::NoozlerDreadnought,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerFighter => Self {
                weapon_sprite: WeaponSprite::NoozlerFighter,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerFrigate => Self {
                weapon_sprite: WeaponSprite::NoozlerFrigate,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerScout => Self {
                weapon_sprite: WeaponSprite::NoozlerScout,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerSupportShip => Self {
                weapon_sprite: WeaponSprite::NoozlerSupportShip,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::NoozlerTorpedoShip => Self {
                weapon_sprite: WeaponSprite::NoozlerTorpedoShip,
                size,
                faction: Faction::Noozler,
            },
            // TODO Weapon sprites
            StarshipSprite::GarnokBattleCruiser => Self {
                weapon_sprite: WeaponSprite::GarnokBattleCruiser,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokBomber => Self {
                weapon_sprite: WeaponSprite::GarnokBomber,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokDreadnought => Self {
                weapon_sprite: WeaponSprite::GarnokDreadnought,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokFighter => Self {
                weapon_sprite: WeaponSprite::GarnokFighter,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokFrigate => Self {
                weapon_sprite: WeaponSprite::GarnokFrigate,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokScout => Self {
                weapon_sprite: WeaponSprite::GarnokScout,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokSupportShip => Self {
                weapon_sprite: WeaponSprite::GarnokSupportShip,
                size,
                faction: Faction::Noozler,
            },
            StarshipSprite::GarnokTorpedoShip => Self {
                weapon_sprite: WeaponSprite::GarnokTorpedoShip,
                size,
                faction: Faction::Noozler,
            },
        }
    }
}
