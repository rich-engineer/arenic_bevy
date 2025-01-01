use bevy::prelude::*;
use bevy::render::camera::ScalingMode;


pub fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::AutoMin { min_width: 1280.0, min_height: 720.0 },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        }
    ));
}

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}