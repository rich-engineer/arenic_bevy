mod arenas;
mod cameras;
mod characters;
mod constants;
mod hud;
mod intro;
mod scenes;
mod state;
mod title;

use crate::hud::HUDPlugin;
use crate::intro::{intro_spawn_guildmaster_and_recruit, IntroPlugin};
use arenas::setup_arenas;
use bevy::prelude::*;
use cameras::{setup_camera, CamerasPlugin};
use constants::RESOLUTION;
use intro::set_camera_intro_arena;
use state::StatePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "A R E N I C".to_string(),
                        resolution: RESOLUTION.into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(StatePlugin)
        .add_plugins(CamerasPlugin)
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_arenas.after(setup_camera),
                set_camera_intro_arena.after(setup_arenas),
                intro_spawn_guildmaster_and_recruit.after(setup_arenas),
            ),
        )
        .add_plugins(IntroPlugin)
        .add_plugins(HUDPlugin)
        .run();
}
