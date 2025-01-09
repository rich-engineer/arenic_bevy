use crate::arena_components::Arena;
use crate::state::{GameState, GlobalState};
use bevy::prelude::*;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(
            OnEnter(GameState::Intro),
            intro_spawn_guildmaster_and_recruit.after(set_camera_pos),
        );
    }
}

fn set_camera_pos(mut state: ResMut<GlobalState>) {
    state.current_arena = 4;
}

fn intro_spawn_guildmaster_and_recruit(
    query: Query<(Entity, &Arena)>,
    asset_server: Res<AssetServer>,
    state: Res<GlobalState>,
) {
    // let texture_selected = asset_server.load("UI/selected.png");
}
