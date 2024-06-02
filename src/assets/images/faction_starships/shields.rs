use std::fmt::Display;

#[allow(dead_code)]
pub enum ShieldSprite {
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

impl Display for ShieldSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            ShieldSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber.png"
            ),
            ShieldSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought.png"
            ),
            ShieldSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter.png"
            ),
            ShieldSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate.png"
            ),
            ShieldSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout.png"
            ),
            ShieldSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship.png"
            ),
            ShieldSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship.png"
            ),
            ShieldSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            ShieldSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber.png"
            ),
            ShieldSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought.png"
            ),
            ShieldSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter.png"
            ),
            ShieldSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate.png"
            ),
            ShieldSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout.png"
            ),
            ShieldSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship.png"
            ),
            ShieldSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship.png"
            ),
            ShieldSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            ShieldSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber.png"
            ),
            ShieldSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought.png"
            ),
            ShieldSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter.png"
            ),
            ShieldSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate.png"
            ),
            ShieldSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout.png"
            ),
            ShieldSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship.png"
            ),
            ShieldSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship.png"
            ),
        }
    }
}
