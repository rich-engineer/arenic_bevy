use bevy::app::{App, Plugin};
use bevy::prelude::{Component, Resource};
use crate::arenas::ArenaEnum;

pub enum SceneEnum {
    Title,
    Menu,
    Arena(ArenaEnum),
    Rotate,
    Roster,
    Gacha,
    AuctionHouse,
    CraftWorkshop,
}
#[derive(Component)]
pub struct Scene {
    pub name: SceneEnum,
}

#[derive(Resource)]
pub(crate) struct ScenePool(Vec<Scene>);


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScenePool(vec![
            Scene {
                name: SceneEnum::Title,
            },
            Scene {
                name: SceneEnum::Menu,
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Bastion),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Casino),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Crucible),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Gala),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::GuildHouse),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Labyrinth),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Mountain),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Pawnshop),
            },
            Scene {
                name: SceneEnum::Arena(ArenaEnum::Sanctum),
            },
            Scene {
                name: SceneEnum::Gacha
            },
            Scene {
                name: SceneEnum::AuctionHouse
            },
            Scene {
                name: SceneEnum::CraftWorkshop
            }
        ]));
    }
}