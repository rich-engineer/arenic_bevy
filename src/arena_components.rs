use bevy::math::Vec2;
use crate::shared_traits::{ArenaTraits};
use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct Labyrinth;
#[derive(Component)]
pub struct Sanctum;

#[derive(Component)]
pub struct Pawnshop;

#[derive(Component)]
pub struct Bastion;

#[derive(Component)]
pub struct Mountain;
#[derive(Component)]
pub struct Crucible;
#[derive(Component)]
pub struct Casino;
#[derive(Component)]
pub struct Gala;
#[derive(Component)]
pub struct GuildHouse;
#[derive(Component)]
pub struct Menu;

// Implement the ComponentDisplay trait for each component
impl ArenaTraits for Labyrinth {
    fn to_display_string(&self) -> String {
        "Labyrinth".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(-1.0, 1.0)
    }
    fn grid_index(&self) -> u8 {
        0
    }
    fn debug_tile(&self) -> &str {
        "UI/hunter_tile.png"
    }
}

impl ArenaTraits for GuildHouse {
    fn to_display_string(&self) -> String {
        "Guild House".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(0.0, 1.0)
    }
    fn grid_index(&self) -> u8 {
        1
    }
    fn debug_tile(&self) -> &str {
        "UI/guild_tile.png"
    }
}

impl ArenaTraits for Sanctum {
    fn to_display_string(&self) -> String {
        "Sanctum".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(1.0, 1.0)
    }
    fn grid_index(&self) -> u8 {
        2
    }
    fn debug_tile(&self) -> &str {
        "UI/cardinal_tile.png"
    }
}

impl ArenaTraits for Mountain {
    fn to_display_string(&self) -> String {
        "Mountain".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(-1.0, 0.0)
    }
    fn grid_index(&self) -> u8 {
        3
    }
    fn debug_tile(&self) -> &str {
        "UI/forager_tile.png"
    }
}

impl ArenaTraits for Bastion {
    fn to_display_string(&self) -> String {
        "Bastion".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    fn grid_index(&self) -> u8 {
        4
    }
    fn debug_tile(&self) -> &str {
        "UI/warrior_tile.png"
    }
}

impl ArenaTraits for Pawnshop {
    fn to_display_string(&self) -> String {
        "Pawnshop".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(1.0, 0.0)
    }
    fn grid_index(&self) -> u8 {
        5
    }
    fn debug_tile(&self) -> &str {
        "UI/thief_tile.png"
    }
}

impl ArenaTraits for Crucible {
    fn to_display_string(&self) -> String {
        "Crucible".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(-1.0, -1.0)
    }
    fn grid_index(&self) -> u8 {
        6
    }
    fn debug_tile(&self) -> &str {
        "UI/alchemist_tile.png"
    }
}

impl ArenaTraits for Casino {
    fn to_display_string(&self) -> String {
        "Casino".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(0.0, -1.0)
    }
    fn grid_index(&self) -> u8 {
        7
    }
    fn debug_tile(&self) -> &str {
        "UI/merchant_tile.png"
    }
}

impl ArenaTraits for Gala {
    fn to_display_string(&self) -> String {
        "Gala".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(1.0, -1.0)
    }
    fn grid_index(&self) -> u8 {
        8
    }
    fn debug_tile(&self) -> &str {
        "UI/bard_tile.png"
    }
}


impl ArenaTraits for Menu {
    fn to_display_string(&self) -> String {
        "---".to_string()
    }
    fn offset_matrix(&self) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    fn grid_index(&self) -> u8 {
        8
    }
    fn debug_tile(&self) -> &str {
        "UI/default_tile.png"
    }
}

#[derive(Component, Debug)]
pub struct Arena {
    pub id: u8,
}
#[derive(Component)]
pub struct ArenasParent;
#[derive(Component)]
pub struct ArenaBossText;

#[derive(Component)]
pub struct SelectedHero(pub Option<Entity>);

#[derive(Component)]
pub struct ArenaName(pub String);
#[derive(Component)]
pub struct InactiveArena;

#[derive(Component)]
pub struct ActiveArena;

#[derive(Component)]
pub struct MenuOpen;