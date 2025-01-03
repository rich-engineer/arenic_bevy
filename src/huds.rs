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

fn top_navigation(mut commands: Commands, asset_server: Res<AssetServer>, global_state: Res<GlobalState>, arena_camera_position: Res<ArenaCameraPositions>) {
    let font = asset_server.load("fonts/DMSans-Black.ttf");
    let display_text = if let Some(current_arena) = arena_camera_position.0.get(global_state.current_arena as usize) {
        info!("Current Selected Name: {:?}", current_arena.1);
        current_arena.1.to_string()
    } else {
        CharacterClassEnum::Menu.to_string()
    };

    info!(global_state.current_arena);
    commands.spawn((
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
            height: Val::Px(36.0),

            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ))
        .with_children(|parent| {
        parent.spawn((
           Node {
               position_type: PositionType::Absolute,
               top: Val::Px(0.0),
               left: Val::Px(0.0),
               right: Val::Px(0.0),
               height: Val::Px(8.0),
               width: Val::Percent(100.0),
               ..default()
           },
           BackgroundColor(Color::Srgba(GRAY_400)),
        )).with_children(|parent_2| {
            parent_2.spawn((
               Node {
                   position_type: PositionType::Absolute,
                   top: Val::Px(0.0),
                   left: Val::Px(0.0),
                   height: Val::Px(8.0),
                   width: Val::Percent(90.0),
                   ..default()
               },
               BackgroundColor(Color::Srgba(RED_400)),
            ));
        });
    })
        .with_children(|parent| {
            parent.spawn((
               Node {
                   width: Val::Percent(100.0),
                   display: Display::Flex,
                   ..default()
               },
               Text::new(&display_text),
               TextFont {
                   font,
                   font_size: 9.0,
                   ..default()
               },
               TextColor(Color::Srgba(GRAY_950)),
               TextLayout::new_with_justify(JustifyText::Left),
            ));
        });
}
fn bottom_navigation(mut commands: Commands, ) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(37.0),
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Px(40.0),
            height: Val::Percent(100.0),

            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(37.0),
            left: Val::Px(1240.0),
            bottom: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Px(40.0),
            height: Val::Percent(100.0),

            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(627.0),
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Px(93.0),

            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}


pub struct HudsPlugin;

impl Plugin for HudsPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, (top_navigation, bottom_navigation));
    }
}