use std::fmt::Display;

#[allow(dead_code)]
pub enum FiringSprite {
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
}

impl Display for FiringSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiringSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            FiringSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber.png"
            ),
            FiringSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought.png"
            ),
            FiringSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter.png"
            ),
            FiringSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate.png"
            ),
            FiringSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout.png"
            ),
            FiringSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship.png"
            ),
            FiringSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship.png"
            ),
            FiringSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            FiringSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber.png"
            ),
            FiringSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought.png"
            ),
            FiringSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter.png"
            ),
            FiringSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate.png"
            ),
            FiringSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout.png"
            ),
            FiringSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship.png"
            ),
            FiringSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship.png"
            ),
            FiringSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            FiringSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber.png"
            ),
            FiringSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought.png"
            ),
            FiringSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter.png"
            ),
            FiringSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate.png"
            ),
            FiringSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout.png"
            ),
            FiringSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship.png"
            ),
            FiringSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship.png"
            ),
        }
    }
}
