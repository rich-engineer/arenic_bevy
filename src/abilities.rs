use bevy::prelude::*;
use crate::characters::{CharacterClass, CharacterClassEnum};
use crate::interactions::Interaction;

#[derive(Component, Clone)]
pub struct Ability {
    pub(crate) name: String,
    cooldown: u8,
    interaction: Interaction,
    owner_classes: Vec<CharacterClassEnum>,
}

#[derive(Resource)]
pub(crate) struct AbilityPool(Vec<Ability>);

pub trait AbilityPoolExt {
    /// Samples `n` random abilities from the pool if available.
    /// Clones them so each consumer can own them.
    fn sample_random(&self, class: CharacterClassEnum) -> Vec<Ability>;
}

impl AbilityPoolExt for AbilityPool {
    fn sample_random(&self, class: CharacterClassEnum) -> Vec<Ability> {
        // later will make that take shuffle the Ability order
        self.0.iter().filter(|ability| ability.owner_classes.contains(&class)).take(4).cloned().collect()
    }
}

/// Define the AbilitiesPlugin
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        // Insert the AbilityPool resource with 10 example abilities
        app.insert_resource(AbilityPool(vec![
            Ability {
                name: "Split Shot".to_string(),
                cooldown: 5,
                interaction: Interaction::Tap,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Explosive Arrow".to_string(),
                cooldown: 10,
                interaction: Interaction::Hold,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Rain of Arrows".to_string(),
                cooldown: 8,
                interaction: Interaction::HoldRelease,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Piercing Shot".to_string(),
                cooldown: 6,
                interaction: Interaction::Tap,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Shadow Step".to_string(),
                cooldown: 4,
                interaction: Interaction::Hold,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Poison Arrow".to_string(),
                cooldown: 7,
                interaction: Interaction::Tap,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Trap Mastery".to_string(),
                cooldown: 12,
                interaction: Interaction::HoldRelease,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Flame Arrow".to_string(),
                cooldown: 9,
                interaction: Interaction::Hold,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Quick Draw".to_string(),
                cooldown: 3,
                interaction: Interaction::Tap,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Eagle Eye".to_string(),
                cooldown: 5,
                interaction: Interaction::HoldRelease,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
        ]));
    }
}