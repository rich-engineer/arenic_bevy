use crate::arena_components::{
    Arena, ArenaBossText, ArenaName, ArenasParent, Bastion, Casino, Crucible, Gala, GuildHouse,
    Labyrinth, Mountain, Pawnshop, Sanctum, SelectedHero,
};
use crate::constants::{
    ARENA_HEIGHT, ARENA_WIDTH, GRID_HEIGHT, GRID_WIDTH, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE,
};
use crate::shared_traits::ComponentDisplay;
use crate::state::GlobalState;
use bevy::prelude::*;

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

    let get_position = |index: usize| -> Vec3 {
        let offset = OFFSET_MATRIX[index];
        let start_x = -(ARENA_WIDTH / 2.0) + (TILE_SIZE / 2.0) + (ARENA_WIDTH * offset.x);
        let start_y = (ARENA_HEIGHT / 2.0) + (TILE_SIZE - 1.0) + (ARENA_HEIGHT * offset.y);
        Vec3::new(start_x, start_y, 0.0)
    };
    let texture = asset_server.load("UI/default_tile.png");

    commands
        .spawn((
            Labyrinth,
            Transform::from_translation(get_position(0)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Labyrinth.to_display_string()),
            Arena { id: 0 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            GuildHouse,
            Transform::from_translation(get_position(1)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(GuildHouse.to_display_string()),
            Arena { id: 1 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Casino,
            Transform::from_translation(get_position(2)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Sanctum.to_display_string()),
            Arena { id: 2 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Mountain,
            Transform::from_translation(get_position(3)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Mountain.to_display_string()),
            Arena { id: 3 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Bastion,
            Transform::from_translation(get_position(4)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Bastion.to_display_string()),
            Arena { id: 4 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Pawnshop,
            Transform::from_translation(get_position(5)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Pawnshop.to_display_string()),
            Arena { id: 5 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Crucible,
            Transform::from_translation(get_position(6)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Crucible.to_display_string()),
            Arena { id: 6 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Casino,
            Transform::from_translation(get_position(7)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Casino.to_display_string()),
            Arena { id: 7 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));

    commands
        .spawn((
            Gala,
            Transform::from_translation(get_position(8)),
            GlobalTransform::default(),
            InheritedVisibility::default(),
            ArenaName(Gala.to_display_string()),
            Arena { id: 8 },
            SelectedHero(None),
        ))
        .set_parent(parent_entity)
        .with_children(|parent| setup_tiles(parent, texture.clone()));
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
