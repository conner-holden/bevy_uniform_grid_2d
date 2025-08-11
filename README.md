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
        .insert_resource(Grid::new(UVec2::splat(50), UVec2::splat(2), Vec2::ZERO))
        .run()
}
```

### Bevy Version Support
| bevy | bevy_uniform_grid_2d |
| ---- | -------------------  |
| 0.15 | 0.1                  |
