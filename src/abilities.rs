use bevy::prelude::*;
use crate::characters::{CharacterClass, CharacterClassEnum};
use crate::interactions::{InputBinding, Interaction, InteractionMode, KeyBinding};
use crate::metadata::Description;


#[derive(Clone)]
pub enum TargetTypeEnum {
    SingleTarget,
    MultiTarget,
    AreaOfEffect { radius: f32 },
    SelfTarget,
    BossTarget,
    CurrentGridTarget,
    Directional,
    Global,
}

#[derive(Clone)]
pub enum CastTypeEnum {
    InstantCast,
    CastTime,
}

#[derive(Component, Clone)]
pub struct Ability {
    pub(crate) name: String,
    description: String,
    cooldown: u8,
    target_type: TargetTypeEnum,
    cast_type: CastTypeEnum,
    interactions: Vec<Interaction>,
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
        app.insert_resource(AbilityPool(vec![
            Ability {
                name: "Split Shot".to_string(),
                description: "Next auto shot will fork".to_string(),
                cooldown: 5,
                cast_type: CastTypeEnum::InstantCast,
                interactions: vec![
                    Interaction{ binding: InputBinding::Keyboard(KeyBinding::Single(KeyCode::Digit1)), mode: InteractionMode::Tap }
                ],
                target_type: TargetTypeEnum::Directional,
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Auto Shot".to_string(),
                description: "Places a trap on the grid that deals damage when an enemy steps on it.".to_string(),
                cooldown: 1,
                target_type: TargetTypeEnum::Directional,
                cast_type: CastTypeEnum::InstantCast,
                interactions: vec![
                    Interaction{ binding: InputBinding::Keyboard(KeyBinding::Single(KeyCode::Digit2)), mode: InteractionMode::Hold }
                ],
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Trap".to_string(),
                description: "Lays a trap down on current grid space. If enemy touch it small AOE explosion 2x2.".to_string(),
                cooldown: 3,
                target_type: TargetTypeEnum::CurrentGridTarget,
                cast_type: CastTypeEnum::CastTime,
                interactions:
                vec![
                    Interaction{ binding: InputBinding::Keyboard(KeyBinding::Single(KeyCode::Digit3)), mode: InteractionMode::HoldRelease }
                ],
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
            Ability {
                name: "Sniper".to_string(),
                description: "Fires any distance always at the boss".to_string(),
                cooldown: 9,
                target_type: TargetTypeEnum::BossTarget,
                cast_type: CastTypeEnum::InstantCast,
                interactions: vec![
                    Interaction{ binding: InputBinding::Keyboard(KeyBinding::Single(KeyCode::Digit4)), mode: InteractionMode::Tap }
                ],
                owner_classes: vec![CharacterClassEnum::Hunter]
            },
        ]));
    }
}