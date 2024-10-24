use crate::assets::{
    images::starship_type::StarshipType, user_interface::icons::starship_icons::StarshipIcon,
};
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum StarshipSprite {
    AtarkBattleCruiser,
    AtarkBomber,
    AtarkDreadnought,
    AtarkFighter,
    AtarkFrigate,
    AtarkScout,
    AtarkSupportShip,
    AtarkTorpedoShip,
    KarcanBattleCruiser,
    KarcanBomber,
    KarcanDreadnought,
    KarcanFighter,
    KarcanFrigate,
    KarcanScout,
    KarcanSupportShip,
    KarcanTorpedoShip,
    NoozlerBattleCruiser,
    NoozlerBomber,
    NoozlerDreadnought,
    NoozlerFighter,
    NoozlerFrigate,
    NoozlerScout,
    NoozlerSupportShip,
    NoozlerTorpedoShip,
    GranokBattleCruiser,
    GranokBomber,
    GranokDreadnought,
    GranokFighter,
    GranokFrigate,
    GranokScout,
    GranokSupportShip,
    GranokTorpedoShip,
}

impl Display for StarshipSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            StarshipSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/bomber/atark_bomber.png"
            ),
            StarshipSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/dreadnought/atark_dreadnought.png"
            ),
            StarshipSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/fighter/atark_fighter.png"
            ),
            StarshipSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/frigate/atark_frigate.png"
            ),
            StarshipSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/scout/atark_scout.png"
            ),
            StarshipSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/support_ship/atark_support_ship.png"
            ),
            StarshipSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/torpedo_ship/atark_torpedo_ship.png"
            ),
            StarshipSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            StarshipSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/bomber/karcan_bomber.png"
            ),
            StarshipSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/dreadnought/karcan_dreadnought.png"
            ),
            StarshipSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/fighter/karcan_fighter.png"
            ),
            StarshipSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/frigate/karcan_frigate.png"
            ),
            StarshipSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/scout/karcan_scout.png"
            ),
            StarshipSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/support_ship/karcan_support_ship.png"
            ),
            StarshipSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/torpedo_ship/karcan_torpedo_ship.png"
            ),
            StarshipSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            StarshipSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/bomber/noozler_bomber.png"
            ),
            StarshipSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/dreadnought/noozler_dreadnought.png"
            ),
            StarshipSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/fighter/noozler_fighter.png"
            ),
            StarshipSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/frigate/noozler_frigate.png"
            ),
            StarshipSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/scout/noozler_scout.png"
            ),
            StarshipSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/support_ship/noozler_support_ship.png"
            ),
            StarshipSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/torpedo_ship/noozler_torpedo_ship.png"
            ),
            StarshipSprite::GranokBattleCruiser => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokBomber => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokDreadnought => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokFighter => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokFrigate => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokScout => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokSupportShip => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
            StarshipSprite::GranokTorpedoShip => write!(
                formatter,
                "images/factions/granok/starships/torpedo_ship/granok_torpedo_ship.png"
            ),
        }
    }
}

