use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_raycast::RaycastSource;
use bevy_rts_cursor::{Bounds2D, CursorPlugin, CursorReflector, Pickable, RayReflector};

pub const SCREEN_WIDTH: f32 = 720.0;
pub const SCREEN_HEIGHT: f32 = 640.0;
pub const GAME_TITLE: &str = "Scene as Cursor Base";
pub const START_X_POX: f32 = 960.0;
pub const START_Y_POX: f32 = 0.0;

pub const CAMERA_ROTATION_SPEED: f32 = 2.5;
pub const CAMERA_MOVEMENT_SPEED: f32 = 25.0;

pub const CAM_ORIGIN_X: f32 = 15.0;
pub const CAM_ORIGIN_Y: f32 = 15.0;
pub const CAM_ORIGIN_Z: f32 = 15.0;

pub const GAME_X_MIN: f32 = -13.0;
pub const GAME_Z_MIN: f32 = -13.0;
pub const GAME_X_MAX: f32 = 13.0;
pub const GAME_Z_MAX: f32 = 13.0;

pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: GAME_X_MIN,
    min_z: GAME_Z_MIN,
    max_x: GAME_X_MAX,
    max_z: GAME_Z_MAX,
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                title: GAME_TITLE.to_string(),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::At(Vec2::new(START_X_POX, START_Y_POX)),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CursorPlugin {
            bounds: GAME_BOUNDS,
            ..Default::default()
        })
        .init_resource::<Game>()
        .add_system(move_scene_entities)
        .add_system(bevy::window::close_on_esc)
        .add_state(GameState::Playing)
        .add_startup_system(setup)
        // .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(camera_controls))
        .run();
}

#[derive(Component)]
struct MovedScene;

#[derive(Resource, Default)]
pub struct Game {
    mechanics: Mechanics,
}

#[derive(Default)]
pub struct Mechanics {
    pub move_cooldown: Timer,
    pub rotate_cooldown: Timer,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(20., 25., 20.)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        })
        .insert(RaycastSource::<RayReflector>::new());

    // Spawn a second scene, and add a tag component to be able to target it later
    commands
        .spawn((SceneBundle {
            scene: asset_server.load("BACKROOMS_POLY_GAME1.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(7.4, 0.0, 3.0),
                ..Default::default()
            },
            ..default()
        },))
        .insert(CursorReflector)
        .insert(Name::new("scene"));
    commands
        .spawn((SceneBundle {
            scene: asset_server.load("scp-096/scene.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(4.4, 0.0, 3.0),
                ..Default::default()
            },
            ..default()
        },))
        .insert(Pickable)
        .insert(Name::new("scp-096"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: GAME_BOUNDS.min_x.abs() + GAME_BOUNDS.max_x.abs(),
            })),
            material: materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                },
                unlit: false,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.5, 0.0),
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(NotShadowReceiver)
        .insert(NotShadowCaster)
        .insert(CursorReflector);
}

// This system will move all entities that are descendants of MovedScene (which will be all entities spawned in the scene)
fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MovedScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) {
    for moved_scene_entity in &moved_scene {
        let mut offset = 0.;
        for entity in children.iter_descendants(moved_scene_entity) {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.translation = Vec3::new(
                    offset * time.elapsed_seconds().sin() / 20.,
                    0.,
                    time.elapsed_seconds().cos() / 20.,
                );
                offset += 1.0;
            }
        }
    }
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    if game.mechanics.move_cooldown.tick(time.delta()).finished() {
        let mut camera = camera_query.single_mut();

        let mut forward = camera.forward();
        forward.y = 0.0;
        forward = forward.normalize();

        let mut left = camera.left();
        left.y = 0.0;
        left = left.normalize();

        let speed = CAMERA_MOVEMENT_SPEED;
        let rotate_speed = CAMERA_ROTATION_SPEED;

        //Leafwing
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
        if keyboard.pressed(KeyCode::E) {
            camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
        }
        if keyboard.pressed(KeyCode::Q) {
            camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
        }
    }
}
