use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

use crate::{effects::Blinker, Aesthetics, Cursor};

pub const AFTER_SELECTION_BLINK_DURATION: f32 = 0.08;

pub fn create_selection_confirmation_outline(
    commands: &mut Commands,
    cursor: &Cursor,
    aesthetics: &Aesthetics,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    //                  c1
    //                  *
    //              *      *
    //        c4 *            * c2
    //              *      *
    //                  *
    //                 c3

    let mut lines =
        get_selection_confirmed_box(cursor.xyz1, cursor.xyz2, aesthetics.line_thickness);
    let box_material = StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: aesthetics.bounding_box_color,
        emissive: aesthetics.bounding_box_color,
        unlit: false,
        ..default()
    };
    let blinker = Blinker::new(0.01, AFTER_SELECTION_BLINK_DURATION, 2);

    if let Some(parent_line) = &mut lines.pop() {
        let parent_id = commands
            .spawn(PbrBundle {
                material: materials.add(box_material.clone()),
                mesh: meshes.add(parent_line.clone()),
                ..default()
            })
            .insert(NotShadowReceiver)
            .insert(NotShadowCaster)
            .insert(blinker.clone())
            .insert(Name::new("SelectionBox"))
            .id();
        for line in lines {
            let child_id = commands
                .spawn(PbrBundle {
                    material: materials.add(box_material.clone()),
                    mesh: meshes.add(line),
                    ..default()
                })
                .insert(NotShadowReceiver)
                .insert(NotShadowCaster)
                .insert(blinker.clone())
                .insert(Name::new("SelectionBox"))
                .id();
            commands.entity(parent_id).add_child(child_id);
        }
    }
}

fn get_selection_confirmed_box(pt1: Vec3, pt2: Vec3, line_thickness: f32) -> Vec<Mesh> {
    vec![
        Mesh::from(shape::Box {
            min_x: pt1[0],
            max_x: pt2[0] - line_thickness,
            min_y: pt1[1],
            max_y: pt2[1] + line_thickness,
            min_z: pt2[2] - line_thickness,
            max_z: pt2[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: pt2[0] + line_thickness,
            max_x: pt1[0] + line_thickness,
            min_y: pt1[1],
            max_y: pt2[1] + line_thickness,
            min_z: pt1[2] - line_thickness,
            max_z: pt1[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: pt2[0] - line_thickness,
            max_x: pt2[0] + line_thickness,
            min_y: pt1[1],
            max_y: pt2[1] + line_thickness,
            min_z: pt1[2] + line_thickness,
            max_z: pt2[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: pt1[0] - line_thickness,
            max_x: pt1[0] + line_thickness,
            min_y: pt1[1],
            max_y: pt2[1] + line_thickness,
            min_z: pt1[2] - line_thickness,
            max_z: pt2[2] + line_thickness,
        }),
    ]
}
