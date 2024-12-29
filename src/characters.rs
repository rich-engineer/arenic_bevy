use bevy::prelude::Component;
use crate::abilities::Ability;

// Types
pub enum CharacterTypeEnum {
    Hero,
    Boss,
    Mob,
}

pub enum CharacterClassEnum {
    Warrior,
    Hunter,
}


#[derive(Component)]
pub struct CharacterType(pub CharacterTypeEnum);

#[derive(Component)]
pub struct CharacterClass(pub CharacterClassEnum);

#[derive(Component)]
pub struct CharacterAbilities(pub Vec<Ability>);


