use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalState {
    pub active_menu: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self { active_menu: true }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalState>();
    }
}
