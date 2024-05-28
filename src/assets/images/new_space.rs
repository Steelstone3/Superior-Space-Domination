use bevy::reflect::Reflect;
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Copy, Clone, Debug, PartialEq, Reflect)]
pub enum NewSpaceSprite {
    Space1,
    Space2,
    Space3,
    Space4,
    Space5,
    Space6,
    Space7,
    Space8,
    Space9,
    Space10,
    Space11,
    Space12,
    Space13,
    Space14,
    Space15,
    Space16,
    Space17,
    Space18,
    Space19,
    Space20,
}

impl Display for NewSpaceSprite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewSpaceSprite::Space1 => {
                write!(formatter, "images/new_space/space_1.png")
            }
            NewSpaceSprite::Space2 => {
                write!(formatter, "images/new_space/space_2.png")
            }
            NewSpaceSprite::Space3 => {
                write!(formatter, "images/new_space/space_3.png")
            }
            NewSpaceSprite::Space4 => {
                write!(formatter, "images/new_space/space_4.png")
            }
            NewSpaceSprite::Space5 => {
                write!(formatter, "images/new_space/space_5.png")
            }
            NewSpaceSprite::Space6 => {
                write!(formatter, "images/new_space/space_6.png")
            }
            NewSpaceSprite::Space7 => {
                write!(formatter, "images/new_space/space_7.png")
            }
            NewSpaceSprite::Space8 => {
                write!(formatter, "images/new_space/space_8.png")
            }
            NewSpaceSprite::Space9 => {
                write!(formatter, "images/new_space/space_9.png")
            }
            NewSpaceSprite::Space10 => {
                write!(formatter, "images/new_space/space_10.png")
            }
            NewSpaceSprite::Space11 => {
                write!(formatter, "images/new_space/space_11.png")
            }
            NewSpaceSprite::Space12 => {
                write!(formatter, "images/new_space/space_12.png")
            }
            NewSpaceSprite::Space13 => {
                write!(formatter, "images/new_space/space_13.png")
            }
            NewSpaceSprite::Space14 => {
                write!(formatter, "images/new_space/space_14.png")
            }
            NewSpaceSprite::Space15 => {
                write!(formatter, "images/new_space/space_15.png")
            }
            NewSpaceSprite::Space16 => {
                write!(formatter, "images/new_space/space_16.png")
            }
            NewSpaceSprite::Space17 => {
                write!(formatter, "images/new_space/space_17.png")
            }
            NewSpaceSprite::Space18 => {
                write!(formatter, "images/new_space/space_18.png")
            }
            NewSpaceSprite::Space19 => {
                write!(formatter, "images/new_space/space_19.png")
            }
            NewSpaceSprite::Space20 => {
                write!(formatter, "images/new_space/space_20.png")
            }
        }
    }
}
