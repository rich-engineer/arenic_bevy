use crate::state::GameState;
use bevy::color::palettes::tailwind::GRAY_50;
use bevy::prelude::Display::Flex;
use bevy::prelude::*;

pub struct GlobalChatPlugin;

impl Plugin for GlobalChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GuildHouse), spawn_global_chat_system);
    }
}

fn spawn_global_chat_system(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::Srgba(GRAY_50)),
    ));
}
