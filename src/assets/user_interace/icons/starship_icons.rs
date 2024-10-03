// pub mod atark_icons;
// pub mod karcan_icons;
// pub mod noozler_icons;

use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum StarshipIcon {
    AtarkSupportShip,
    AtarkScout,
    AtarkFighter,
    AtarkTorpedoShip,
    AtarkBomber,
    AtarkFrigate,
    AtarkBattlecruiser,
    AtarkDreadnought,
    KarcanSupportShip,
    KarcanScout,
    KarcanFighter,
    KarcanTorpedoShip,
    KarcanBomber,
    KarcanFrigate,
    KarcanBattlecruiser,
    KarcanDreadnought,
    NoozlerSupportShip,
    NoozlerScout,
    NoozlerFighter,
    NoozlerTorpedoShip,
    NoozlerBomber,
    NoozlerFrigate,
    NoozlerBattlecruiser,
    NoozlerDreadnought,
    None,
}

impl Display for StarshipIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StarshipIcon::AtarkSupportShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_support_ship_icon.png"
                )
            }
            StarshipIcon::AtarkScout => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_scout_icon.png"
                )
            }
            StarshipIcon::AtarkFighter => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_fighter_icon.png"
                )
            }
            StarshipIcon::AtarkTorpedoShip => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_torpedo_ship_icon.png"
                )
            }
            StarshipIcon::AtarkBomber => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_bomber_icon.png"
                )
            }
            StarshipIcon::AtarkFrigate => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_frigate_icon.png"
                )
            }
            StarshipIcon::AtarkBattlecruiser => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_battlecruiser_icon.png"
                )
            }
            StarshipIcon::AtarkDreadnought => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_dreadnought_icon.png"
                )
            }
            StarshipIcon::KarcanSupportShip => todo!(),
            StarshipIcon::KarcanScout => todo!(),
            StarshipIcon::KarcanFighter => todo!(),
            StarshipIcon::KarcanTorpedoShip => todo!(),
            StarshipIcon::KarcanBomber => todo!(),
            StarshipIcon::KarcanFrigate => todo!(),
            StarshipIcon::KarcanBattlecruiser => todo!(),
            StarshipIcon::KarcanDreadnought => todo!(),
            StarshipIcon::NoozlerSupportShip => todo!(),
            StarshipIcon::NoozlerScout => todo!(),
            StarshipIcon::NoozlerFighter => todo!(),
            StarshipIcon::NoozlerTorpedoShip => todo!(),
            StarshipIcon::NoozlerBomber => todo!(),
            StarshipIcon::NoozlerFrigate => todo!(),
            StarshipIcon::NoozlerBattlecruiser => todo!(),
            StarshipIcon::NoozlerDreadnought => todo!(),
            StarshipIcon::None => write!(formatter, ""),
        }
    }
}


// KarcanIcon::Battlecruiser => {
    //     write!(
        //         formatter,
        //         "user_interface/icons/starships/karcan/karcan_battlecruiser_icon.png"
        //     )
// }
// KarcanIcon::Bomber => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_bomber_icon.png"
//     )
// }
// KarcanIcon::Dreadnought => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_dreadnought_icon.png"
//     )
// }
// KarcanIcon::Fighter => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_fighter_icon.png"
//     )
// }
// KarcanIcon::Frigate => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_frigate_icon.png"
//     )
// }
// KarcanIcon::Scout => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_scout_icon.png"
//     )
// }
// KarcanIcon::SupportShip => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_support_ship_icon.png"
//     )
// }
// KarcanIcon::TorpedoShip => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/karcan/karcan_torpedo_ship_icon.png"
//     )
// }

// NoozlerIcon::Battlecruiser => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_battlecruiser_icon.png"
//     )
// }
// NoozlerIcon::Bomber => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_bomber_icon.png"
//     )
// }
// NoozlerIcon::Dreadnought => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_dreadnought_icon.png"
//     )
// }
// NoozlerIcon::Fighter => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_fighter_icon.png"
//     )
// }
// NoozlerIcon::Frigate => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_frigate_icon.png"
//     )
// }
// NoozlerIcon::Scout => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_scout_icon.png"
//     )
// }
// NoozlerIcon::SupportShip => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_support_ship_icon.png"
//     )
// }
// NoozlerIcon::TorpedoShip => {
//     write!(
//         formatter,
//         "user_interface/icons/starships/noozler/noozler_torpedo_ship_icon.png"
//     )
// }