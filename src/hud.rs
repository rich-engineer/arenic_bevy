use crate::arenas::ArenaBossText;
use crate::constants::{FONT_SIZE, PROGRESS_BAR_HEIGHT};
use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Handle};
use bevy::color::palettes::tailwind::{GRAY_400, GRAY_50, GRAY_950, RED_400};
use bevy::color::Color;
use bevy::hierarchy::{ChildBuild, ChildBuilder};
use bevy::prelude::*;
use crate::state::GameState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), create_ui);
    }
}

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/DMSans-Black.ttf");

    commands
        .spawn((Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|parent| create_top_navigation(parent, "Hunter", font))
        .with_children(create_inner_container)
        .with_children(create_bottom_bar);
}

fn create_top_navigation(commands: &mut ChildBuilder, text: &str, font: Handle<Font>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Relative,
                width: Val::Percent(100.0),
                height: Val::Percent(5.9),
                ..default()
            },
            BackgroundColor(Color::Srgba(GRAY_50)),
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
fn create_inner_container(commands: &mut ChildBuilder) {
    // no color, it only for spacing
    commands
        .spawn((Node {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(92.0),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },))
        .with_children(create_left_navigation)
        .with_children(create_right_navigation);
}
fn create_left_navigation(commands: &mut ChildBuilder) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}
fn create_right_navigation(commands: &mut ChildBuilder) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}
fn create_bottom_bar(commands: &mut ChildBuilder) {
    commands.spawn((
        Node {
            height: Val::Percent(14.3),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
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
