use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalState {
    // pub selected_character: Option<Entity>,
    pub current_arena: u8,
    pub active_menu: bool,
}


impl Default for GlobalState {
    fn default() -> Self {
        Self {
            // selected_character: None,
            current_arena: 0,
            active_menu: false
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalState>()
            .init_state::<GameState>();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[allow(dead_code)]
pub enum GameState {

    Title,
    Start,

    GuildHouse,
    #[default]
    HighlightRect,
}
