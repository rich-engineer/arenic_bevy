
use crate::characters::CharacterClassEnum;
use bevy::prelude::*;


pub trait DisplayAbility {
    fn to_display_string(&self) -> String;
}

pub enum Ability {
    // 1 Hunter abilities
    SplitShot,
    AutoShot,
    Trap,
    Snipe,

    // 2 Warrior abilities
    Block,
    Bash,
    Taunt,
    Bulwark,

    // 3 Alchemist abilities
    Ironskin,
    Acid,
    Transmute,
    Siphon,

    // 4 Gatherer abilities
    Border,
    Bolder,
    Dig,
    Mushroom,

    // 5 Cardinal abilities
    Barrier,
    Beam,
    Heal,
    Resurrect,

    // 6 Merchant abilities
    Dice,
    CoinToss,
    Fortune,
    Interest,

    // 7 Thief abilities
    SmokeScreen,
    Backstab,
    Pickpocket,
    ShadowStep,

    // 8 Bard abilities
    Cleanse,
    Dance,
    Helix,
    Mimic,
}

impl DisplayAbility for Ability {
    fn to_display_string(&self) -> String {
        match self {
            // Hunter abilities
            Ability::SplitShot => "Split Shot",
            Ability::AutoShot => "Auto Shot",
            Ability::Trap => "Trap",
            Ability::Snipe => "Snipe",

            // Warrior abilities
            Ability::Block => "Block",
            Ability::Bash => "Bash",
            Ability::Taunt => "Taunt",
            Ability::Bulwark => "Bulwark",

            // Alchemist abilities
            Ability::Ironskin => "Iron skin",
            Ability::Acid => "Acid",
            Ability::Transmute => "Transmute",
            Ability::Siphon => "Siphon",

            // Gatherer abilities
            Ability::Border => "Border",
            Ability::Bolder => "Bolder",
            Ability::Dig => "Dig",
            Ability::Mushroom => "Mushroom",

            // Cardinal
            Ability::Barrier => "Barrier",
            Ability::Beam => "Beam",
            Ability::Heal => "Heal",
            Ability::Resurrect => "Resurrect",
            // Merchant
            Ability::Dice => "Dice",
            Ability::CoinToss => "CoinToss",
            Ability::Fortune => "Fortune",
            Ability::Interest => "Interest",

            // Thief
            Ability::SmokeScreen => "Smoke Screen",
            Ability::Backstab => "Backstab",
            Ability::Pickpocket => "Pickpocket",
            Ability::ShadowStep => "Shadow Step",

            // Bard
            Ability::Cleanse => "Cleanse",
            Ability::Dance => "Dance",
            Ability::Helix => "Helix",
            Ability::Mimic => "Mimic",

        }.to_string()
    }
}

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
        app.init_resource::<AbilitySpawner>()
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
        name: String,
        description: &str,
        cooldown: f32,
        target_type: TargetTypeEnum,
        cast_type: CastTypeEnum,
        owner_classes: Vec<CharacterClassEnum>,
    ) -> Entity {
        commands
            .spawn((
                AbilityName(name),
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
