
  
# 3D Entity Selection Cursor

A [Bevy](https://github.com/bevyengine/bevy) plugin for selecting game entities in world space. Built with [aevyrie's](https://github.com/aevyrie) amazing [`bevy_mod_raycast`](https://github.com/aevyrie/bevy_mod_raycast) plugin.

![bevy_cursor_demo](https://user-images.githubusercontent.com/6241517/214854473-659d8e91-283d-4f85-976f-b4ac5ffdf22a.gif)

## Features
* Select entities in world space
* Raycast based cursor for 3D environments
* Drag-and-drop selection
* Selected entities are accessible via `Res<Cursor>` resource

# Quickstart

Provide game-space bounds with `Bounds2D` and add the `CursorPlugin` to your game.
```rust
pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: 0.,
    min_z: 0.,
    max_x: 10.,
    max_z: 10.,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CursorPlugin {
            bounds: GAME_BOUNDS,
            ..Default::default()
        })
        .add_startup_system(setup)
        .run();
}
```

Add the `RaycastSource::<RayReflector>` to your `Camera3dBundle`.
```rust
fn setup_cameras(mut commands: Commands) {
    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(10., 15., 10.)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },))
        .insert(RaycastSource::<RayReflector>::new()); // Designate the camera as our source;
}
```

Mark surfaces you want to interact with the cursor with a `CursorReflector` component.
```rust
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 20.,
            })),
            ..Default::default()
        })
        .insert(CursorReflector);
```

Add `Pickable` to the entities you want to select.
```rust
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.5, 0.5))),
            ..Default::default()
        })
        .insert(Pickable);
```

Access selected entities with the `Res<Cursor>` resource.
```rust
pub fn select_destination(
    mut commands: Commands,
    cursor: Res<Cursor>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for unit in &cursor.selection.selected_units {
            commands.entity(*unit).insert(Destination(cursor.location));
        }
    }
}
```

# Demo

To run a minimal demo, clone this repository and run:

```console
cargo run --example move_units
```

# License

This project is licensed under the [MIT license](https://github.com/ladvien/bevy_rts_cursor/blob/main/LICENSE).


