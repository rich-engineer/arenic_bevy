use crate::shared_traits::EnumDisplay;

pub enum ArenaNameEnum {
    Labyrinth,
    Sanctum,
    Pawnshop,
    Bastion,
    Mountain,
    Crucible,
    Casino,
    Gala,
    GuildHouse,
    Menu
}

impl EnumDisplay for ArenaNameEnum {
    fn to_display_string(&self) -> String {
        match self {
            // Hunter abilities
            ArenaNameEnum::Labyrinth => "Labyrinth",
            ArenaNameEnum::Sanctum => "Sanctum",
            ArenaNameEnum::Pawnshop => "Pawnshop",
            ArenaNameEnum::Bastion => "Bastion",
            ArenaNameEnum::Mountain => "Mountain",
            ArenaNameEnum::Crucible => "Crucible",
            ArenaNameEnum::Casino => "Casino",
            ArenaNameEnum::Gala => "Gala",
            ArenaNameEnum::GuildHouse => "Guild House",
            ArenaNameEnum::Menu => "---",
        }.to_string()
    }
}

// pub struct ArenaPlugin;
//
// impl Plugin for ArenaPlugin {
//     fn build(&self, app: &mut App) {
        // app.insert_resource(ArenaPool(vec![
        //     Arena {
        //         name: ArenaEnum::Labyrinth,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Hunter],
        //     },
        //     Arena {
        //         name: ArenaEnum::Bastion,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Warrior],
        //     },
        //     Arena {
        //         name: ArenaEnum::Sanctum,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Cardinal],
        //     },
        //     Arena {
        //         name: ArenaEnum::Mountain,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Forager],
        //     },
        //     Arena {
        //         name: ArenaEnum::Pawnshop,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Thief],
        //     },
        //     Arena {
        //         name: ArenaEnum::Crucible,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Alchemist],
        //     },
        //     Arena {
        //         name: ArenaEnum::Casino,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Merchant],
        //     },
        //     Arena {
        //         name: ArenaEnum::Gala,
        //         battle_duration_seconds: 120,
        //         owner_classes: vec![CharacterClassEnum::Bard],
        //     },
        //     Arena {
        //         name: ArenaEnum::GuildHouse,
        //         battle_duration_seconds: 0,
        //         owner_classes: vec![CharacterClassEnum::GuildMaster],
        //     },
        // ]));
//     }
// }
