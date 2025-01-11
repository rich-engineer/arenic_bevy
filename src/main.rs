mod constants;
use bevy::prelude::*;
use constants::RESOLUTION;

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
        .run();
}
