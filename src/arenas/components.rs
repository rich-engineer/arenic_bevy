use bevy::prelude::*;

/// Marks an entity as an "Arena" and stores metadata

#[derive(Component)]
pub struct ArenaRootComponent {
    pub arena_id: usize,
    pub arena_total_columns: u32,
    pub arena_total_rows: u32,
}

#[derive(Component)]
pub struct ArenaComponent {
    pub arena_id: usize,
    pub arena_name: ArenaNames,
    pub total_tiles_y: u32,
    pub total_tiles_x: u32,
    pub tile_width: f32,
    pub tile_height: f32,
    pub arena_debug_tile_color: Color,
}

#[derive(Component)]
pub struct TileComponent {
    pub x: u32,
    pub y: u32,
    pub parent_arena_id: usize,
}

pub enum ArenaNames {
    Hunter,
    Alchemist,
    Sprinter,
    Gatherer,
    Warrior,
    Cardinal,
    Bard,
    Thief,
}
