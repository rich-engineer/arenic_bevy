use bevy::prelude::{Component, Entity};
use crate::shared_traits::ComponentDisplay;

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
impl ComponentDisplay for Labyrinth {
    fn to_display_string(&self) -> String {
        "Labyrinth".to_string()
    }
}

impl ComponentDisplay for Sanctum {
    fn to_display_string(&self) -> String {
        "Sanctum".to_string()
    }
}

impl ComponentDisplay for Pawnshop {
    fn to_display_string(&self) -> String {
        "Pawnshop".to_string()
    }
}

impl ComponentDisplay for Bastion {
    fn to_display_string(&self) -> String {
        "Bastion".to_string()
    }
}

impl ComponentDisplay for Mountain {
    fn to_display_string(&self) -> String {
        "Mountain".to_string()
    }
}

impl ComponentDisplay for Crucible {
    fn to_display_string(&self) -> String {
        "Crucible".to_string()
    }
}

impl ComponentDisplay for Casino {
    fn to_display_string(&self) -> String {
        "Casino".to_string()
    }
}

impl ComponentDisplay for Gala {
    fn to_display_string(&self) -> String {
        "Gala".to_string()
    }
}

impl ComponentDisplay for GuildHouse {
    fn to_display_string(&self) -> String {
        "Guild House".to_string()
    }
}

impl ComponentDisplay for Menu {
    fn to_display_string(&self) -> String {
        "---".to_string()
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