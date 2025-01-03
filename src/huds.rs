use bevy::color::palettes::tailwind::{GRAY_400, GRAY_50, GRAY_950, RED_400};
use bevy::prelude::*;
use crate::cameras::ArenaCameraPositions;
use crate::characters::CharacterClassEnum;
use crate::state::GlobalState;

const TOP_BAR_HEIGHT: f32 = 36.0;
const PROGRESS_BAR_HEIGHT: f32 = 8.0;
const SIDE_NAV_WIDTH: f32 = 40.0;
const BOTTOM_NAV_HEIGHT: f32 = 93.0;
const FONT_SIZE: f32 = 9.0;

// Define UI component markers
#[derive(Component)]
struct TopNavigation;

#[derive(Component)]
struct BottomNavigation;

fn get_arena_name(
    arena_camera_position: &Res<ArenaCameraPositions>,
    global_state: &Res<GlobalState>,
) -> String {
    let name = if let Some((_, class, _)) = arena_camera_position
        .0
        .get(global_state.current_arena as usize)
    {
        info!("Current Selected Name: {:?}", class);
        class.to_string()
    } else {
        CharacterClassEnum::Menu.to_string()
    };

    name.to_uppercase()
}

fn top_navigation(mut commands: Commands, asset_server: Res<AssetServer>, global_state: Res<GlobalState>, arena_camera_position: Res<ArenaCameraPositions>) {
    let font = asset_server.load("fonts/DMSans-Black.ttf");
    let display_text = get_arena_name(&arena_camera_position, &global_state);

    commands.spawn((
        TopNavigation,
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Px(TOP_BAR_HEIGHT),

            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ))
        .with_children(spawn_progress_bar)
        .with_children(|parent| spawn_arena_boss(parent, &display_text, font));
}

fn spawn_arena_boss(parent: &mut ChildBuilder, text: &str, font: Handle<Font>) {

    parent.spawn((
        Node {
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

fn spawn_progress_bar(parent: &mut ChildBuilder) {
    const boss_current_health: f32 = 90.0;
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
fn spawn_bottom_navigation(mut commands: Commands) {
    // Spawn left navigation
    spawn_side_nav(&mut commands, 0.0);
    // Spawn right navigation
    spawn_side_nav(&mut commands, 1240.0);
    // Spawn bottom bar
    spawn_bottom_bar(&mut commands);
}


fn spawn_side_nav(commands: &mut Commands, left_position: f32) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(TOP_BAR_HEIGHT + 1.0),
            left: Val::Px(left_position),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Px(SIDE_NAV_WIDTH),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}

fn spawn_bottom_bar(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(627.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Px(BOTTOM_NAV_HEIGHT),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}


pub struct HudsPlugin;

impl Plugin for HudsPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, (top_navigation, spawn_bottom_navigation));
    }
}