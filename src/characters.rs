
use crate::abilities::AbilitySpawner;
use crate::interactions::KeyBindingsForAbility;
use bevy::prelude::*;
use crate::shared_traits::EnumDisplay;

pub enum CharacterTypeEnum {
    Hero,
    Boss,
    Mob,
}


#[derive(PartialEq, Eq)]
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
    Menu
}

impl EnumDisplay for CharacterClassEnum {
    fn to_display_string(&self) -> String {
        match self {
            CharacterClassEnum::Alchemist => "Alchemist",
            CharacterClassEnum::Bard => "Bard",
            CharacterClassEnum::Cardinal => "Cardinal",
            CharacterClassEnum::Forager => "Forager",
            CharacterClassEnum::Merchant => "Merchant",
            CharacterClassEnum::Hunter => "Hunter",
            CharacterClassEnum::Thief => "Thief",
            CharacterClassEnum::Warrior => "Warrior",
            CharacterClassEnum::GuildMaster => "Guild Master",
            CharacterClassEnum::Menu => "---",
        }.to_string()
    }
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
    ) -> Entity {
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
        let spawned_character = commands
            .spawn((
                CharacterName(character_name.to_string()),
                CharacterType(character_type),
                CharacterClass(character_class),
                CharacterAbilities {
                    abilities: class_abilities,
                },
                KeyBindingsForAbility {
                    bindings: key_bindings,
                },
            ))
            .id();

        spawned_character
    }
}

// pub fn intro_gm_hunter() {
    // let guild_chat = vec!["Approaches the lone figure near the Guild House entrance"];
    // let guild_master = vec![
    //     "Ah, there you are. I heard rumor of a Hunter seeking to join our cause. You must be the one they call ‘the Wanderer of the Labyrinth.’ Correct?",
    //     "Welcome, Anden. We’ve only just begun rebuilding. Our aim is to unite all who’d stand against the rising entropy in these arenas. Some say it’s an impossible task, but we’ll find a way. Word is, your marksmanship is second to none.",
    //     "Then consider yourself our first official recruit, Hunter. We’ll gather more allies soon enough, but for now, your bow will be invaluable. Trust me—these eight arenas won’t conquer themselves."
    // ];
    // let hunter = vec![
    //     "That’s right. Name’s Anden. The Labyrinth’s become… unruly of late. I’ve seen beasts twisted by some new chaos. Figured it was time to lend my arrows to a bigger fight—if your Guild’ll have me.",
    //     "I’ve trained in the Labyrinth’s shifting pathways all my life. If you can promise me worthy targets—and a chance to learn what’s behind this disorder—I’m in.",
    //     "Heh, I like the sound of that. Just point me to our next target, Commander."
    //
    // ];
// }
