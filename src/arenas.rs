use bevy::prelude::*;
use crate::constants::{GRID_HEIGHT, GRID_WIDTH, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE};
use crate::shared_traits::EnumDisplay;
use crate::state::GlobalState;



#[derive(Component)]
pub struct Arena {
    pub id: u8,
}
#[derive(Component)]
pub struct ArenasParentTransform;
#[derive(Component)]
pub struct ArenaBossText;

#[derive(Component)]
pub struct ArenaName(pub String);


pub enum ArenaNameEnum {
    Labyrinth,
    Sanctum,
    Pawnshop,
    Bastion,
    Mountain,
    Crucible,
    Casino,
    Gala,
    GuildHouse,
    Menu
}

impl EnumDisplay for ArenaNameEnum {
    fn to_display_string(&self) -> String {
        match self {
            // Hunter abilities
            ArenaNameEnum::Labyrinth => "Labyrinth",
            ArenaNameEnum::Sanctum => "Sanctum",
            ArenaNameEnum::Pawnshop => "Pawnshop",
            ArenaNameEnum::Bastion => "Bastion",
            ArenaNameEnum::Mountain => "Mountain",
            ArenaNameEnum::Crucible => "Crucible",
            ArenaNameEnum::Casino => "Casino",
            ArenaNameEnum::Gala => "Gala",
            ArenaNameEnum::GuildHouse => "Guild House",
            ArenaNameEnum::Menu => "---",
        }.to_string()
    }
}

fn update_arena_boss_text(
    mut query: Query<&mut Text, With<ArenaBossText>>,
    arenas: Query<(&Arena, &ArenaName)>,
    state: Res<GlobalState>,
) {
    let current_arena_id = state.current_arena;

    if let Some((_, arena_name)) = arenas.iter().find(|(arena, _)| arena.id == current_arena_id) {
        for mut text in &mut query {
            text.clear();
            text.push_str(&arena_name.0);
        }
    }
}

fn highlight_arena_system(mut gizmos: Gizmos, state: Res<GlobalState>) {
    if state.active_menu == false { return; }
    let current_arena_index = state.current_arena as usize;
    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;
    for i in 0..3 {
        let pos = Vec2::new(
            total_width * OFFSET_MATRIX[current_arena_index].x + i as f32,
            total_height * OFFSET_MATRIX[current_arena_index].y - (MENU_Y_OFFSET / 2.0) -  i as f32,
        );
        gizmos.rect_2d(
            pos,
            Vec2::new(total_width, total_height),
            Color::hsla(0.0, 0.0, 0.0, 1.0),
        );
    }
}




pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_arena_boss_text, highlight_arena_system));
    }
}
