use crate::characters::CharacterClassEnum;
use crate::constants::{
    ARENA_HEIGHT, ARENA_WIDTH, GRID_HEIGHT, GRID_WIDTH, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE,
    TOTAL_ARENAS_LENGTH,
};
use crate::shared_traits::EnumDisplay;
use crate::state::GlobalState;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Arena {
    pub id: u8,
}
#[derive(Component)]
pub struct ArenasParent;
#[derive(Component)]
pub struct ArenaBossText;

#[derive(Component)]
pub struct SelectedHero(pub Option<Entity>);

#[derive(Component)]
pub struct ArenaName(pub String);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    Menu,
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
        }
        .to_string()
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

fn highlight_arena_system(mut gizmos: Gizmos, state: Res<GlobalState>) {
    if state.active_menu == false {
        return;
    }
    let current_arena_index = state.current_arena as usize;
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

pub fn get_arena_boss_name(state: &Res<GlobalState>) -> String {
    let current_arena = state.current_arena;

    match current_arena {
        0 => CharacterClassEnum::Hunter,
        1 => CharacterClassEnum::GuildMaster,
        2 => CharacterClassEnum::Cardinal,
        3 => CharacterClassEnum::Forager,
        4 => CharacterClassEnum::Warrior,
        5 => CharacterClassEnum::Thief,
        6 => CharacterClassEnum::Alchemist,
        7 => CharacterClassEnum::Merchant,
        8 => CharacterClassEnum::Bard,
        _ => CharacterClassEnum::Menu,
    }
    .to_display_string()
    .to_uppercase()
}

pub fn get_arena_name_for_id(arena_id: u8) -> String {
    match arena_id {
        0 => ArenaNameEnum::Labyrinth,
        1 => ArenaNameEnum::GuildHouse,
        2 => ArenaNameEnum::Sanctum,
        3 => ArenaNameEnum::Mountain,
        4 => ArenaNameEnum::Bastion,
        5 => ArenaNameEnum::Pawnshop,
        6 => ArenaNameEnum::Crucible,
        7 => ArenaNameEnum::Casino,
        8 => ArenaNameEnum::Gala,
        _ => ArenaNameEnum::Menu,
    }
    .to_display_string()
    .to_uppercase()
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

    for i in 0..TOTAL_ARENAS_LENGTH {
        let arena_id = i as u8;
        let offset = OFFSET_MATRIX[i];
        let texture = match 9 {
            0 => asset_server.load("UI/hunter_tile.png"),
            1 => asset_server.load("UI/guild_tile.png"),
            2 => asset_server.load("UI/cardinal_tile.png"),
            3 => asset_server.load("UI/forager_tile.png"),
            4 => asset_server.load("UI/warrior_tile.png"),
            5 => asset_server.load("UI/thief_tile.png"),
            6 => asset_server.load("UI/alchemist_tile.png"),
            7 => asset_server.load("UI/merchant_tile.png"),
            8 => asset_server.load("UI/bard_tile.png"),
            _ => asset_server.load("UI/default_tile.png"),
        };
        // move 4th quadrant + offset for tile size + Translate by ARENA_SIZE 1280 * 0 (make 4th quadrant)
        let start_x = -(ARENA_WIDTH / 2.0) + (TILE_SIZE / 2.0) + (ARENA_WIDTH * offset.x);
        let start_y = (ARENA_HEIGHT / 2.0) + (TILE_SIZE - 1.0) + (ARENA_HEIGHT * offset.y);

        commands
            .spawn((
                Arena { id: arena_id },
                ArenaName(get_arena_name_for_id(arena_id)),
                Transform::from_xyz(start_x, start_y, 0.0),
                InheritedVisibility::default(),
                GlobalTransform::default(),
                SelectedHero(None)
            ))
            .set_parent(parent_entity)
            .with_children(|parent| setup_tiles(parent, texture));
    }
}

pub fn setup_tiles(commands: &mut ChildBuilder, texture: Handle<Image>) {
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
        app.add_systems(Update, (update_arena_boss_text, highlight_arena_system));
    }
}
