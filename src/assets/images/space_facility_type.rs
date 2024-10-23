use crate::{
    assets::user_interface::icons::space_facility_icons::SpaceFacilityIcon,
    resources::faction::Faction,
};

pub enum SpaceFacilityType {
    SpaceShipConstructionYard,
}

impl SpaceFacilityType {
    #[allow(dead_code)]
    pub fn icon_convert_from(&self, faction: Faction) -> SpaceFacilityIcon {
        match faction {
            Faction::Atark => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::AtarkSpaceShipConstructionYard
                }
            },
            Faction::Karcan => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::KarcanSpaceShipConstructionYard
                }
            },
            Faction::Noozler => match self {
                SpaceFacilityType::SpaceShipConstructionYard => {
                    SpaceFacilityIcon::NoozlerSpaceShipConstructionYard
                }
            },
        }
    }
}
