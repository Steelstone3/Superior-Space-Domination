use std::fmt::Display;

#[allow(dead_code)]
pub enum DestructionSprite {
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

impl Display for DestructionSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DestructionSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            DestructionSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber.png"
            ),
            DestructionSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought.png"
            ),
            DestructionSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter.png"
            ),
            DestructionSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate.png"
            ),
            DestructionSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout.png"
            ),
            DestructionSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship.png"
            ),
            DestructionSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship.png"
            ),
            DestructionSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            DestructionSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber.png"
            ),
            DestructionSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought.png"
            ),
            DestructionSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter.png"
            ),
            DestructionSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate.png"
            ),
            DestructionSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout.png"
            ),
            DestructionSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship.png"
            ),
            DestructionSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship.png"
            ),
            DestructionSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            DestructionSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber.png"
            ),
            DestructionSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought.png"
            ),
            DestructionSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter.png"
            ),
            DestructionSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate.png"
            ),
            DestructionSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout.png"
            ),
            DestructionSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship.png"
            ),
            DestructionSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship.png"
            ),
        }
    }
}
