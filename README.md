# bevy_uniform_grid_2d
[![CI](https://github.com/conner-holden/bevy_uniform_grid_2d/workflows/CI/badge.svg)](https://github.com/conner-holden/bevy_uniform_grid_2d/actions)
[![Crates.io](https://img.shields.io/crates/v/bevy_uniform_grid_2d.svg)](https://crates.io/crates/bevy_uniform_grid_2d)
[![Docs](https://docs.rs/bevy_uniform_grid_2d/badge.svg)](https://docs.rs/bevy_uniform_grid_2d/latest/bevy_uniform_grid_2d/)
[![Downloads](https://img.shields.io/crates/d/bevy_uniform_grid_2d.svg)](https://crates.io/crates/bevy_uniform_grid_2d)
![Issues](https://img.shields.io/github/issues/conner-holden/bevy_uniform_grid_2d)
![Closed Issues](https://img.shields.io/github/issues-closed/conner-holden/bevy_uniform_grid_2d)

An easy-to-use plugin for people who need basic spatial indexing.

## Installation
```sh
cargo add bevy_uniform_grid_2d
```

## Quickstart
```rust
// Add the import
use bevy_uniform_grid_2d::prelude::*;
```

```rust
// Create a marker to opt entities into the grid
#[derive(Component)]
struct MyMarker;
```

```rust
// Add the plugin to initialize grid (debug is optional)
.add_plugins(UniformGrid2dPlugin::<MyMarker>::default()
    .debug(true)
    .dimensions(UVec2::splat(30)) // Size of the grid in units of grid cells
    .spacing(UVec2::splat(20)), // Size of each cell in units of integer world coordinates
)
```

```rust
// Spawn an entity
commands.spawn((
    // Visualize the entity (optional)
    Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::splat(10.0)),
        ..default()
    },
    Transform::from_xyz(300., 300., 0.), // Track position (required)
    MyMarker, // Opt entity into the grid (required)
));
```

## Examples

### Minimal
For the [minimal example](examples/minimal.rs), the UI shows the current grid cell and all grid changes are logged. The player can move the sprite with WASD.

```sh
cargo run --example minimal
```

<details>
  <summary>Code</summary>

```rust
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;

fn main() {
    App::new()
        // Add default pluugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(800., 800.),
                title: "Minimal Example".to_string(),
                present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
                ..default()
            }),
            ..default()
        }))
        // Add grid plugin. `debug` toggles grid lines (default is false).
        //
        // The plugin is generic over `Player`. Anything with this component
        // will get added to the grid. This allows you to create multiple grids
        // for distinct purposes.
        //
        // The below creates a square 600x600 grid with the bottom left at the origin
        .add_plugins(UniformGrid2dPlugin::<Player>::default().debug(true)
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
        .add_systems(Update, update_ui)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct GridCellUI;

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

    // Add UI for grid cell display
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Grid Cell: N/A"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                GridCellUI,
            ));
        });
}

fn handle_grid_changes(
    grid: Res<Grid<Player>>,
    // The current grid cell of an entity is synced to `GridCell`
    grid_cells: Query<&GridCell, With<Player>>,
    mut events: EventReader<GridEvent>,
) {
    // Events are emitted any time an entity enters, leaves, or changes which grid cell it's in
    for event in events.read() {
        // The grid `operation` can be `Insert`, `Remove`, or `Update`
        info!("{}", event);

        if let GridOperation::Update { from, to } = event.operation {
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

// Display current grid cell
fn update_ui(
    player_query: Query<&Transform, With<Player>>,
    mut ui_query: Query<&mut Text, With<GridCellUI>>,
    grid: Res<Grid<Player>>,
) {
    if let (Ok(transform), Ok(mut text)) = (player_query.single(), ui_query.single_mut()) {
        match grid.world_to_grid(transform.translation) {
            Ok(cell) => {
                **text = format!("Grid Cell: ({}, {})", cell.x, cell.y);
            }
            Err(_) => {
                **text = "Grid Cell: Out of bounds".to_string();
            }
        }
    }
}
```
</details>

### Stress Test
For the [stress_test example](examples/stress_test.rs), 1000 entities are spawned and move in random directions.
Their color changes when they enter or leave the grid, or when their grid cell changes.

```sh
cargo run --example stress_test
```

<details>
  <summary>Code</summary>

```rust
use bevy::{color::palettes::tailwind, prelude::*, window::WindowResolution};
use bevy_uniform_grid_2d::prelude::*;
use iyes_perf_ui::{
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
    prelude::*,
};
use rand::Rng;

// Colors that are toggled as the sprite moves inside (and outside) the grid
const ON: Color = Color::Srgba(tailwind::GRAY_200);
const OFF: Color = Color::Srgba(tailwind::RED_500);
const OUT: Color = Color::Srgba(tailwind::GRAY_950);

// Pre-allocated capacity for each grid cell (see plugin initialization)
const N: usize = 8;

fn main() {
    let mut app = App::new();
    // Setup window
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800., 800.),
            title: "Stress Test Example".to_string(),
            present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
            ..default()
        }),
        ..default()
    }))
    // Add performance UI
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugins(PerfUiPlugin)
    .add_plugins(
        // Add grid plugin. `Marker` is a marker component for opting entities into the grid.
        // Our const `N` sets pre-allocated capacity of 8 for each grid cell. Default is 4.
        UniformGrid2dPlugin::<Marker, N>::default()
            .debug(true)
            // The grid shape is defined using the plugin's builder methods.
            .dimensions(UVec2::splat(30))
            .spacing(UVec2::splat(20)),
    )
    // Change direction of sprites every 3 seconds
    .insert_resource(ChangeDirectionTimer(Timer::from_seconds(
        3.,
        TimerMode::Repeating,
    )))
    .add_systems(Startup, setup)
    .add_systems(Update, movement)
    .add_systems(Update, update_color)
    .run();
}

#[derive(Resource)]
struct ChangeDirectionTimer(Timer);

#[derive(Component)]
struct Direction(Vec2);

// Marker for opting entities into the grid
#[derive(Component)]
struct Marker;

fn setup(mut commands: Commands, grid: Res<Grid<Marker, N>>) {
    // Add performance diagnostics UI
    commands.spawn((
        PerfUiRoot::default(),
        // Contains everything related to FPS and frame time
        PerfUiFramerateEntries::default(),
        // Contains everything related to the window and cursor
        PerfUiWindowEntries::default(),
        // Contains everything related to system diagnostics (CPU, RAM)
        PerfUiSystemEntries::default(),
        // Contains everything related to fixed timestep
        PerfUiFixedTimeEntries::default(),
    ));

    // Spawn 1000 sprites randomly within (and possibly a little outside) the grid
    let mut rng = rand::thread_rng();
    let padding = 50.;
    let max = (grid.dimensions() * grid.spacing()).as_vec2() + Vec2::splat(padding) + grid.anchor();
    let min = Vec2::splat(-padding) + grid.anchor();

    let entity_count = 1000;
    let entity_size = Vec2::splat(5.);
    for _ in 0..entity_count {
        let position = Vec2::new(rng.gen_range(min.x..max.x), rng.gen_range(min.y..max.y));
        let direction = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

        commands.spawn((
            Sprite {
                color: OUT,
                custom_size: Some(entity_size),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 10.),
            Direction(direction),
            Marker,
        ));
    }

    // Add camera
    commands.spawn((Camera2d, Transform::from_xyz(max.x / 2., max.y / 2., 0.)));
}

// Move sprites in their current `Direction` with a speed of 10.0
fn movement(
    time: Res<Time>,
    mut direction_timer: ResMut<ChangeDirectionTimer>,
    mut query: Query<(&mut Transform, &mut Direction)>,
) {
    let mut rng = rand::thread_rng();
    let t = time.delta_secs();
    direction_timer.0.tick(time.delta());
    let change_direction = direction_timer.0.just_finished();
    for (mut transform, mut direction) in &mut query {
        if change_direction {
            *direction = Direction(
                Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize(),
            );
        }
        transform.translation += t * 10. * direction.0.extend(0.);
    }
}

// Update the sprite's color whenever it enters or leaves the grid,
// as well as whenever it moves to a new grid cell
fn update_color(mut sprites: Query<&mut Sprite>, mut events: EventReader<GridEvent>) {
    for &GridEvent { entity, operation } in events.read() {
        let Ok(mut sprite) = sprites.get_mut(entity) else {
            continue;
        };
        match operation {
            GridOperation::Update { .. } => {
                if sprite.color == ON {
                    sprite.color = OFF;
                } else {
                    sprite.color = ON;
                }
            }
            GridOperation::Insert { .. } => {
                let mut rng = rand::thread_rng();
                sprite.color = if rng.gen_bool(0.5) { OFF } else { ON };
            }
            GridOperation::Remove { .. } => {
                sprite.color = OUT;
            }
        }
    }
}
```
</details>

### Grid Resizing
For the [resize example](examples/resize.rs), the UI shows the current grid cell and all grid changes are logged. Press `<SPACEBAR>` to shuffle the grid size.

```sh
cargo run --example resize
```

<details>
  <summary>Code</summary>

```rust
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        // Add default pluugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(800., 800.),
                title: "Resize Example".to_string(),
                present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
                ..default()
            }),
            ..default()
        }))
        // Add grid plugin. `debug` toggles grid lines (default is false).
        //
        // The plugin is generic over `Player`. Anything with this component
        // will get added to the grid. This allows you to create multiple grids
        // for distinct purposes.
        //
        // Unless `dimensions()`, `spacing()`, or `anchor()` are called, a default
        // 1x1 grid will be inserted into the world.
        .add_plugins(UniformGrid2dPlugin::<Player>::default().debug(true))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, shuffle_grid_size)
        .add_systems(Update, log_grid_events)
        .add_systems(Update, update_ui)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct GridCellUI;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Ready,
}

fn setup(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
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
        Transform::from_xyz(200., 200., 0.),
        // Player marker for movement handling
        Player,
    ));

    // Add UI for grid cell display
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Grid Cell: N/A"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                GridCellUI,
            ));
        });

    app_state.set(AppState::Ready);
}

fn shuffle_grid_size(
    keyboard: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut grid: EventWriter<TransformGridEvent<Player>>,
) {
    // If the user presses the spacebar or the app state just changed to ready,
    // change the grid to a random size
    if keyboard.just_pressed(KeyCode::Space)
        || (app_state.is_changed() && *app_state.get() == AppState::Ready)
    {
        let mut rng = rand::thread_rng();
        grid.write(
            TransformGridEvent::default()
                .with_dimensions(UVec2::splat(rng.gen_range(10..20)))
                .with_spacing(UVec2::splat(rng.gen_range(15..25))),
        );
    }
}

fn log_grid_events(
    mut grid_events: EventReader<GridEvent>,
    mut transform_grid_events: EventReader<TransformGridEvent<Player>>,
) {
    // Grid events are emitted any time an entity enters, leaves, or changes which grid cell it's in
    for event in grid_events.read() {
        // The grid `operation` can be `Insert`, `Remove`, or `Update`
        info!("{}", event);
    }

    for event in transform_grid_events.read() {
        info!("{}", event);
    }
}

// Display current grid cell
fn update_ui(
    player_query: Query<&Transform, With<Player>>,
    mut ui_query: Query<&mut Text, With<GridCellUI>>,
    grid: Res<Grid<Player>>,
) {
    if let (Ok(transform), Ok(mut text)) = (player_query.single(), ui_query.single_mut()) {
        match grid.world_to_grid(transform.translation) {
            Ok(cell) => {
                **text = format!("Grid Cell: ({}, {})", cell.x, cell.y);
            }
            Err(_) => {
                **text = "Grid Cell: Out of bounds".to_string();
            }
        }
    }
}
```
</details>

## Bevy Version Support
| bevy | bevy_uniform_grid_2d |
| ---- | -------------------  |
| 0.16 | 0.4                  |
| 0.15 | 0.3                  |
| 0.14 | 0.2                  |
| 0.13 | 0.1                  |
