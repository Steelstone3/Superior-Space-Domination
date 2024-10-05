use bevy::{
    math::{Quat, Vec3, Vec3Swizzles},
    prelude::{Query, Res, Transform},
    time::Time,
};

use crate::components::controllable::Movement;

pub fn new_controllable_move_to_target(
    time: Res<Time>,
    mut controllable_query: Query<(&Movement, &mut Transform)>,
) {
    for mut controllable in controllable_query.iter_mut() {
        if controllable.1.translation == controllable.0.target_location {
            return;
        }

        let current_location = controllable.1.translation;
        let mut target_location = controllable.0.target_location;
        //ensure moveable stays at current z index
        target_location.z = current_location.z;

        controllable.1.translation +=
            (controllable.0.target_location - current_location) * time.delta_seconds();

        let to_target = (target_location.xy() - current_location.xy()).normalize();
        let rotate_to_target = Quat::from_rotation_arc(Vec3::Y, to_target.extend(0.0));
        controllable.1.rotation = rotate_to_target;
    }
}
