use std::collections::HashSet;

use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod,
    RaycastSource, RaycastSystem,
};

mod components;
mod resources;
mod util;

pub use components::{CursorReflector, Selected, SelectionHighlighter};
pub use resources::{Aesthetics, Bounds2D, CursorPlugin};
use util::keep_in_bounds;

impl Default for CursorPlugin {
    fn default() -> Self {
        Self {
            bounds: Bounds2D {
                min_x: 1.,
                min_z: 1.,
                max_x: 1.,
                max_z: 1.,
            },
            aesthetics: Default::default(),
        }
    }
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .insert_resource(Bounds2D {
                min_x: self.bounds.min_x,
                min_z: self.bounds.min_z,
                max_x: self.bounds.max_x,
                max_z: self.bounds.max_z,
            })
            .insert_resource(self.aesthetics.to_owned())
            .add_plugin(DefaultRaycastingPlugin::<RayReflector>::default())
            .add_startup_system(setup)
            .add_system(selection_system)
            .add_system(mouse_system)
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays::<RayReflector>),
            )
            .add_system_to_stage(CoreStage::PostUpdate, make_scene_pickable);
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
    pub entity: Option<Entity>,
    pub location: Location,
    pub pressed_location: Location,
    pub pressed: bool,
    pub selection: Selection,
    pub xyz1: Vec3,
    pub xyz2: Vec3,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Selection {
    entity: Option<Entity>,
    pub selected_units: HashSet<Entity>,
    pub just_selected: bool,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Pickable;

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct Location {
    pub xyz: Vec3,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct BoundingBox;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct BoundingBoxArea {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct RayReflector;

fn setup(mut commands: Commands) {
    // Overwrite the default plugin state with one that enables the debug cursor. This line can be
    // removed if the debug cursor isn't needed as the state is set to default values when the
    // default plugin is added.
    commands.insert_resource(DefaultPluginState::<RayReflector>::default().with_debug_cursor());
}

#[allow(clippy::type_complexity)]
fn make_scene_pickable(
    mut commands: Commands,
    mesh_query: Query<
        Entity,
        (
            // With<Handle<Mesh>>,
            // Without<RaycastMesh<RayReflector>>,
            With<CursorReflector>,
        ),
    >,
) {
    for entity in &mesh_query {
        commands
            .entity(entity)
            .insert(RaycastMesh::<RayReflector>::default()); // Make this mesh ray cast-able
    }

    // if let Err(e) = app_state.set(CursorState::Ready) {
    //     eprintln!("{:?}", e);
    // }
}

fn get_selection_confirmed_box(
    cursor: Cursor,
    line_thickness: f32,
    ground_height: f32,
) -> Vec<Mesh> {
    vec![
        Mesh::from(shape::Box {
            min_x: cursor.xyz1[0],
            max_x: cursor.xyz2[0] - line_thickness,
            min_y: ground_height + 0.3,
            max_y: ground_height + 0.3,
            min_z: cursor.xyz2[2] - line_thickness,
            max_z: cursor.xyz2[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: cursor.xyz2[0] + line_thickness,
            max_x: cursor.xyz1[0] + line_thickness,
            min_y: ground_height + 0.3,
            max_y: ground_height + 0.3,
            min_z: cursor.xyz1[2] - line_thickness,
            max_z: cursor.xyz1[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: cursor.xyz2[0] - line_thickness,
            max_x: cursor.xyz2[0] + line_thickness,
            min_y: ground_height + 0.3,
            max_y: ground_height + 0.3,
            min_z: cursor.xyz1[2] + line_thickness,
            max_z: cursor.xyz2[2] + line_thickness,
        }),
        Mesh::from(shape::Box {
            min_x: cursor.xyz1[0] - line_thickness,
            max_x: cursor.xyz1[0] + line_thickness,
            min_y: ground_height + 0.3,
            max_y: ground_height + 0.3,
            min_z: cursor.xyz1[2] - line_thickness,
            max_z: cursor.xyz2[2] + line_thickness,
        }),
    ]
}

fn selection_system(
    mut commands: Commands,
    mut cursor: ResMut<Cursor>,
    aesthetics: Res<Aesthetics>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transforms: Query<&Transform>,
    mut query: Query<(Entity, &mut Pickable), With<Pickable>>,
) {
    if cursor.selection.just_selected {
        //                  c1
        //                  *
        //              *      *
        //        c4 *            * c2
        //              *      *
        //                  *
        //                 c3

        let mut lines = get_selection_confirmed_box(
            cursor.clone(),
            aesthetics.line_thickness,
            aesthetics.ground_height,
        );
        let box_material = StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            base_color: aesthetics.bounding_box_color,
            emissive: aesthetics.bounding_box_color,
            unlit: false,
            ..default()
        };
        // let blinker = Blinker::new(0.01, AFTER_SELECTION_BLINK_DURATION, 2);

        if let Some(parent_line) = &mut lines.pop() {
            let parent_id = commands
                .spawn(PbrBundle {
                    material: materials.add(box_material.clone()),
                    mesh: meshes.add(parent_line.clone()),
                    ..default()
                })
                .insert(NotShadowReceiver)
                .insert(NotShadowCaster)
                // .insert(blinker.clone())
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
                    // .insert(blinker.clone())
                    .insert(Name::new("SelectionBox"))
                    .id();
                commands.entity(parent_id).add_child(child_id);
            }
        }

        for (entity, _) in query.iter_mut() {
            let transform = *transforms.get(entity).unwrap();

            // Check if entities are within the highlighted area.
            if !transform.translation.cmplt(cursor.xyz1).any()
                && !transform.translation.cmpgt(cursor.xyz2).any()
            {
                // Add selected torus.
                let child_id = commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Torus {
                            ring_radius: aesthetics.selected_line_thickness,
                            radius: transform.scale.x.max(transform.scale.z) / 2.,
                            ..default()
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: aesthetics.selected_area_box_color,
                            emissive: aesthetics.selected_area_box_color,
                            ..default()
                        }),
                        transform: Transform {
                            translation: Vec3::new(0., 0.1, 0.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(SelectionHighlighter)
                    .insert(Name::new("SelectionHighlighter"))
                    .id();
                commands.entity(entity).add_child(child_id);

                // Track selected.
                cursor.selection.selected_units.insert(entity);

                commands.entity(entity).insert(Selected);
            }
        }
    }
    cursor.selection.just_selected = false;
}

fn mouse_system(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    aesthetics: Res<Aesthetics>,
    mut cursor: ResMut<Cursor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    intersection_query: Query<&Intersection<RayReflector>>,
    mut query: Query<(&mut Transform, &BoundingBox)>,
    selection_hihlights: Query<Entity, With<SelectionHighlighter>>,
    selected: Query<Entity, With<Selected>>,
    bounds: Res<Bounds2D>,
) {
    // RayCast to get the mouse position in game coordinates.
    for intersection in &intersection_query {
        if let Some(xyz) = intersection.position() {
            cursor.location.xyz = xyz.to_owned();
            cursor.location.xyz = keep_in_bounds(&bounds, cursor.location.xyz, 0.);
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        cursor.pressed = true;
        cursor.pressed_location = cursor.location;
    }
    if buttons.just_released(MouseButton::Left) {
        cursor.pressed = false;
        cursor.pressed_location = Location {
            xyz: Vec3::new(-1., -1., -1.),
        }
    }

    if let Ok((mut transform, _)) = query.get_single_mut() {
        if cursor.pressed {
            let difference = cursor.location.xyz - cursor.pressed_location.xyz;
            transform.translation = cursor.pressed_location.xyz + difference / 2.;
            transform.translation[1] += 0.25;
            transform.scale = Vec3::new(difference.x, 0.0, difference.z);
        }
        if buttons.just_released(MouseButton::Left) {
            if let Some(entity) = cursor.selection.entity {
                let x1 = transform.translation[0] - (transform.scale[0] / 2.);
                let z1 = transform.translation[2] - (transform.scale[2] / 2.);
                let y1 = aesthetics.ground_height;
                let y2 = aesthetics.ground_height + 0.2;
                let x2 = transform.translation[0] + transform.scale[0] - (transform.scale[0] / 2.);
                let z2 = transform.translation[2] + transform.scale[2] - (transform.scale[2] / 2.);

                let min_x = x1.min(x2);
                let max_x = x1.max(x2);
                let min_y = y1.min(y2);
                let max_y = y1.max(y2);
                let min_z = z1.min(z2);
                let max_z = z1.max(z2);

                cursor.selection.just_selected = true;
                cursor.xyz1 = Vec3::new(min_x, min_y, min_z);
                cursor.xyz2 = Vec3::new(max_x, max_y, max_z);

                commands.entity(entity).despawn_recursive();
            }
        }
    }
    if buttons.just_pressed(MouseButton::Left) {
        cursor.xyz1 = Vec3::new(-1., -1., -1.);
        cursor.xyz2 = Vec3::new(-1., -1., -1.);

        cursor.selection.entity = Some(
            commands
                .spawn((
                    PbrBundle {
                        material: materials.add(StandardMaterial {
                            alpha_mode: AlphaMode::Blend,
                            base_color: aesthetics.bounding_box_color,
                            emissive: aesthetics.bounding_box_color,
                            unlit: false,
                            ..default()
                        }),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
                        ..default()
                    },
                    BoundingBox,
                ))
                .insert(NotShadowReceiver)
                .insert(NotShadowCaster)
                .insert(Name::new("SelectionBox"))
                .id(),
        );

        // Handle selection action.
        if !cursor.selection.selected_units.is_empty() {
            cursor.selection.selected_units = HashSet::new();

            // Removed Selected
            for entity in &selected {
                commands.entity(entity).remove::<Selected>();
            }

            // Clear SelectionHighlights on deselect.
            for entity in &selection_hihlights {
                commands.entity(entity).despawn_recursive();
            }
        }
    };
}

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<RayReflector>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}
