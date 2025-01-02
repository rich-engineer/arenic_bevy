use crate::state::GameState;
use bevy::prelude::*;

const GRID_WIDTH: usize = 63;
const GRID_HEIGHT: usize = 31;
const TILE_SIZE: f32 = 19.0;

#[derive(Component)]
struct GridUI;

fn draw_forager_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -((total_width * 1.5) + TILE_SIZE);
    let start_y = (total_height * 1.5) + TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

fn draw_alchemist_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -((total_width * 1.5) + TILE_SIZE);
    let start_y = -(total_height / 2.0) - TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_hunter_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -((total_width * 1.5) + TILE_SIZE);
    let start_y = total_height / 2.0;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_guild_house(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -total_width / 2.0 ;
    let start_y = total_height / 2.0;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_warrior_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -total_width / 2.0 ;
    let start_y = (total_height * 1.5) + TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_merchant_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = -total_width / 2.0 ;
    let start_y = -(total_height / 2.0) - TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

fn draw_cardinal_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = (total_width / 2.0) + TILE_SIZE;
    let start_y = (total_height * 1.5) + TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_thief_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = (total_width / 2.0) + TILE_SIZE;
    let start_y = total_height / 2.0;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn draw_bard_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Calculate the total width and height
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

    // Calculate start position (top-left of grid)
    let start_x = (total_width / 2.0) + TILE_SIZE;
    let start_y = -(total_height / 2.0) - TILE_SIZE;

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let x = start_x + (col as f32 * TILE_SIZE) + (TILE_SIZE / 2.0);
            let y = start_y - (row as f32 * TILE_SIZE) - (TILE_SIZE / 2.0);

            commands.spawn((
                GridUI,
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: asset_server.load("UI/default_tile.png"),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}
fn cleanup_grid(mut commands: Commands, query: Query<Entity, With<GridUI>>) {
    for entity in &query {
        // Remove this entity and all its children
        commands.entity(entity).despawn_recursive();
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Start), draw_hunter_arena)
            .add_systems(OnEnter(GameState::Start), draw_guild_house)
            .add_systems(OnEnter(GameState::Start), draw_warrior_arena)
            .add_systems(OnEnter(GameState::Start), draw_cardinal_arena)
            .add_systems(OnEnter(GameState::Start), draw_forager_arena)
            .add_systems(OnEnter(GameState::Start), draw_thief_arena)
            .add_systems(OnEnter(GameState::Start), draw_alchemist_arena)
            .add_systems(OnEnter(GameState::Start), draw_merchant_arena)
            .add_systems(OnEnter(GameState::Start), draw_bard_arena)
            .add_systems(OnExit(GameState::Start), cleanup_grid);
    }
}