impl StarshipSprite {
    pub fn sprite_convert_from(starship_icon: StarshipIcon) -> StarshipSprite {
        match starship_icon {
            StarshipIcon::AtarkSupportShip => StarshipSprite::AtarkSupportShip,
            StarshipIcon::AtarkScout => StarshipSprite::AtarkScout,
            StarshipIcon::AtarkFighter => StarshipSprite::AtarkFighter,
            StarshipIcon::AtarkTorpedoShip => StarshipSprite::AtarkTorpedoShip,
            StarshipIcon::AtarkBomber => StarshipSprite::AtarkBomber,
            StarshipIcon::AtarkFrigate => StarshipSprite::AtarkFrigate,
            StarshipIcon::AtarkBattleCruiser => StarshipSprite::AtarkBattleCruiser,
            StarshipIcon::AtarkDreadnought => StarshipSprite::AtarkDreadnought,
            StarshipIcon::KarcanSupportShip => StarshipSprite::KarcanSupportShip,
            StarshipIcon::KarcanScout => StarshipSprite::KarcanScout,
            StarshipIcon::KarcanFighter => StarshipSprite::KarcanFighter,
            StarshipIcon::KarcanTorpedoShip => StarshipSprite::KarcanTorpedoShip,
            StarshipIcon::KarcanBomber => StarshipSprite::KarcanBomber,
            StarshipIcon::KarcanFrigate => StarshipSprite::KarcanFrigate,
            StarshipIcon::KarcanBattleCruiser => StarshipSprite::KarcanBattleCruiser,
            StarshipIcon::KarcanDreadnought => StarshipSprite::KarcanDreadnought,
            StarshipIcon::NoozlerSupportShip => StarshipSprite::NoozlerSupportShip,
            StarshipIcon::NoozlerScout => StarshipSprite::NoozlerScout,
            StarshipIcon::NoozlerFighter => StarshipSprite::NoozlerFighter,
            StarshipIcon::NoozlerTorpedoShip => StarshipSprite::NoozlerTorpedoShip,
            StarshipIcon::NoozlerBomber => StarshipSprite::NoozlerBomber,
            StarshipIcon::NoozlerFrigate => StarshipSprite::NoozlerFrigate,
            StarshipIcon::NoozlerBattleCruiser => StarshipSprite::NoozlerBattleCruiser,
            StarshipIcon::NoozlerDreadnought => StarshipSprite::NoozlerDreadnought,
            StarshipIcon::GranokSupportShip => StarshipSprite::GranokSupportShip,
            StarshipIcon::GranokScout => StarshipSprite::GranokScout,
            StarshipIcon::GranokFighter => StarshipSprite::GranokFighter,
            StarshipIcon::GranokTorpedoShip => StarshipSprite::GranokTorpedoShip,
            StarshipIcon::GranokBomber => StarshipSprite::GranokBomber,
            StarshipIcon::GranokFrigate => StarshipSprite::GranokFrigate,
            StarshipIcon::GranokBattleCruiser => StarshipSprite::GranokBattleCruiser,
            StarshipIcon::GranokDreadnought => StarshipSprite::GranokDreadnought,
            StarshipIcon::None => panic!("Spaceship Sprite: Must have an icon to convert"),
        }
    }

    pub fn starship_type_convert_from(starship_sprite: StarshipSprite) -> StarshipType {
        match starship_sprite {
            StarshipSprite::AtarkBattleCruiser
            | StarshipSprite::KarcanBattleCruiser
            | StarshipSprite::NoozlerBattleCruiser
            | StarshipSprite::GranokBattleCruiser => StarshipType::BattleCruiser,
            StarshipSprite::AtarkBomber
            | StarshipSprite::KarcanBomber
            | StarshipSprite::NoozlerBomber
            | StarshipSprite::GranokBomber => StarshipType::Bomber,
            StarshipSprite::AtarkDreadnought
            | StarshipSprite::KarcanDreadnought
            | StarshipSprite::NoozlerDreadnought
            | StarshipSprite::GranokDreadnought => StarshipType::Dreadnought,
            StarshipSprite::AtarkFighter
            | StarshipSprite::KarcanFighter
            | StarshipSprite::NoozlerFighter
            | StarshipSprite::GranokFighter => StarshipType::Fighter,
            StarshipSprite::AtarkFrigate
            | StarshipSprite::KarcanFrigate
            | StarshipSprite::NoozlerFrigate
            | StarshipSprite::GranokFrigate => StarshipType::Frigate,
            StarshipSprite::AtarkScout
            | StarshipSprite::KarcanScout
            | StarshipSprite::NoozlerScout
            | StarshipSprite::GranokScout => StarshipType::Scout,
            StarshipSprite::AtarkSupportShip
            | StarshipSprite::KarcanSupportShip
            | StarshipSprite::NoozlerSupportShip
            | StarshipSprite::GranokSupportShip => StarshipType::SupportShip,
            StarshipSprite::AtarkTorpedoShip
            | StarshipSprite::KarcanTorpedoShip
            | StarshipSprite::NoozlerTorpedoShip
            | StarshipSprite::GranokTorpedoShip => StarshipType::TorpedoShip,
        }
    }
}

#[cfg(test)]
mod starship_sprite_should {
    // use super::*;
    use rstest::rstest;

    #[rstest]
    #[ignore = "to do"]
    fn sprite_convert_from_none() {}

    #[rstest]
    #[ignore = "to do"]
    fn sprite_convert_from() {}

    #[rstest]
    #[ignore = "to do"]
    fn starship_type_convert_from() {}
}
