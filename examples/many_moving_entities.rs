use bevy::{prelude::*, window::WindowResolution};
use bevy_uniform_grid_2d::prelude::*;
use rand::Rng;

fn main() {
    let mut app = App::new();
    // Setup window
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800., 800.),
            title: "Many Moving Entities Example".to_string(),
            present_mode: bevy::window::PresentMode::Immediate, // Disable VSync to show max FPS
            ..default()
        }),
        ..default()
    }))
    // Add performance UI
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    // Add grid plugin. `Marker` is a marker component for opting entities into the grid.
    .add_plugins(UniformGrid2dPlugin::<Marker>::default().debug(true))
    .insert_resource(
        Grid::<Marker>::default()
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

fn setup(mut commands: Commands, grid: Res<Grid<Marker>>) {
    // Spawn 1000 sprites randomly within (and possibly a little outside) the grid
    let mut rng = rand::thread_rng();
    let padding = 50.;
    let max = (grid.dimensions * grid.spacing).as_vec2() + Vec2::splat(padding) + grid.anchor;
    let min = Vec2::splat(-padding) + grid.anchor;

    let entity_count = 1000;
    let entity_size = Vec2::splat(5.);
    for _ in 0..entity_count {
        let position = Vec2::new(rng.gen_range(min.x..max.x), rng.gen_range(min.y..max.y));
        let direction = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(0, 0, 0),
                    custom_size: Some(entity_size),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 10.),
                ..default()
            },
            Direction(direction),
            Marker,
        ));
    }

    // Add camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(max.x / 2., max.y / 2., 0.),
        ..default()
    });
}

// Move sprites in their current `Direction` with a speed of 10.0
fn movement(
    time: Res<Time>,
    mut direction_timer: ResMut<ChangeDirectionTimer>,
    mut query: Query<(&mut Transform, &mut Direction)>,
) {
    let mut rng = rand::thread_rng();
    let t = time.delta_seconds();
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
                if sprite.color == Color::rgb_u8(220, 220, 220) {
                    sprite.color = Color::rgb_u8(200, 0, 0);
                } else {
                    sprite.color = Color::rgb_u8(220, 220, 220);
                }
            }
            GridOperation::Insert { .. } => {
                let mut rng = rand::thread_rng();
                sprite.color = if rng.gen_bool(0.5) {
                    Color::rgb_u8(200, 0, 0)
                } else {
                    Color::rgb_u8(220, 220, 220)
                };
            }
            GridOperation::Remove { .. } => {
                sprite.color = Color::rgb_u8(0, 0, 0);
            }
        }
    }
}
