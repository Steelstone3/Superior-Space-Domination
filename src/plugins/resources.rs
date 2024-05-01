use bevy::app::Plugin;

use crate::resources::camera_settings::CameraSettings;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraSettings::default());
    }
}
