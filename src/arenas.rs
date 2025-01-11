use crate::arena_components::{ActiveArena, Arena, ArenaBossText, ArenaName, ArenasParent, Bastion, Casino, Crucible, Gala, GuildHouse, Labyrinth, Menu, Mountain, Pawnshop, Sanctum, SelectedHero};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH, GRID_HEIGHT, GRID_WIDTH, HALF_TILE_SIZE, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE};
use crate::shared_traits::{ArenaTraits};
use crate::state::{GameState, GlobalState};
use bevy::prelude::*;



fn get_position(offset: Vec2) -> Vec3 {
    // Example logic, matching your original offset usage:
    let start_x = -(ARENA_WIDTH) + (TILE_SIZE / 2.0) + (ARENA_WIDTH * offset.x);
    let start_y = (ARENA_HEIGHT) + (TILE_SIZE - 1.0) + (ARENA_HEIGHT * offset.y);
    Vec3::new(start_x, start_y, 0.0)
}

fn spawn_single_arena<T: ArenaTraits + Component>(
    commands: &mut Commands,
    parent_entity: Entity,
    arena: T,
    texture: &Handle<Image>,
) {
    let offset = arena.offset_matrix();
    let display_string = arena.to_display_string();
    let grid_idx = arena.grid_index();

    commands
        .spawn((
            arena,
            Transform::from_translation(get_position(offset)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(display_string),
            Arena { id: grid_idx },
            SelectedHero(None),
        ))
        .set_parent(parent_entity).with_children(|parent| setup_tiles(parent, texture));
}

pub fn setup_all_arenas(
    mut commands: Commands,
    parent: Query<Entity, With<ArenasParent>>,
    asset_server: Res<AssetServer>,
) {
    let parent_entity = if let Ok(entity) = parent.get_single() {
        entity
    } else {
        commands
            .spawn((
                ArenasParent,
                Transform::from_xyz(0.0, 0.0, 0.0),
                InheritedVisibility::default(),
                GlobalTransform::default(),
            ))
            .id()
    };

    let texture0 = asset_server.load(Labyrinth.debug_tile());
    let texture1 = asset_server.load(GuildHouse.debug_tile());
    let texture2 = asset_server.load(Sanctum.debug_tile());
    let texture3 = asset_server.load(Mountain.debug_tile());
    let texture4 = asset_server.load(Bastion.debug_tile());
    let texture5 = asset_server.load(Pawnshop.debug_tile());
    let texture6 = asset_server.load(Crucible.debug_tile());
    let texture7 = asset_server.load(Casino.debug_tile());
    let texture8 = asset_server.load(Gala.debug_tile());

    // Spawn each arena individually
    spawn_single_arena(&mut commands, parent_entity, Labyrinth, &texture0); // 0
    spawn_single_arena(&mut commands, parent_entity, GuildHouse, &texture1); // 1
    spawn_single_arena(&mut commands, parent_entity, Sanctum, &texture2); // 2
    spawn_single_arena(&mut commands, parent_entity, Mountain, &texture3); // 3
    spawn_single_arena(&mut commands, parent_entity, Bastion, &texture4); // 4
    spawn_single_arena(&mut commands, parent_entity, Pawnshop, &texture5); // 5
    spawn_single_arena(&mut commands, parent_entity, Crucible, &texture6); // 6
    spawn_single_arena(&mut commands, parent_entity, Casino, &texture7); // 7
    spawn_single_arena(&mut commands, parent_entity, Gala, &texture8); // 8
}


pub fn setup_tiles(commands: &mut ChildBuilder, texture: &Handle<Image>) {
    for col in 0..GRID_WIDTH {
        for row in 0..GRID_HEIGHT {
            let x = col as f32 * TILE_SIZE;
            let y = -(row as f32 * TILE_SIZE);
            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: texture.clone(),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

fn update_arena_boss_text(
    mut text_query: Query<&mut Text, With<ArenaBossText>>,
    // We only query for the arena that is active:
    active_arena_query: Query<(&Arena, &ArenaName), With<ActiveArena>>,
) {
    // Get the first (and presumably only) active arena:
    let Ok((arena, arena_name)) = active_arena_query.get_single() else {
        warn!("No active Arena found boss Text");
        return;
    };

    // Update the arena boss text with the active arena's name:
    for mut text in &mut text_query {
        text.clear();
        text.push_str(&arena_name.0);
    }
}

fn highlight_arena_system(
    mut gizmos: Gizmos,
    active_arena: Query<(&Arena, &Transform), With<ActiveArena>>,
    menu_state: Res<GlobalState>,
) {
    if !menu_state.active_menu {
        return;
    }

    if let Ok((arena, _)) = active_arena.get_single() {
        let current_arena_index = arena.id as usize;

        for i in 0..3 {
            let pos = Vec2::new(
                -(ARENA_WIDTH/2.0 - HALF_TILE_SIZE) + ARENA_WIDTH * OFFSET_MATRIX[current_arena_index].x + i as f32,
                (ARENA_HEIGHT/2.0 + TILE_SIZE) + ARENA_HEIGHT * OFFSET_MATRIX[current_arena_index].y - (MENU_Y_OFFSET / 2.0) - i as f32,
            );
            gizmos.rect_2d(
                pos,
                Vec2::new(ARENA_WIDTH, ARENA_HEIGHT),
                Color::hsla(0.0, 0.0, 0.0, 1.0),
            );
        }
    }
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_all_arenas);
        app.add_systems(Update, (highlight_arena_system, update_arena_boss_text).run_if(in_state(GameState::Intro)));
    }
}