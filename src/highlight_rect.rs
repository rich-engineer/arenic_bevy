use bevy::color::palettes::tailwind::{GRAY_400, GRAY_50, GRAY_950, RED_400};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::arenas::{ArenaNameEnum};
use crate::characters::CharacterClassEnum;
use crate::shared_traits::EnumDisplay;
use crate::state::{GlobalState};

pub struct HighlightRectPlugin;

impl Plugin for HighlightRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (camera_setup, create_ui, setup_all_arenas));
        app.add_systems(Update, (
            handle_camera_input,
            highlight_arena_system,
            update_arena_boss_text,
            update_camera.after(handle_camera_input),

        ));
    }
}

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

const PROGRESS_BAR_HEIGHT: f32 = 8.0;
const FONT_SIZE: f32 = 9.0;

// Magic number just get over it;
const MENU_Y_OFFSET: f32 = -53.0;
const TOTAL_ARENAS_LENGTH: usize = 9;
const MENU_SCALE: f32 = 3.0;
const MENU_POS: Vec3 =  Vec3::new(
    0.0,
    MENU_Y_OFFSET,
    0.0
);
const GAME_SCALE: f32 = 1.0;
const GRID_WIDTH: usize = 65;
const GRID_HEIGHT: usize = 31;
const TILE_SIZE: f32 = 19.0;
const OFFSET_MATRIX: [Vec2; TOTAL_ARENAS_LENGTH] = [
    Vec2::new(-1.0, 1.0), // 0
    Vec2::new(0.0, 1.0), // 1
    Vec2::new(1.0, 1.0), // 2
    Vec2::new(-1.0, 0.0), // 3
    Vec2::new(0.0, 0.0), // 4
    Vec2::new(1.0, 0.0), // 5
    Vec2::new(-1.0, -1.0), // 6
    Vec2::new(0.0, -1.0), // 7
    Vec2::new(1.0, -1.0), // 8
];

fn get_current_arena_pos(global_state: &Res<GlobalState>) -> Vec3 {
    let current_arena = global_state.current_arena as usize;
    let offset_x = GRID_WIDTH as f32 * TILE_SIZE;
    let offset_y = GRID_HEIGHT as f32  * TILE_SIZE;

    Vec3::new(
        offset_x * OFFSET_MATRIX[current_arena].x - 1.0,
        offset_y * OFFSET_MATRIX[current_arena].y - 1.0,
        0.0
    )
}
pub fn camera_setup(mut commands: Commands, global_state: Res<GlobalState>) {
    let new_position = get_current_arena_pos(&global_state);
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::from(GRAY_50)),
            ..Default::default()
        },
        OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5,0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
        Transform {
            translation: new_position,
            ..Default::default()
        },
    ));
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

fn handle_camera_input(mut global_state: ResMut<GlobalState>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        global_state.current_arena = (global_state.current_arena + 9 - 1) % 9;
    }
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        global_state.current_arena = (global_state.current_arena + 1) % 9;
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        global_state.active_menu = !global_state.active_menu;
    }
}
fn update_camera(
    state: Res<GlobalState>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {

    let Ok((mut projection, mut transform)) = query.get_single_mut() else { return };

    let (scale, position) = if state.active_menu {
        (MENU_SCALE, MENU_POS)
    } else {
        (GAME_SCALE, get_current_arena_pos(&state))
    };

    projection.scale = scale;
    transform.translation = position;
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