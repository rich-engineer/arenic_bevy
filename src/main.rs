use bevy::prelude::*;
mod abilities;
mod arenas;
mod characters;
mod interactions;
mod state;
mod cameras;
mod tiles;

use abilities::{AbilitiesPlugin, AbilitySpawner, CastTypeEnum, TargetTypeEnum};
use characters::{CharacterClassEnum, CharacterSpawner, CharacterTypeEnum, CharactersPlugin};
use interactions::InteractionsPlugin;
use state::{SelectedCharacter, StatePlugin};
use cameras::CamerasPlugin;
use tiles::TilesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CamerasPlugin)
        .add_plugins(AbilitiesPlugin)
        .add_plugins(TilesPlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(InteractionsPlugin)
        .add_plugins(StatePlugin)
        .add_systems(Startup, start_game)
        .add_systems(Update, spawn_selected_character)
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