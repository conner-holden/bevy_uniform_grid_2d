use bevy::{
    color::{Alpha, palettes::tailwind},
    ecs::{component::Component, system::Res},
    gizmos::gizmos::Gizmos,
    math::Vec2,
};

use crate::resource::Grid;

pub(crate) fn update_debug_grid_lines<Marker: Component, const N: usize>(
    mut gizmos: Gizmos,
    grid: Res<Grid<Marker, N>>,
) {
    let min = grid.anchor();
    let max = grid.dimensions().as_vec2() * grid.spacing() + min;

    for x in 0..=grid.dimensions().x {
        let x = x as f32 * grid.spacing().x + min.x;
        let start = Vec2::new(x, min.y);
        let end = Vec2::new(x, max.y);
        gizmos.line_2d(start, end, tailwind::GRAY_300.with_alpha(0.03));
    }
    for y in 0..=grid.dimensions().y {
        let y = y as f32 * grid.spacing().y + min.y;
        let start = Vec2::new(min.x, y);
        let end = Vec2::new(max.x, y);
        gizmos.line_2d(start, end, tailwind::GRAY_300.with_alpha(0.03));
    }
}
