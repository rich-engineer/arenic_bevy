use crate::arenas::ArenaNameEnum;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalState {
    // pub selected_character: Option<Entity>,
    pub current_arena: u8,
    pub active_menu: bool,
    // TODO Remove these obsolete values
    pub record_mode: bool,
    pub selected_characters_cached_transform: Transform,
}
impl GlobalState {
    /// Checks if `current_arena` is not present in the given array.
    pub fn is_current_arena_not_in(&self, arr: &[u8; 3]) -> bool {
        !arr.contains(&self.current_arena)
    }
    pub fn is_in_current_arena(&self, arr: &[u8; 3]) -> bool {
        arr.contains(&self.current_arena)
    }
}
impl Default for GlobalState {
    fn default() -> Self {
        Self {
            // selected_character: None,
            current_arena: 4,
            active_menu: false,
            record_mode: false,
            selected_characters_cached_transform: Transform::IDENTITY,
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalState>().init_state::<GameState>();
        // .add_systems(Update, log_state_changes);
        // Add our system to run whenever GameState changes
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    Title,

    Intro,
    Menu,
    Arena(ArenaNameEnum),
    Rotate,
    Roster,
    Gacha,
    AuctionHouse,
    CraftWorkshop,
}

fn log_state_changes(state: Res<State<GameState>>) {
    // This will run on startup and whenever the state changes
    info!("Current Game State: {:?}", *state);
}
