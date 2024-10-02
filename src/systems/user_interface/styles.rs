use bevy::{
    asset::AssetServer,
    color::palettes::css::{DARK_GRAY, WHITE},
    prelude::{ButtonBundle, ImageBundle, Res},
    ui::{Style, UiImage, UiRect, Val},
};

use crate::{
    assets::user_interace::icons::starships::atark_icons::AtarkIcon,
    components::user_interface::SelectStarshipSpawnButton,
};

pub fn create_starship_button_bundle(icon: AtarkIcon) -> (ButtonBundle, SelectStarshipSpawnButton) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
                ..Default::default()
            },
            border_color: DARK_GRAY.into(),
            ..Default::default()
        },
        SelectStarshipSpawnButton { icon },
    )
}

pub fn create_starship_button_icon(
    asset_server: &Res<AssetServer>,
    animal_sub_menu: AtarkIcon,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(animal_sub_menu.to_string())),
        background_color: WHITE.into(),
        ..Default::default()
    }
}
