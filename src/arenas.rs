use crate::characters::CharacterClassEnum;
use bevy::app::App;
use bevy::prelude::{Component, Plugin, Resource};

#[derive(Hash, PartialEq, Eq)]
pub enum ArenaEnum {
    Labyrinth,
    Sanctum,
    Pawnshop,
    Bastion,
    Mountain,
    Crucible,
    Casino,
    Gala,
    GuildHouse,
}

#[derive(Component)]
pub struct Arena {
    pub name: ArenaEnum,
    battle_duration_seconds: u8,
    owner_classes: Vec<CharacterClassEnum>,
}

#[derive(Resource)]
pub(crate) struct ArenaPool(Vec<Arena>);

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ArenaPool(vec![
            Arena {
                name: ArenaEnum::Labyrinth,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Hunter],
            },
            Arena {
                name: ArenaEnum::Bastion,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Warrior],
            },
            Arena {
                name: ArenaEnum::Sanctum,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Cardinal],
            },
            Arena {
                name: ArenaEnum::Mountain,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Forager],
            },
            Arena {
                name: ArenaEnum::Pawnshop,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Thief],
            },
            Arena {
                name: ArenaEnum::Crucible,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Alchemist],
            },
            Arena {
                name: ArenaEnum::Casino,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Merchant],
            },
            Arena {
                name: ArenaEnum::Gala,
                battle_duration_seconds: 120,
                owner_classes: vec![CharacterClassEnum::Bard],
            },
            Arena {
                name: ArenaEnum::GuildHouse,
                battle_duration_seconds: 0,
                owner_classes: vec![CharacterClassEnum::GuildMaster],
            },
        ]));
    }
}
