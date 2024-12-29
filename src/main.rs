mod arenas;
mod local_storage;
mod abilities;
mod interactions;
mod names;
mod characters;

use bevy::prelude::*;
use local_storage::{LocalStorage, LocalStoragePlugin};
use crate::abilities::AbilityPoolExt;
use crate::characters::{CharacterAbilities, CharacterClass, CharacterClassEnum, CharacterType, CharacterTypeEnum};
use crate::names::Name;

fn example_system(storage: Res<LocalStorage>) {
    storage.save_string("my_key2", "Hello from Cross-Platform Storage!");
    if let Some(loaded) = storage.load_string("my_key2") {
        info!("Loaded: {}", loaded);
    } else {
        info!("No value found for 'my_key2'.");
    }
}

fn character_startup_system(mut commands: Commands, ability_pool: Res<abilities::AbilityPool>) {
    let hunter_entity =  commands.spawn((
        Name("Dean".to_string()),
        CharacterType(CharacterTypeEnum::Hero),
        CharacterClass(CharacterClassEnum::Hunter),
        CharacterAbilities(ability_pool.sample_random(CharacterClassEnum::Hunter)),
    )).id();

    info!("Hunter has been spawned successfully with Entity ID: {:?}",  hunter_entity);

}

fn log_hunter_abilities_system(
    query: Query<(&Name, &CharacterClass, &CharacterAbilities), With<CharacterType>>,
) {
    for (name, class, abilities) in &query {
        if let CharacterClassEnum::Hunter = class.0 {
            info!("{} the Hunter has these abilities:", name.0);
            for ability in &abilities.0 {
                info!(" - {}", ability.name);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(arenas::ScreenDimensions::default())
        .add_plugins(abilities::AbilitiesPlugin)
        .add_plugins(LocalStoragePlugin)
        .add_systems(
            Startup,
            (
                example_system,
                character_startup_system,
                arenas::resize_system,
                arenas::setup_scene,
                arenas::spawn_arenas,
            ),

        )
        .add_systems(Update, log_hunter_abilities_system)
        .run();
}
