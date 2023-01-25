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

pub fn is_position_in_area(
    position: Vec3,
    area_pt1: Vec3,
    area_pt2: Vec3,
    tolerance: Vec3,
) -> bool {
    !position.cmplt(area_pt1 - tolerance).any() && !position.cmpgt(area_pt2 + tolerance).any()
}

pub fn hypotenuse(a: f32, b: f32) -> f32 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub fn map_value_to_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
