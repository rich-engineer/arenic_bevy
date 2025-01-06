use bevy::prelude::*;
mod abilities;
mod arenas;
mod characters;
mod global_chat;
mod interactions;
mod state;
mod title;
mod shared_traits;
mod cameras;
mod constants;
mod hud;

use state::{StatePlugin};
use title::TitlePlugin;
use arenas::ArenaPlugin;
use cameras::CamerasPlugin;
use abilities::AbilitiesPlugin;
use characters::CharactersPlugin;
use hud::HUDPlugin;

const RESOLUTION: (f32, f32) = (1280.0, 720.0);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arenic".to_string(),
                    resolution: RESOLUTION.into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_plugins(StatePlugin)
        .add_plugins(CamerasPlugin)
        .add_plugins(TitlePlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(AbilitiesPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(CharactersPlugin)
        .run();
}

