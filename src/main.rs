use bevy::prelude::*;
mod abilities;
mod arenas;
mod characters;
mod interactions;

use crate::characters::{CharacterSpawner, CharacterTypeEnum};
use abilities::{AbilitiesPlugin, AbilitySpawner, CastTypeEnum, TargetTypeEnum};
use characters::{CharacterClassEnum, CharactersPlugin};
use interactions::InteractionsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AbilitiesPlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(InteractionsPlugin)
        .add_systems(Startup, start_game)
        .run();
}

fn start_game(mut commands: Commands) {
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
    CharacterSpawner::spawn_character(
        &mut commands,
        "Dean",
        CharacterTypeEnum::Hero,
        CharacterClassEnum::Hunter,
        vec![ability1, ability2, ability3, ability4],
    )
}
