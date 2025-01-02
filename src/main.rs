use bevy::prelude::*;
mod abilities;
mod arenas;
mod cameras;
mod characters;
mod interactions;
mod state;
mod tiles;
mod title;
mod global_chat;

use crate::state::GameState;
use abilities::{AbilitiesPlugin, AbilitySpawner, CastTypeEnum, TargetTypeEnum};
use cameras::CamerasPlugin;
use characters::{CharacterClassEnum, CharacterSpawner, CharacterTypeEnum, CharactersPlugin};
use interactions::InteractionsPlugin;
use state::{SelectedCharacter, StatePlugin};
use tiles::TilesPlugin;
use title::TitlePlugin;
use global_chat::GlobalChatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(StatePlugin)
        .add_plugins(CamerasPlugin)
        .add_plugins(TitlePlugin)
        .add_plugins(AbilitiesPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(InteractionsPlugin)
        .add_plugins(GlobalChatPlugin)
        .add_systems(Startup, start_game)

        .add_systems(
            Update,
            spawn_selected_character.run_if(in_state(GameState::Start)),
        )
        .run();
}

fn start_game(mut commands: Commands, mut selected_character: ResMut<SelectedCharacter>) {
    let ability1 = AbilitySpawner::spawn_ability(
        &mut commands,
        "Split Shot",
        "Next auto shot will fork",
        5.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    let ability2 = AbilitySpawner::spawn_ability(
        &mut commands,
        "Auto Shot",
        "Automatically fires shots in the forward direction",
        1.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    let ability3 = AbilitySpawner::spawn_ability(
        &mut commands,
        "Trap",
        "Places a trap on the grid that deals damage when an enemy steps on it.",
        1.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );

    let ability4 = AbilitySpawner::spawn_ability(
        &mut commands,
        "Snipe",
        "Fires any distance always at the boss",
        4.0,
        TargetTypeEnum::BossTarget,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    // help me set this character to a new resource below
    let guild_master = CharacterSpawner::spawn_character(
        &mut commands,
        "Dean",
        CharacterTypeEnum::Hero,
        CharacterClassEnum::Hunter,
        vec![ability1, ability2, ability3, ability4],
    );

    selected_character.0 = Some(guild_master);
}

fn spawn_selected_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected: Res<SelectedCharacter>,
) {
    if selected.0.is_some() {
        commands.spawn((
            Sprite {
                image: asset_server.load("UI/player_selected.png"),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    }
}
