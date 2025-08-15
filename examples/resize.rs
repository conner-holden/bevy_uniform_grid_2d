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
