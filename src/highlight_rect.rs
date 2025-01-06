use bevy::prelude::*;
use bevy::color::palettes::tailwind::{GRAY_400, GRAY_950, RED_400};
use crate::arenas::{Arena, ArenaBossText, ArenaName, ArenaNameEnum, ArenasParentTransform};
use crate::characters::CharacterClassEnum;
use crate::constants::{FONT_SIZE, GRID_HEIGHT, GRID_WIDTH,OFFSET_MATRIX, PROGRESS_BAR_HEIGHT, TILE_SIZE, TOTAL_ARENAS_LENGTH};
use crate::shared_traits::EnumDisplay;
use crate::state::{GlobalState};

pub struct HighlightRectPlugin;

impl Plugin for HighlightRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ( create_ui, setup_all_arenas));

    }
}

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/DMSans-Black.ttf");
    // let name = get_arena_name(&state);
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ))
        .with_children(|parent| create_top_navigation(parent, "Hunter", font))
        .with_children(create_inner_container)
        .with_children(create_bottom_bar);
}
fn spawn_arena_boss(parent: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    parent.spawn((
        ArenaBossText,
        Node {
            margin: UiRect {
                left: Val::Px(PROGRESS_BAR_HEIGHT * 3.0),
                right: Default::default(),
                top: Val::Px(PROGRESS_BAR_HEIGHT * 2.0),
                bottom: Default::default(),
            },
            width: Val::Percent(100.0),
            display: Display::Flex,
            ..default()
        },
        Text::new(text),
        TextFont {
            font,
            font_size: FONT_SIZE,
            ..default()
        },
        TextColor(Color::Srgba(GRAY_950)),
        TextLayout::new_with_justify(JustifyText::Left),
    ));
}


fn create_top_navigation(mut commands: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    let top_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
       Node {
           position_type: PositionType::Relative,
           width: Val::Percent(100.0),
           height: Val::Percent(5.9),
           ..default()
       },
       BackgroundColor(top_bar_color),
    ))
        .with_children(spawn_progress_bar)
        .with_children(|parent| spawn_arena_boss(parent, text, font));
}
fn spawn_progress_bar(parent: &mut ChildBuilder) {
    let boss_current_health: f32 = 90.0;
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                height: Val::Px(PROGRESS_BAR_HEIGHT),
                width: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(GRAY_400)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    height: Val::Px(PROGRESS_BAR_HEIGHT),
                    width: Val::Percent(boss_current_health),
                    ..default()
                },
                BackgroundColor(Color::Srgba(RED_400)),
            ));
        });
}
fn create_inner_container(mut commands: &mut ChildBuilder) {
    // no color, it only for spacing
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(92.0),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).with_children(create_left_navigation)
        .with_children(create_right_navigation);
}
fn create_left_navigation(mut commands: &mut ChildBuilder) {
    let left_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(left_bar_color),
    ));
}
fn create_right_navigation(mut commands: &mut ChildBuilder) {
    let left_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(left_bar_color),
    ));
}
fn create_bottom_bar(mut commands: &mut ChildBuilder) {
    let bottom_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            height: Val::Percent(14.3),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(bottom_bar_color),
    ));
}

pub fn setup_all_arenas(mut commands:Commands,  asset_server: Res<AssetServer>) {

    commands
        .spawn((
            ArenasParentTransform,
            Transform::from_xyz(0.0, 0.0, 0.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
        ))
        .with_children(|arena| {
            for i in 0..TOTAL_ARENAS_LENGTH  {
                let arena_id = i as u8;
                let total_width = GRID_WIDTH as f32 * TILE_SIZE;
                let total_height = GRID_HEIGHT as f32 * TILE_SIZE;
                let offset = OFFSET_MATRIX[i];
                let texture = match i {
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


                let start_x = -(total_width / 2.0) + (TILE_SIZE / 2.0) + (total_width * offset.x);
                let start_y = (total_height / 2.0) + (TILE_SIZE - 1.0) + (total_height * offset.y);

                arena
                    .spawn((
                        Arena { id: arena_id },
                        ArenaName(get_arena_name_for_id(arena_id)),
                        // ArenaBosses,
                        // ArenaHeroes,
                        Transform::from_xyz(start_x, start_y, 0.0),
                        InheritedVisibility::default(),
                        GlobalTransform::default(),
                    ))
                    .with_children(|parent| setup_tiles(parent, texture));
            }
        });
}
fn get_arena_name_for_id(arena_id: u8) -> String {
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
    }.to_display_string().to_uppercase()
}
pub fn setup_tiles(mut commands: &mut ChildBuilder, texture: Handle<Image>) {
    for col in 0..GRID_WIDTH {
        for row in 0..GRID_HEIGHT {
            let x = col as f32 * TILE_SIZE;
            let y = - (row as f32 * TILE_SIZE);
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

fn get_arena_name(
    state: &Res<GlobalState>,
) -> String {
    let current_arena= state.current_arena;

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
    }.to_display_string().to_uppercase()
}