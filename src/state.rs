use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct SelectedCharacter(pub Option<Entity>);

impl Default for SelectedCharacter {
    fn default() -> Self {
        SelectedCharacter(None)
    }
}


pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCharacter>();
    }
}
