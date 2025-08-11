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
        .add_plugins(UniformGrid2dPlugin)
        .insert_resource(Grid {
            dimensions: UVec2::splat(50), // Size of the grid (units are grid cells)
            spacing: UVec2::splat(2),     // Size of each grid cell (units are integer world-space coordinates)
            // You can anchor the grid somewhere specific
            // anchor: Vec2::new(20., 10.)  
            ..Default::default()
        })
        .run()
}
```

### Bevy Version Support
| bevy | bevy_uniform_grid_2d |
| ---- | -------------------  |
| 0.15 | 0.1                  |
