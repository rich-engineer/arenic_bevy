use crate::arena_components::{ActiveArena, Arena, ArenaBossText, ArenaName, ArenasParent, Bastion, Casino, Crucible, Gala, GuildHouse, Labyrinth, Menu, Mountain, Pawnshop, Sanctum, SelectedHero};
use crate::constants::{
    ARENA_HEIGHT, ARENA_WIDTH, GRID_HEIGHT, GRID_WIDTH, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE,
};
use crate::shared_traits::{ArenaTraits};
use crate::state::GlobalState;
use bevy::prelude::*;



fn get_position(offset: Vec2) -> Vec3 {
    // Example logic, matching your original offset usage:
    let start_x = -(ARENA_WIDTH / 2.0) + (TILE_SIZE / 2.0) + (ARENA_WIDTH * offset.x);
    let start_y = (ARENA_HEIGHT / 2.0) + (TILE_SIZE - 1.0) + (ARENA_HEIGHT * offset.y);
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

    let texture = asset_server.load("UI/default_tile.png");

    // Spawn each arena individually
    spawn_single_arena(&mut commands, parent_entity, Labyrinth, &texture); // 0
    spawn_single_arena(&mut commands, parent_entity, GuildHouse, &texture); // 1
    spawn_single_arena(&mut commands, parent_entity, Sanctum, &texture); // 2
    spawn_single_arena(&mut commands, parent_entity, Mountain, &texture); // 3
    spawn_single_arena(&mut commands, parent_entity, Bastion, &texture); // 4
    spawn_single_arena(&mut commands, parent_entity, Pawnshop, &texture); // 5
    spawn_single_arena(&mut commands, parent_entity, Crucible, &texture); // 6
    spawn_single_arena(&mut commands, parent_entity, Casino, &texture); // 7
    spawn_single_arena(&mut commands, parent_entity, Gala, &texture); // 8
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

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_all_arenas);
        // app.add_systems(Update, (update_arena_boss_text, highlight_arena_system));
    }
}


fn update_arena_boss_text(
    mut query: Query<&mut Text, With<ArenaBossText>>,
    arenas: Query<(&Arena, &ArenaName)>,
    state: Res<GlobalState>,
) {
    let current_arena_id = state.current_arena;

    if let Some((_, arena_name)) = arenas
        .iter()
        .find(|(arena, _)| arena.id == current_arena_id)
    {
        for mut text in &mut query {
            text.clear();
            text.push_str(&arena_name.0);
        }
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
        let total_width = GRID_WIDTH as f32 * TILE_SIZE;
        let total_height = GRID_HEIGHT as f32 * TILE_SIZE;

        for i in 0..3 {
            let pos = Vec2::new(
                total_width * OFFSET_MATRIX[current_arena_index].x + i as f32,
                total_height * OFFSET_MATRIX[current_arena_index].y - (MENU_Y_OFFSET / 2.0) - i as f32,
            );
            gizmos.rect_2d(
                pos,
                Vec2::new(total_width, total_height),
                Color::hsla(0.0, 0.0, 0.0, 1.0),
            );
        }
    }
}