use bevy::{
    asset::AssetServer,
    color::palettes::css::{DARK_GRAY, WHITE},
    prelude::{ButtonBundle, ImageBundle, Res},
    ui::{Style, UiImage, UiRect, Val},
};

use crate::{

    assets::user_interace::icons::starship_icons::StarshipIcon, components::user_interface::SelectStarshipSpawnButton
};

pub fn create_starship_button_bundle(icon: StarshipIcon) -> (ButtonBundle, SelectStarshipSpawnButton) {
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
    sub_menu: StarshipIcon,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(sub_menu.to_string())),
        background_color: WHITE.into(),
        ..Default::default()
    }
}
