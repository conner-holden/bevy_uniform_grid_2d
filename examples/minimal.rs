#![allow(unused_variables)]
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;

#[rustfmt::skip]
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
                // Size of each grid cell (units are world-space coordinates)
                .spacing(Vec2::splat(20.))
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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(300., 300., 0.),
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            transform: Transform::from_xyz(300., 300., 0.),
            ..default()
        },
        // Player marker for movement and grid
        Player,
    ));

    // Add UI for grid cell display
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Grid Cell: N/A",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                GridCellUI,
            ));
        });
}

fn handle_grid_changes(
    grid: Res<Grid<Player>>,
    // The current grid cell of an entity is synced to `GridCell`
    grid_cells: Query<&GridCell<Player>>,
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
    let mut position = transform.single_mut();

    let t = time.delta_seconds();
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
    let (transform, mut text) = (player_query.single(), ui_query.single_mut());
    match grid.world_to_grid(transform.translation) {
        Ok(cell) => {
            text.sections[0].value = format!("Grid Cell: ({}, {})", cell.x, cell.y);
        }
        Err(_) => {
            text.sections[0].value = "Grid Cell: Out of bounds".to_string();
        }
    }
}
