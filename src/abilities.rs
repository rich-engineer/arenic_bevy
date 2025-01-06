use crate::characters::CharacterClassEnum;
use crate::shared_traits::EnumDisplay;
use bevy::prelude::*;

#[allow(dead_code)]
pub enum AbilityNameEnum {
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

impl EnumDisplay for AbilityNameEnum {
    fn to_display_string(&self) -> String {
        match self {
            // Hunter abilities
            AbilityNameEnum::SplitShot => "Split Shot",
            AbilityNameEnum::AutoShot => "Auto Shot",
            AbilityNameEnum::Trap => "Trap",
            AbilityNameEnum::Snipe => "Snipe",

            // Warrior abilities
            AbilityNameEnum::Block => "Block",
            AbilityNameEnum::Bash => "Bash",
            AbilityNameEnum::Taunt => "Taunt",
            AbilityNameEnum::Bulwark => "Bulwark",

            // Alchemist abilities
            AbilityNameEnum::Ironskin => "Iron skin",
            AbilityNameEnum::Acid => "Acid",
            AbilityNameEnum::Transmute => "Transmute",
            AbilityNameEnum::Siphon => "Siphon",

            // Gatherer abilities
            AbilityNameEnum::Border => "Border",
            AbilityNameEnum::Bolder => "Bolder",
            AbilityNameEnum::Dig => "Dig",
            AbilityNameEnum::Mushroom => "Mushroom",

            // Cardinal
            AbilityNameEnum::Barrier => "Barrier",
            AbilityNameEnum::Beam => "Beam",
            AbilityNameEnum::Heal => "Heal",
            AbilityNameEnum::Resurrect => "Resurrect",
            // Merchant
            AbilityNameEnum::Dice => "Dice",
            AbilityNameEnum::CoinToss => "CoinToss",
            AbilityNameEnum::Fortune => "Fortune",
            AbilityNameEnum::Interest => "Interest",

            // Thief
            AbilityNameEnum::SmokeScreen => "Smoke Screen",
            AbilityNameEnum::Backstab => "Backstab",
            AbilityNameEnum::Pickpocket => "Pickpocket",
            AbilityNameEnum::ShadowStep => "Shadow Step",

            // Bard
            AbilityNameEnum::Cleanse => "Cleanse",
            AbilityNameEnum::Dance => "Dance",
            AbilityNameEnum::Helix => "Helix",
            AbilityNameEnum::Mimic => "Mimic",
        }
        .to_string()
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
