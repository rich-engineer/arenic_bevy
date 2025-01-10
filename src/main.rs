use bevy::prelude::*;
mod abilities;
mod arena_components;
mod arenas;
mod cameras;
mod characters;
mod constants;
mod events;
mod global_chat;
mod hud;
mod interactions;
mod intro;
mod shared_traits;
mod state;
mod title;
mod character_components;

use abilities::AbilitiesPlugin;
use arenas::ArenaPlugin;
use cameras::CamerasPlugin;
use constants::RESOLUTION;
use hud::HUDPlugin;
// use intro::IntroPlugin;
use crate::intro::IntroPlugin;
use state::StatePlugin;
use title::TitlePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Arenic".to_string(),
                        resolution: RESOLUTION.into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(StatePlugin)
        .add_plugins(CamerasPlugin)
        .add_plugins(IntroPlugin)
        .add_plugins(TitlePlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(AbilitiesPlugin)
        .add_plugins(ArenaPlugin)
        .run();
}
