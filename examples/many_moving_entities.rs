use bevy::{color::palettes::tailwind, prelude::*, window::WindowResolution};
use bevy_uniform_grid_2d::{
    event::{GridEvent, GridOp},
    prelude::*,
};
use glam::Vec2;
use rand::Rng;

const ON: Color = Color::Srgba(tailwind::GRAY_200);
const OFF: Color = Color::Srgba(tailwind::RED_500);
const OUT: Color = Color::Srgba(tailwind::GRAY_950);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800., 800.),
            title: "Many Moving Entities Example".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(UniformGrid2dPlugin)
    .insert_resource(Grid {
        dimensions: UVec2::splat(30),
        spacing: UVec2::splat(20),
        ..Default::default()
    })
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

pub fn setup(mut commands: Commands, grid: Res<Grid>) {
    let mut rng = rand::thread_rng();

    let padding = 10.;
    let max = (grid.dimensions * grid.spacing).as_vec2() + Vec2::splat(padding) + grid.anchor;
    let min = Vec2::splat(-padding) + grid.anchor;

    commands.spawn((Camera2d, Transform::from_xyz(max.x / 2., max.y / 2., 0.)));

    let entity_count = 1000;
    let entity_size = Vec2::splat(5.);
    for _ in 0..entity_count {
        let position = Vec2::new(rng.gen_range(min.x..max.x), rng.gen_range(min.y..max.y));
        let direction = Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

        commands.spawn((
            Sprite {
                color: OUT,
                custom_size: Some(entity_size),
                ..Default::default()
            },
            Transform::from_xyz(position.x, position.y, 10.),
            Direction(direction),
        ));
    }
}

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

fn update_color(mut sprites: Query<&mut Sprite>, mut events: EventReader<GridEvent>) {
    for &GridEvent { entity, op } in events.read() {
        let Ok(mut sprite) = sprites.get_mut(entity) else {
            continue;
        };
        match op {
            GridOp::Update { .. } => {
                if sprite.color == OFF {
                    sprite.color = ON;
                } else {
                    sprite.color = OFF;
                }
            }
            GridOp::Insert { .. } => {
                let mut rng = rand::thread_rng();
                sprite.color = if rng.gen_bool(0.5) { OFF } else { ON };
            }
            GridOp::Remove { .. } => {
                sprite.color = OUT;
            }
        }
    }
}
