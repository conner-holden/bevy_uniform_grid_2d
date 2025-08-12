# bevy_uniform_grid_2d

### Installation
```sh
cargo add bevy_uniform_grid_2d
```

### Usage
```rust
use bevy::prelude::*;
use bevy_uniform_grid_2d::prelude::*;
use glam::UVec2;

fn main() {
    App::new()
        // `debug` toggles grid lines (default is false)
        .add_plugins(UniformGrid2dPlugin { debug: true }) 
        // The below creates a square 600x600 grid with the bottom left at the origin
        .insert_resource(Grid {
            // Size of the grid (units are grid cells)
            dimensions: UVec2::splat(30),
            // Size of each grid cell (units are integer world-space coordinates)
            spacing: UVec2::splat(20),
            // You can anchor the grid somewhere specific (default is the origin)
            // anchor: Vec2::new(23.4, 10.1)  
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, handle_grid_changes)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn((
        // Add a sprite so we can visualize the entity
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(10.0)),
            ..Default::default()
        },
        // Entities with a `Transform` are automatically added to the grid
        Transform::from_xyz(300., 300., 0.),
    ));
}

fn handle_grid_changes(
    grid: Res<Grid>,
    sprites: Query<&GridCell, With<Sprite>>,
    mut events: EventReader<GridEvent>,
) {
    // Events are emitted any time an entity enters, leaves, or changes
    // which grid cell it's in
    for &GridEvent { entity, op } in events.read() {
        // The `op` is the grid operation, which can be `Insert`, `Remove`, or `Update`
        info!("entity {entity} cell position in grid: {op}");
        
        if let GridOp::Update { from, to } = op {
            // Here we are checking all the entities in neighboring grid cells
            // whenever the entity in question changes the cell it's in
            for neighbor_entity in grid.iter_neighbors(to) {
                // ... attack neighbors?
            }
        }
    }
}
```

### Bevy Version Support
| bevy | bevy_uniform_grid_2d |
| ---- | -------------------  |
| 0.15 | 0.1                  |
