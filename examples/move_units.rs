// TODO: Add a system to add a Destination for units.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rts_cursor::{
    Bounds2D, Cursor, CursorPlugin, CursorReflector, Pickable, RayReflector, RaycastSource,
};

pub const GAME_X_MIN: f32 = -16.0;
pub const GAME_Z_MIN: f32 = -16.0;
pub const GAME_X_MAX: f32 = 16.0;
pub const GAME_Z_MAX: f32 = 16.0;

pub const CAMERA_ROTATION_SPEED: f32 = 2.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 25.0;

pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: GAME_X_MIN,
    min_z: GAME_Z_MIN,
    max_x: GAME_X_MAX,
    max_z: GAME_Z_MAX,
};

pub const ARRIVAL_TOLERANCE: f32 = 0.75;
pub const SOCIAL_DISTANCE: f32 = 1.7;
pub const GROUND_LEVEL: f32 = 0.5;
pub const MOVE_COOLDOWN: f32 = 0.01;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Unit;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MovementSpeed {
    pub value: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Destination(pub Vec3);

#[derive(Resource, Default)]
pub struct Game {
    mechanics: Mechanics,
}

#[derive(Default)]
pub struct Mechanics {
    pub move_cooldown: Timer,
    pub direction: Direction,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CursorPlugin {
            bounds: GAME_BOUNDS,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(camera_controls)
        .add_system(bevy::window::close_on_esc)
        .add_system(movement_system)
        .add_system(adjust_still_units_system)
        .add_system(select_destination)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    game.mechanics.move_cooldown = Timer::from_seconds(MOVE_COOLDOWN, TimerMode::Repeating);

    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(
                GAME_BOUNDS.max_x,
                GAME_BOUNDS.max_x * 1.25,
                GAME_BOUNDS.max_z,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },))
        .insert(RaycastSource::<RayReflector>::new());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5900.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(GAME_BOUNDS.max_x, GAME_BOUNDS.max_z, GAME_BOUNDS.max_z),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: GAME_BOUNDS.max_x,
            })),
            material: materials.add(StandardMaterial {
                emissive: Color::DARK_GRAY,
                ..default()
            }),
            ..Default::default()
        })
        .insert(CursorReflector);

    let units_to_spawn = 4;
    let scale = 1.;

    let num_of_iters = (units_to_spawn as f64).log2().ceil() as i32;
    let group_size = num_of_iters as f32 * scale - scale;
    let group_offset = group_size * -2.;

    for i in 0..num_of_iters {
        for j in 0..num_of_iters {
            let current_unit_index = i * num_of_iters + j;
            if current_unit_index >= units_to_spawn {
                break;
            }
            let adjusted_x = i as f32 + SOCIAL_DISTANCE + i as f32 * scale + group_offset;
            let adjusted_z = j as f32 + SOCIAL_DISTANCE + j as f32 * scale + group_offset;
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(scale, scale, scale))),
                    material: materials.add(StandardMaterial {
                        emissive: Color::MIDNIGHT_BLUE,
                        ..default()
                    }),
                    transform: Transform {
                        translation: Vec3::new(adjusted_x, scale / 2., adjusted_z),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Unit)
                .insert(Pickable)
                .insert(MovementSpeed { value: 2. })
                .insert(Name::new("BLUE"));
        }
    }
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = CAMERA_MOVEMENT_SPEED;
    let rotate_speed = CAMERA_ROTATION_SPEED;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

pub fn adjust_still_units_system(
    mut units: Query<
        (Entity, &mut Transform, &MovementSpeed),
        (With<MovementSpeed>, Without<Destination>),
    >,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    if game.mechanics.move_cooldown.tick(time.delta()).finished() {
        let all_units_positions: Vec<(Entity, Vec3)> = units
            .into_iter()
            .map(|t| return (t.0, t.1.translation))
            .collect();

        for (entity, mut transform, speed) in &mut units {
            let new_destination = adjust_movement_for_neighbors(
                &entity,
                &transform.translation,
                transform.translation,
                &all_units_positions,
            );

            transform.translation = move_unit(
                &transform.translation,
                new_destination,
                &speed,
                time.delta_seconds(),
            );

            transform.translation = keep_in_bounds(GAME_BOUNDS, transform.translation, 0.0);
        }
    }
}

