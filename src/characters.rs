use crate::shared_traits::EnumDisplay;
use bevy::prelude::*;



#[derive(PartialEq, Eq)]
#[allow(dead_code)]
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
    Menu,
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
        }
        .to_string()
    }
}

#[derive(Component)]
pub struct CharacterName(pub String);

#[derive(Component)]
pub struct CharacterClass(pub CharacterClassEnum);
#[derive(Component, Clone, PartialEq, Eq)]
pub struct ParentArena(pub u8);


#[derive(Component)]
pub struct CharacterAbilities {
    pub abilities: Vec<Entity>,
}
#[derive(Resource, Default)]
pub struct CharacterSpawner;

#[derive(Component)]
pub struct CachedState {
    pub previous_transform: Transform,
    pub previous_arena: ParentArena,
    pub record_start_time: Option<f64>,
    pub playback_start_time: Option<f64>,
    pub playback_current_index: usize,
}

impl Default for CharacterAbilities {
    fn default() -> Self {
        Self {
            abilities: Vec::new(),
        }
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
