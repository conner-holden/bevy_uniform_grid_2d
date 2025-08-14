# bevy_uniform_grid_2d
An easy-to-use plugin for people who need basic spatial indexing.

### Installation
```sh
cargo add bevy_uniform_grid_2d
```

### Usage
Below is the [hello_world example](examples/hello_world.rs). Grid changes are logged. For a more detailed example, see the [many_moving_entities example](examples/many_moving_entities.rs).

```sh
cargo run --example hello_world
```

```rust
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;

fn main() {
    App::new()
        // Add default pluugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(800., 800.),
                title: "Hello World Example".to_string(),
                present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
                ..default()
            }),
            ..default()
        }))
        // Add grid plugin. `debug` toggles grid lines (default is false).
        // The plugin is generic over `Player`. Anything with this component
        // will get added to the grid. This allows you to create multiple grids
        // for distinct purposes.
        .add_plugins(UniformGrid2dPlugin::<Player>::default().debug(true))
        // The below creates a square 600x600 grid with the bottom left at the origin
        .insert_resource(
            Grid::<Player>::default()
                // Size of the grid (units are grid cells)
                .dimensions(UVec2::splat(30))
                // Size of each grid cell (units are integer world-space coordinates)
                .spacing(UVec2::splat(20))
                // You can anchor the grid somewhere specific (default is the origin)
                // .anchor(Vec2::new(23.4, 10.1))
        )
        .add_systems(Startup, setup)
        .add_systems(Update, handle_grid_changes)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    // Add a camera
    commands.spawn((Camera2d, Transform::from_xyz(300., 300., 0.)));

    commands.spawn((
        // Add a sprite so we can visualize the entity
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(10.0)),
            ..default()
        },
        // Entities with a `Transform` are automatically added to the grid
        Transform::from_xyz(300., 300., 0.),
        // Player marker for movement handling
        Player,
    ));
}

fn handle_grid_changes(
    grid: Res<Grid<Player>>,
    // The current grid cell of an entity is synced to `GridCell`
    grid_cells: Query<&GridCell, With<Player>>,
    mut events: EventReader<GridEvent>,
) {
    // Events are emitted any time an entity enters, leaves, or changes which grid cell it's in
    for &GridEvent { entity, operation } in events.read() {
        // The grid `operation` can be `Insert`, `Remove`, or `Update`
        info!("entity={entity} grid_event={operation}");

        if let GridOperation::Update { from, to } = operation {
            // Here we are checking all the entities in neighboring grid cells
            // whenever the entity in question changes the cell it's in
            for neighbor_entity in grid.iter_neighbors(to) {
                // ... attack neighbors?
            }
        }
    }
}

// Move with WASD
fn movement(
    mut transform: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut position = transform.single_mut().unwrap();

    let t = time.delta_secs();
    let up = keyboard.any_pressed([KeyCode::KeyW]);
    let down = keyboard.any_pressed([KeyCode::KeyS]);
    let left = keyboard.any_pressed([KeyCode::KeyA]);
    let right = keyboard.any_pressed([KeyCode::KeyD]);

    let x = -(left as i8) + right as i8;
    let y = -(down as i8) + up as i8;

    let mut move_delta = Vec2::new(x as f32, y as f32);
    if move_delta != Vec2::ZERO {
        move_delta /= move_delta.length();
        move_delta *= t * 100.;
    }
    position.translation += move_delta.extend(0.);
}
```

### Bevy Version Support
| bevy | bevy_uniform_grid_2d |
| ---- | -------------------  |
| 0.16 | 0.4                  |
| 0.15 | 0.3                  |
| 0.14 | 0.2                  |
| 0.13 | 0.1                  |
