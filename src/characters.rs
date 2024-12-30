use crate::abilities::AbilitySpawner;
use crate::interactions::KeyBindingsForAbility;
use bevy::prelude::*;

pub enum CharacterTypeEnum {
    Hero,
    Boss,
    Mob,
}

#[derive(Clone, PartialEq)]
pub enum CharacterClassEnum {
    Alchemist,
    Bard,
    Cardinal,
    Forager,
    Merchant,
    Hunter,
    Thief,
    Warrior,
    GuildMaster,
}

#[derive(Component)]
pub struct CharacterName(pub String);

#[derive(Component)]
pub struct CharacterType(pub CharacterTypeEnum);

#[derive(Component)]
pub struct CharacterClass(pub CharacterClassEnum);

#[derive(Component)]
pub struct CharacterAbilities {
    pub abilities: Vec<Entity>,
}
#[derive(Resource, Default)]
pub struct CharacterSpawner;

impl Default for CharacterAbilities {
    fn default() -> Self {
        Self {
            abilities: Vec::new(),
        }
    }
}

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilitySpawner>();
    }
}

impl CharacterSpawner {
    pub fn spawn_character(
        commands: &mut Commands,
        character_name: &str,
        character_type: CharacterTypeEnum,
        character_class: CharacterClassEnum,
        class_abilities: Vec<Entity>,
    ) {
        let key_bindings = class_abilities
            .iter()
            .enumerate()
            .map(|(index, &entity)| {
                let key = match index {
                    0 => KeyCode::Digit1,
                    1 => KeyCode::Digit2,
                    2 => KeyCode::Digit3,
                    3 => KeyCode::Digit4,
                    _ => KeyCode::KeyR,
                };
                (entity, key)
            })
            .collect();
        commands.spawn((
            CharacterName(character_name.to_string()),
            CharacterType(character_type),
            CharacterClass(character_class),
            CharacterAbilities {
                abilities: class_abilities,
            },
            KeyBindingsForAbility {
                bindings: key_bindings,
            },
        ));
    }
}
