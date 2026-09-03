use bevy::prelude::*;

/// Tile的大小(px)
pub const TILE_SIZE: f32 = 16.0;
pub const TILE_OFFSET: f32 = 8.0;

/// 世界坐标转Tile坐标
pub fn world_to_tile(pos: Vec2) -> IVec2 {
    IVec2::new(
        ((pos.x + TILE_OFFSET) / TILE_SIZE).floor() as i32,
        ((pos.y + TILE_OFFSET) / TILE_SIZE).floor() as i32,
    )
}

/// Tile坐标转世界坐标
pub fn tile_to_world(tile: IVec2) -> Vec2 {
    Vec2::new(tile.x as f32 * TILE_SIZE, tile.y as f32 * TILE_SIZE)
}

pub fn y_sort(y: f32) -> f32 {
    y * 1.01
}
