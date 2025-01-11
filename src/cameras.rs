use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH, GAME_SCALE, GRID_WIDTH, HALF_TILE_SIZE, MENU_POS, MENU_SCALE, MENU_Y_OFFSET, OFFSET_MATRIX, TILE_SIZE};
use crate::state::{GameState, GlobalState};
use bevy::color::palettes::tailwind::GRAY_50;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::arena_components::{ActiveArena, Arena};

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            Update,
            (handle_camera_input).run_if(not(in_state(GameState::Title))),
        );
        app.add_systems(Update, update_camera.after(handle_camera_input));
    }
}

fn setup_camera(mut commands: Commands) {
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
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

fn handle_camera_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
    all_arenas: Query<(Entity, &Arena)>,
    mut state: ResMut<GlobalState>,
) {
    let Some((current_active_entity, current_arena)) = arena_query.iter_mut().next() else {
        warn!("No active Arena found");
        return; // or handle no active arena
    };
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        let new_id = (current_arena.id + 9 - 1) % 9;
        info!("New ID: {}", new_id);
        swap_active_arena(&mut commands, current_active_entity, new_id, &all_arenas);
    }
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        let new_id = (current_arena.id + 1) % 9;
        swap_active_arena(&mut commands, current_active_entity, new_id, &all_arenas);
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        state.active_menu = !state.active_menu;
    }
}
fn swap_active_arena(
    commands: &mut Commands,
    old_active: Entity,
    new_id: u8,
    all_arenas: &Query<(Entity, &Arena)>,
) {
    // Remove marker from old
    commands.entity(old_active).remove::<ActiveArena>();

    // Add marker to the new arena
    if let Some((new_entity, _arena)) = all_arenas.iter().find(|(_, a)| a.id == new_id) {
        commands.entity(new_entity).insert(ActiveArena);
    }
}


/// Reference
/// https://chatgpt.com/c/6780cc05-88f8-800c-8b99-610b39be98ce
fn update_camera(
    arena_query: Query<(&Arena, &Transform), (With<ActiveArena>, Without<Camera>)>,
    state: Res<GlobalState>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut projection, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    if state.active_menu {
        projection.scale = MENU_SCALE;
        camera_transform.translation = MENU_POS;
    } else {
        if let Ok((_, arena_transform)) = arena_query.get_single() {
            let pos = Vec3::new(
                arena_transform.translation.x + (ARENA_WIDTH/2.0 - HALF_TILE_SIZE),
                arena_transform.translation.y - (ARENA_HEIGHT/2.0 + TILE_SIZE),
                arena_transform.translation.z
            ) ;
            projection.scale = GAME_SCALE;
            camera_transform.translation = pos;
        }
    }
}
