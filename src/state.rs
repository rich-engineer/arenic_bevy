use crate::abilities;
use crate::characters::{CharacterAbilities, CharacterName};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedCharacter(pub Option<Entity>);

impl Default for SelectedCharacter {
    fn default() -> Self {
        SelectedCharacter(None)
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCharacter>()
            .init_state::<GameState>();
        // .add_systems(Update, log_selected_character_abilities);
    }
}

fn log_selected_character_abilities(
    selected: Res<SelectedCharacter>,
    characters: Query<(&CharacterName, &CharacterAbilities)>,
    abilities: Query<(&abilities::AbilityName, &abilities::AbilityDescription)>,
) {
    if let Some(character_entity) = selected.0 {
        if let Ok((character_name, character_abilities)) = characters.get(character_entity) {
            println!("=== {}'s Abilities ===", character_name.0);

            for &ability in &character_abilities.abilities {
                if let Ok((name, desc)) = abilities.get(ability) {
                    println!(" - {}: {}", name.0, desc.0);
                } else {
                    println!(" - Missing components for ability {:?}", ability);
                }
            }

            println!("======================================");
        } else {
            println!("Selected character has no abilities.");
        }
    } else {
        println!("No character selected.");
    }
}

// Define game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Title,
    Start,
    GuildHouse,
}
