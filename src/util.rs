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
    println!("pos: {:?}", position);
    println!("pt1: {:?}", area_pt1);
    println!("pt2: {:?}", area_pt2);
    println!("tol: {:?}", tolerance);
    let t =
        !position.cmplt(area_pt1 - tolerance).any() && !position.cmpgt(area_pt2 + tolerance).any();
    println!("is_position_in_area: {:?}", t);
    t
}

pub fn are_positions_near(v1: &Vec3, v2: &Vec3, sensitivity: f32) -> bool {
    v2.cmpgt(*v1 - sensitivity).all() && v2.cmplt(*v1 + sensitivity).all()
}
