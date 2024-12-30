use bevy::prelude::*;
use crate::characters::CharacterClassEnum;

#[derive(Component)]
pub struct Cooldown {
    pub total: f32,
    pub remaining: f32,
}

#[derive(Component)]
pub struct TargetType(pub TargetTypeEnum);

#[derive(Component)]
pub struct CastType(pub CastTypeEnum);

#[derive(Component)]
pub struct AbilityName(pub String);

#[derive(Component)]
pub struct AbilityDescription(pub String);

#[derive(Component, PartialEq)]
pub struct OwnerClasses(pub Vec<CharacterClassEnum>);

#[derive(Resource, Default)]
pub struct AbilitySpawner;

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

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilitySpawner>()
            .add_systems(Update, update_cooldowns);
    }
}

fn update_cooldowns(time: Res<Time>, mut cooldowns: Query<&mut Cooldown>) {
    for mut cooldown in &mut cooldowns {
        if cooldown.remaining > 0.0 {
            cooldown.remaining -= time.delta_secs();
        }
    }
}

impl AbilitySpawner {
    pub fn spawn_ability(
        commands: &mut Commands,
        name: &str,
        description: &str,
        cooldown: f32,
        target_type: TargetTypeEnum,
        cast_type: CastTypeEnum,
        owner_classes: Vec<CharacterClassEnum>,
    ) -> Entity {
        commands
            .spawn((
                AbilityName(name.to_string()),
                AbilityDescription(description.to_string()),
                Cooldown {
                    total: cooldown,
                    remaining: 0.0,
                },
                TargetType(target_type),
                CastType(cast_type),
                OwnerClasses(owner_classes),
            ))
            .id()
    }
}