use bevy::prelude::*;


pub struct HighlightRectPlugin;

impl Plugin for HighlightRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

pub fn setup_scene() {
    info!("Setting up HighlightRectScene");
}