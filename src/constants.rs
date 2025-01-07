use bevy::math::{Vec2, Vec3};

pub const PROGRESS_BAR_HEIGHT: f32 = 8.0;
pub const FONT_SIZE: f32 = 9.0;

// Magic number just get over it;
pub const MENU_Y_OFFSET: f32 = -53.0;
pub const TOTAL_ARENAS_LENGTH: usize = 9;
pub const MENU_SCALE: f32 = 3.0;
pub const MENU_POS: Vec3 = Vec3::new(0.0, MENU_Y_OFFSET, 0.0);
pub const GAME_SCALE: f32 = 1.0;
pub const GRID_WIDTH: usize = 65;
pub const GRID_HEIGHT: usize = 31;
pub const TILE_SIZE: f32 = 19.0;
pub const ARENA_WIDTH: f32 = GRID_WIDTH as f32 * TILE_SIZE;
pub const ARENA_HEIGHT: f32 = GRID_HEIGHT as f32 * TILE_SIZE;

pub const ARENA_CENTER: Vec2 = Vec2::new((ARENA_WIDTH / 2.0) - (TILE_SIZE / 2.0), -((ARENA_HEIGHT / 2.0) - (TILE_SIZE / 2.0)));
pub const OFFSET_MATRIX: [Vec2; TOTAL_ARENAS_LENGTH] = [
    Vec2::new(-1.0, 1.0),  // 0
    Vec2::new(0.0, 1.0),   // 1
    Vec2::new(1.0, 1.0),   // 2
    Vec2::new(-1.0, 0.0),  // 3
    Vec2::new(0.0, 0.0),   // 4
    Vec2::new(1.0, 0.0),   // 5
    Vec2::new(-1.0, -1.0), // 6
    Vec2::new(0.0, -1.0),  // 7
    Vec2::new(1.0, -1.0),  // 8
];
