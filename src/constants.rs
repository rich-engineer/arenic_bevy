use bevy::math::{Vec2, Vec3};

pub const RESOLUTION: (f32, f32) = (1280.0, 720.0);
pub const GRID_WIDTH: usize = 65;
pub const GRID_HEIGHT: usize = 31;
pub const TILE_SIZE: f32 = 19.0;
pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;

pub const ARENA_WIDTH_HALF: f32 = ARENA_WIDTH / 2.0;
pub const ARENA_HEIGHT_HALF: f32 = ARENA_HEIGHT / 2.0;

pub const MENU_SCALE: f32 = 3.0;
pub const MENU_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const GAME_SCALE: f32 = 1.0;
pub const ARENA_CENTER: Vec2 = Vec2::new(
    (ARENA_WIDTH / 2.0) - (TILE_SIZE / 2.0),
    -((ARENA_HEIGHT / 2.0) - (TILE_SIZE / 2.0)),
);