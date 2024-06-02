use std::fmt::Display;

#[allow(dead_code)]
pub enum EngineSprite {
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

impl Display for EngineSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineSprite::AtarkBattleCruiser => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_battlecruiser.png"
            ),
            EngineSprite::AtarkBomber => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_bomber.png"
            ),
            EngineSprite::AtarkDreadnought => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_dreadnought.png"
            ),
            EngineSprite::AtarkFighter => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_fighter.png"
            ),
            EngineSprite::AtarkFrigate => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_frigate.png"
            ),
            EngineSprite::AtarkScout => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_scout.png"
            ),
            EngineSprite::AtarkSupportShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_support_ship.png"
            ),
            EngineSprite::AtarkTorpedoShip => write!(
                formatter,
                "images/factions/atark/starships/battlecruiser/atark_torpedo_ship.png"
            ),
            EngineSprite::KarcanBattleCruiser => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_battlecruiser.png"
            ),
            EngineSprite::KarcanBomber => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_bomber.png"
            ),
            EngineSprite::KarcanDreadnought => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_dreadnought.png"
            ),
            EngineSprite::KarcanFighter => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_fighter.png"
            ),
            EngineSprite::KarcanFrigate => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_frigate.png"
            ),
            EngineSprite::KarcanScout => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_scout.png"
            ),
            EngineSprite::KarcanSupportShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_support_ship.png"
            ),
            EngineSprite::KarcanTorpedoShip => write!(
                formatter,
                "images/factions/karcan/starships/battlecruiser/karcan_torpedo_ship.png"
            ),
            EngineSprite::NoozlerBattleCruiser => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_battlecruiser.png"
            ),
            EngineSprite::NoozlerBomber => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_bomber.png"
            ),
            EngineSprite::NoozlerDreadnought => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_dreadnought.png"
            ),
            EngineSprite::NoozlerFighter => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_fighter.png"
            ),
            EngineSprite::NoozlerFrigate => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_frigate.png"
            ),
            EngineSprite::NoozlerScout => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_scout.png"
            ),
            EngineSprite::NoozlerSupportShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_support_ship.png"
            ),
            EngineSprite::NoozlerTorpedoShip => write!(
                formatter,
                "images/factions/noozler/starships/battlecruiser/noozler_torpedo_ship.png"
            ),
        }
    }
}