pub fn select_destination(
    mut commands: Commands,
    cursor: Res<Cursor>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for unit in &cursor.selection.selected_units {
            println!("Adding destination to {:?}", unit);
            commands.entity(*unit).insert(Destination(cursor.location));
        }
    }
}

pub fn movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut units: Query<
        (Entity, &mut Transform, &mut Destination, &MovementSpeed),
        (With<Destination>, With<MovementSpeed>),
    >,
) {
    let units_positions: Vec<(Entity, Vec3)> = units
        .into_iter()
        .map(|t| return (t.0, t.1.translation))
        .collect();
    for (entity, mut transform, destination, speed) in &mut units {
        if game.mechanics.move_cooldown.tick(time.delta()).finished() {
            let new_destination = adjust_movement_for_neighbors(
                &entity,
                &transform.translation,
                destination.0,
                &units_positions,
            );
            transform.translation = move_unit(
                &transform.translation,
                new_destination,
                &speed,
                time.delta_seconds(),
            );
            stop_at_destination(
                &mut commands,
                entity,
                transform.translation,
                destination.0,
                ARRIVAL_TOLERANCE,
            );
        }
    }
}

fn stop_at_destination(
    commands: &mut Commands,
    unit: Entity,
    unit_position: Vec3,
    destination: Vec3,
    arrival_tolerance: f32,
) -> () {
    if are_positions_near(&destination, &unit_position, arrival_tolerance) {
        commands.entity(unit).remove::<Destination>();
    }
}

fn move_unit(
    unit_position: &Vec3,
    new_destination: Vec3,
    unit_speed: &MovementSpeed,
    delta_seconds: f32,
) -> Vec3 {
    let mut new_unit_position =
        unit_position.lerp(new_destination, unit_speed.value * delta_seconds);
    new_unit_position = keep_in_bounds(GAME_BOUNDS, new_unit_position, 0.0);
    new_unit_position.y = GROUND_LEVEL;
    new_unit_position
}

fn adjust_movement_for_neighbors(
    unit: &Entity,
    unit_position: &Vec3,
    unit_destination: Vec3,
    all_units_positions: &Vec<(Entity, Vec3)>,
) -> Vec3 {
    let mut new_destination = Vec3::from(unit_destination);
    for (other_entity, other_position) in all_units_positions {
        // Skip comparing to self.
        if *unit == *other_entity {
            continue;
        }

        if are_positions_near(&unit_position, &other_position, SOCIAL_DISTANCE) {
            let difference = *unit_position - *other_position;
            let minimum_distance = SOCIAL_DISTANCE + SOCIAL_DISTANCE; // Replace with unit specific social distances
            new_destination +=
                difference.normalize() * (minimum_distance - difference) / minimum_distance;
            new_destination = keep_in_bounds(GAME_BOUNDS, new_destination, 0.0);
        }
    }
    new_destination
}

fn are_positions_near(v1: &Vec3, v2: &Vec3, sensitivity: f32) -> bool {
    v2.cmpgt(*v1 - sensitivity).all() && v2.cmplt(*v1 + sensitivity).all()
}

fn keep_in_bounds(bounds: Bounds2D, mut pos: Vec3, padding: f32) -> Vec3 {
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

#[test]
fn keep_in_bounds_when_vec3_contains_negative() {
    let bounds = Bounds2D {
        min_x: -2.,
        min_z: -2.,
        max_x: 2.,
        max_z: 2.,
    };

    let pos = Vec3::new(-3., -2., -3.);

    assert_eq!(keep_in_bounds(bounds, pos, 0.0), Vec3::new(-2., -2., -2.));
}
