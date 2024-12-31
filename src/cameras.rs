use bevy::prelude::*;
use bevy::window::WindowResized;


#[derive(Resource)]
pub struct ScreenDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for ScreenDimensions {
    fn default() -> Self {
        // Default to some initial 16:9 size (e.g. 1280x720)
        Self {
            width: 1280.0,
            height: 720.0,
        }
    }
}

pub fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera2d,
    ));
}

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenDimensions>()
            .add_systems(Startup, (setup_scene, resize_system));
    }
}

pub fn resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut screen_dims: ResMut<ScreenDimensions>,
) {
    for resize_event in resize_reader.read() {
        let aspect_ratio = 16.0 / 9.0;
        let window_aspect = resize_event.width / resize_event.height;

        for mut projection in query.iter_mut() {
            let (new_width, new_height) = if window_aspect > aspect_ratio {
                let height = resize_event.height;
                (height * aspect_ratio, height)
            } else {
                let width = resize_event.width;
                (width, width / aspect_ratio)
            };
            let scale = projection.scale;
            projection.area = Rect {
                min: Vec2::new(-new_width / 2.0, -new_height / 2.0),
                max: Vec2::new(new_width / 2.0, new_height / 2.0),
            };
            projection.scale = scale;
            // Also update our resource so other systems know the new size
            screen_dims.width = new_width;
            screen_dims.height = new_height;
        }
    }
}