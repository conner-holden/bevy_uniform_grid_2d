#![allow(clippy::unit_arg)]
#![allow(clippy::type_complexity)]

use bevy::{
    MinimalPlugins,
    app::{App, Startup, Update},
    ecs::{component::Component, entity::Entity},
    math::{UVec2, Vec2, Vec3},
    prelude::{Commands, Transform},
};
use bevy_uniform_grid_2d::{plugin::UniformGrid2dPlugin, resource::Grid};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::Rng;

const ENTITY_COUNT: usize = 10000;
const GRID_SIZE: u32 = 100;
const CELL_SIZE: u32 = 32;

criterion_group!(
    benches,
    grid_update_benchmark,
    grid_neighbor_benchmark,
    grid_insertion_benchmark,
);
criterion_main!(benches);

#[derive(Component)]
struct TestMarker;

#[derive(Component)]
struct Velocity(Vec3);

fn grid_update_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_update");
    group.warm_up_time(std::time::Duration::from_millis(500));

    group.bench_function("update_10k_entities", |b| {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(UniformGrid2dPlugin::<TestMarker>::default())
            .insert_resource(Grid::<TestMarker>::new(
                UVec2::splat(GRID_SIZE),
                UVec2::splat(CELL_SIZE),
                Vec2::ZERO,
            ))
            .add_systems(Startup, spawn_moving_entities)
            .add_systems(Update, move_entities);

        // Initialize
        app.update();

        b.iter(|| {
            black_box(app.update());
        });
    });

    group.bench_function("baseline_no_grid", |b| {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_moving_entities_no_marker)
            .add_systems(Update, move_entities_no_marker);

        // Initialize
        app.update();

        b.iter(|| {
            black_box(app.update());
        });
    });
}

fn grid_neighbor_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_neighbors");
    group.warm_up_time(std::time::Duration::from_millis(200));

    // Test different SmallVec sizes
    for &cell_capacity in &[1, 2, 4, 8, 16] {
        group.bench_function(format!("neighbors_capacity_{cell_capacity}"), |b| {
            let grid = create_populated_grid(cell_capacity);
            let center_cell = UVec2::new(GRID_SIZE / 2, GRID_SIZE / 2);

            b.iter(|| {
                let neighbors: Vec<Entity> = black_box(grid.iter_neighbors(center_cell).collect());
                black_box(neighbors.len());
            });
        });
    }
}

fn grid_insertion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_insertion");
    group.warm_up_time(std::time::Duration::from_millis(200));

    for &cell_capacity in &[1, 2, 4, 8, 16] {
        group.bench_function(
            format!("insert_1k_entities_capacity_{cell_capacity}"),
            |b| {
                b.iter(|| {
                    let mut grid = create_empty_grid(cell_capacity);
                    let mut rng = rand::thread_rng();

                    for i in 0..1000 {
                        let entity = Entity::from_raw(i as u32);
                        let cell =
                            UVec2::new(rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
                        black_box(grid.insert(entity, cell).ok());
                    }
                    black_box(grid);
                });
            },
        );
    }
}

fn spawn_moving_entities(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let world_size = (GRID_SIZE * CELL_SIZE) as f32;

    for _ in 0..ENTITY_COUNT {
        let position = Vec3::new(
            rng.gen_range(0.0..world_size),
            rng.gen_range(0.0..world_size),
            0.0,
        );
        let velocity = Vec3::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0), 0.0);

        commands.spawn((
            Transform::from_translation(position),
            Velocity(velocity),
            TestMarker,
        ));
    }
}

fn spawn_moving_entities_no_marker(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let world_size = (GRID_SIZE * CELL_SIZE) as f32;

    for _ in 0..ENTITY_COUNT {
        let position = Vec3::new(
            rng.gen_range(0.0..world_size),
            rng.gen_range(0.0..world_size),
            0.0,
        );
        let velocity = Vec3::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0), 0.0);

        commands.spawn((Transform::from_translation(position), Velocity(velocity)));
    }
}

fn move_entities(mut entities: bevy::ecs::system::Query<(&mut Transform, &Velocity)>) {
    let dt = 0.016; // ~60 FPS
    for (mut transform, velocity) in &mut entities {
        transform.translation += velocity.0 * dt;

        // Wrap around world bounds
        let world_size = (GRID_SIZE * CELL_SIZE) as f32;
        if transform.translation.x < 0.0 {
            transform.translation.x = world_size;
        } else if transform.translation.x > world_size {
            transform.translation.x = 0.0;
        }

        if transform.translation.y < 0.0 {
            transform.translation.y = world_size;
        } else if transform.translation.y > world_size {
            transform.translation.y = 0.0;
        }
    }
}

fn move_entities_no_marker(entities: bevy::ecs::system::Query<(&mut Transform, &Velocity)>) {
    move_entities(entities);
}

// Helper functions for micro-benchmarks
fn create_empty_grid(cell_capacity: usize) -> Box<dyn GridTrait> {
    match cell_capacity {
        1 => Box::new(Grid::<TestMarker, 1>::new(
            UVec2::splat(GRID_SIZE),
            UVec2::splat(CELL_SIZE),
            Vec2::ZERO,
        )),
        2 => Box::new(Grid::<TestMarker, 2>::new(
            UVec2::splat(GRID_SIZE),
            UVec2::splat(CELL_SIZE),
            Vec2::ZERO,
        )),
        4 => Box::new(Grid::<TestMarker, 4>::new(
            UVec2::splat(GRID_SIZE),
            UVec2::splat(CELL_SIZE),
            Vec2::ZERO,
        )),
        8 => Box::new(Grid::<TestMarker, 8>::new(
            UVec2::splat(GRID_SIZE),
            UVec2::splat(CELL_SIZE),
            Vec2::ZERO,
        )),
        16 => Box::new(Grid::<TestMarker, 16>::new(
            UVec2::splat(GRID_SIZE),
            UVec2::splat(CELL_SIZE),
            Vec2::ZERO,
        )),
        _ => panic!("Unsupported cell capacity"),
    }
}

fn create_populated_grid(cell_capacity: usize) -> Box<dyn GridTrait> {
    let mut grid = create_empty_grid(cell_capacity);
    let mut rng = rand::thread_rng();

    // Populate with entities
    for i in 0..5000 {
        let entity = Entity::from_raw(i);
        let cell = UVec2::new(rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
        let _ = grid.insert(entity, cell);
    }

    grid
}

// Trait to allow benchmarking different grid configurations
trait GridTrait {
    fn insert(
        &mut self,
        entity: Entity,
        cell: UVec2,
    ) -> Result<(), bevy_uniform_grid_2d::error::GridError>;
    fn iter_neighbors(&self, cell: UVec2) -> Box<dyn Iterator<Item = Entity> + '_>;
}

impl<const N: usize> GridTrait for Grid<TestMarker, N> {
    fn insert(
        &mut self,
        entity: Entity,
        cell: UVec2,
    ) -> Result<(), bevy_uniform_grid_2d::error::GridError> {
        self.insert(entity, cell)
    }

    fn iter_neighbors(&self, cell: UVec2) -> Box<dyn Iterator<Item = Entity> + '_> {
        Box::new(self.iter_neighbors(cell))
    }
}
