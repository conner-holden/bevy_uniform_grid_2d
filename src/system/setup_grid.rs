use bevy::{
    color::{Alpha, palettes::tailwind},
    ecs::system::Res,
    gizmos::gizmos::Gizmos,
};
use glam::Vec2;

use crate::resource::Grid;

pub(crate) fn setup_grid(mut gizmos: Gizmos, grid: Res<Grid>) {
    let min = grid.anchor.as_ivec2();
    let max = (grid.dimensions * grid.spacing).as_ivec2() + min;

    for x in 0..=grid.dimensions.x {
        let x = (x * grid.spacing.x) as f32 + min.x as f32;
        let start = Vec2::new(x, min.y as f32);
        let end = Vec2::new(x, max.y as f32);
        gizmos.line_2d(start, end, tailwind::GRAY_300.with_alpha(0.03));
    }
    for y in 0..=grid.dimensions.y {
        let y = (y * grid.spacing.y) as f32 + min.y as f32;
        let start = Vec2::new(min.x as f32, y);
        let end = Vec2::new(max.x as f32, y);
        gizmos.line_2d(start, end, tailwind::GRAY_300.with_alpha(0.03));
    }
}
