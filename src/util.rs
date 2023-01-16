use bevy::prelude::Vec3;

use crate::Bounds2D;

pub fn keep_in_bounds(bounds: &Bounds2D, mut pos: Vec3, padding: f32) -> Vec3 {
    if pos.x < bounds.min_x + padding {
        pos.x = bounds.min_x + padding
    };
    if pos.x > bounds.max_x - padding {
        pos.x = bounds.max_x - padding
    };
    if pos.z < bounds.min_z + padding {
        pos.z = bounds.min_z + padding
    };
    if pos.z > bounds.max_z - padding {
        pos.z = bounds.max_z - padding
    };
    pos
}
