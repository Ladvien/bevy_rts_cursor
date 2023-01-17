use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rts_cursor::{Bounds2D, CursorPlugin, CursorReflector, RayReflector, RaycastSource};

pub const GAME_X_MIN: f32 = 0.0;
pub const GAME_Z_MIN: f32 = 0.0;
pub const GAME_X_MAX: f32 = 32.0;
pub const GAME_Z_MAX: f32 = 32.0;

pub const CAMERA_ROTATION_SPEED: f32 = 2.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 25.0;

pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: GAME_X_MIN,
    min_z: GAME_Z_MIN,
    max_x: GAME_X_MAX,
    max_z: GAME_Z_MAX,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CursorPlugin {
            bounds: GAME_BOUNDS,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(camera_controls)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(GAME_BOUNDS.max_x + 5., 24., GAME_BOUNDS.max_z + 5.)
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
                size: GAME_BOUNDS.max_x / 2.,
            })),
            material: materials.add(StandardMaterial {
                emissive: Color::DARK_GRAY,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(12., 0., 12.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CursorReflector);
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    // mut game: ResMut<Game>,
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
