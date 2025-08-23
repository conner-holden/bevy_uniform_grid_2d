#![allow(unused_variables)]
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;

#[rustfmt::skip]
fn main() {
    App::new()
        // Add default pluugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800., 800.).into(),
                title: "Multiple Grids Example".to_string(),
                present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UniformGrid2dPlugin::<Grid1>::default().debug(true)
                .dimensions(UVec2::splat(30))
                .spacing(Vec2::splat(20.))
        )
        .add_plugins(UniformGrid2dPlugin::<Grid2>::default().debug(true)
                .dimensions(UVec2::splat(30))
                .spacing(Vec2::splat(20.))
                .anchor(Vec2::new(310., 315.))
        )
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, update_ui)
        .run();
}

#[derive(Component)]
struct Grid1;

#[derive(Component)]
struct Grid2;

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
        // Player marker for movement handling
        Player,
        Grid1,
        Grid2,
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
    grid_1: Res<Grid<Grid1>>,
    grid_2: Res<Grid<Grid2>>,
) {
    let (transform, mut text) = (player_query.single(), ui_query.single_mut());
    text.sections[0].value = "".to_string();
    match grid_1.world_to_grid(transform.translation) {
        Ok(cell) => {
            text.sections[0].value += &format!("Grid Cell (1): ({}, {})", cell.x, cell.y);
        }
        Err(_) => {
            text.sections[0].value += "Grid Cell (1): Out of bounds";
        }
    }
    match grid_2.world_to_grid(transform.translation) {
        Ok(cell) => {
            text.sections[0].value += &format!("\nGrid Cell (2): ({}, {})", cell.x, cell.y);
        }
        Err(_) => {
            text.sections[0].value += "\nGrid Cell (2): Out of bounds";
        }
    }
}
