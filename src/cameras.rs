use crate::constants::{
    GAME_SCALE, GRID_HEIGHT, GRID_WIDTH, MENU_POS, MENU_SCALE, OFFSET_MATRIX, TILE_SIZE,
};
use crate::state::GlobalState;
use bevy::color::palettes::tailwind::GRAY_50;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            Update,
            (
                handle_camera_input,
                update_camera.after(handle_camera_input),
            ),
        );
    }
}

fn setup_camera(mut commands: Commands, global_state: Res<GlobalState>) {
    let new_position = get_current_arena_pos(&global_state);
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::from(GRAY_50)),
            ..Default::default()
        },
        OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
        Transform {
            translation: new_position,
            ..Default::default()
        },
    ));
}

fn get_current_arena_pos(global_state: &Res<GlobalState>) -> Vec3 {
    let current_arena = global_state.current_arena as usize;
    let offset_x = GRID_WIDTH as f32 * TILE_SIZE;
    let offset_y = GRID_HEIGHT as f32 * TILE_SIZE;

    Vec3::new(
        offset_x * OFFSET_MATRIX[current_arena].x - 1.0,
        offset_y * OFFSET_MATRIX[current_arena].y - 1.0,
        0.0,
    )
}

fn handle_camera_input(
    mut global_state: ResMut<GlobalState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        global_state.current_arena = (global_state.current_arena + 9 - 1) % 9;
    }
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        global_state.current_arena = (global_state.current_arena + 1) % 9;
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        global_state.active_menu = !global_state.active_menu;
    }
}

fn update_camera(
    state: Res<GlobalState>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut projection, mut transform)) = query.get_single_mut() else {
        return;
    };

    let (scale, position) = if state.active_menu {
        (MENU_SCALE, MENU_POS)
    } else {
        (GAME_SCALE, get_current_arena_pos(&state))
    };

    projection.scale = scale;
    transform.translation = position;
}
