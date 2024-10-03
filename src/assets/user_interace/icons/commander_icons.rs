use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Debug, PartialEq, Reflect, Clone, Copy)]
pub enum CommanderIcon {
    AtarkCommander,
    KaranCommander,
    NoozlerCommander,
    None,
}

impl Display for CommanderIcon {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommanderIcon::AtarkCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/atark/atark_commander.png"
                )
            }
            CommanderIcon::KaranCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/karcan/karcan_commander.png"
                )
            }
            CommanderIcon::NoozlerCommander => {
                write!(
                    formatter,
                    "user_interface/icons/starships/noozler/noozler_commander.png"
                )
            }
            CommanderIcon::None => {
                write!(formatter, "")
            }
        }
    }
}
