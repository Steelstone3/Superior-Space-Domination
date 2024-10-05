use bevy::prelude::{Query, Transform, With, Without};

use crate::components::{tracking::Tracking, user_interface::SelectedSprite};

pub fn new_update_selection_sprite_location(
    mut selection_query: Query<(&Tracking, &mut Transform), With<SelectedSprite>>,
    mut entity_query: Query<&mut Transform, Without<Tracking>>,
) {
    for mut selection in selection_query.iter_mut() {
        let Ok(entity_to_follow_transform) = entity_query.get_mut(selection.0.entity_to_follow)
        else {
            return;
        };
        selection.1.translation = entity_to_follow_transform.translation;
        selection.1.rotation = entity_to_follow_transform.rotation;
    }
}
