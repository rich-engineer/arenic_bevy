use crate::title::TitlePlugin;
use bevy::app::{App, Plugin};

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TitlePlugin);
    }
}
