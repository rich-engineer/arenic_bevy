use crate::arenas::{arena_offset, ActiveArena, Arenas, ArenasContainer};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH, GAME_SCALE, MENU_POS, MENU_SCALE, TILE_SIZE};
use crate::state::GlobalState;
use bevy::color::palettes::tailwind::GRAY_50;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::from(GRAY_50)),
            ..Default::default()
        },
        OrthographicProjection {
            near: -1000.0,
            scale: 3.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
        Transform {
            ..Default::default()
        },
    ));
}

fn update_camera_scale_position_by_arena(
    arena_query: Query<(&Arenas, &Transform), (With<ActiveArena>, Without<Camera>)>,
    state: Res<GlobalState>,
    mut camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let Ok((mut projection, mut camera_transform)) = camera.get_single_mut() else {
        info!("No Camera found ");
        return;
    };
    if state.active_menu {
        projection.scale = MENU_SCALE;
        camera_transform.translation = MENU_POS;
    } else {
        if let Ok((arena, arena_transform)) = arena_query.get_single() {
            let x = arena_transform.translation.x - ARENA_WIDTH;
            let y = arena_transform.translation.y + ARENA_HEIGHT;
            let pos = Vec3::new(x, y, arena_transform.translation.z);

            projection.scale = GAME_SCALE;

            camera_transform.translation = pos;
        } else {
            info!("Camera has no active menu");
        }
    }
}
fn handle_camera_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut arena_query: Query<(Entity, &Arenas), With<ActiveArena>>,
    all_arenas: Query<(Entity, &Arenas)>,
    mut state: ResMut<GlobalState>,
) {
    let Some((current_active_entity, current_arena)) = arena_query.iter_mut().next() else {
        warn!("No active Arena found");
        return; // or handle no active arena
    };
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        let new_id = (current_arena.order + 9 - 1) % 9;
        info!("New ID: {}", new_id);
        swap_active_arena(&mut commands, current_active_entity, new_id, &all_arenas);
    }
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        let new_id = (current_arena.order + 1) % 9;
        info!("New ID: {}", new_id);
        swap_active_arena(&mut commands, current_active_entity, new_id, &all_arenas);
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {
        state.active_menu = !state.active_menu;
    }
}
fn swap_active_arena(
    commands: &mut Commands,
    old_active: Entity,
    new_id: usize,
    all_arenas: &Query<(Entity, &Arenas)>,
) {
    // Remove marker from old
    commands.entity(old_active).remove::<ActiveArena>();

    // Add marker to the new arena
    if let Some((new_entity, ..)) = all_arenas.iter().find(|(_, a)| a.order == new_id) {
        commands.entity(new_entity).insert(ActiveArena);
    }
}
fn any_camera_control_keys_just_pressed(input: Res<ButtonInput<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::BracketRight)
        || input.just_pressed(KeyCode::BracketLeft)
        || input.just_pressed(KeyCode::KeyP)
}

fn highlight_arena_system(
    mut gizmos: Gizmos,
    active_arena: Query<(&Arenas, &Transform), With<ActiveArena>>,
    menu_state: Res<GlobalState>,
) {
    if (!menu_state.active_menu) {
        return;
    }
    if let Ok((arena, transform)) = active_arena.get_single() {
        let border_width = 8;
        let half_border_width = 4.0;
        for i in 0..border_width {
            let x = transform.translation.x - ARENA_WIDTH - half_border_width + i as f32;
            let y = transform.translation.y + ARENA_HEIGHT - half_border_width + i as f32;
            let pos = Vec2::new(x, y);
            gizmos.rect_2d(
                pos,
                Vec2::new(ARENA_WIDTH, ARENA_HEIGHT),
                Color::hsla(0.0, 0.0, 0.0, 1.0),
            );
        }
    }
}
pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_camera_scale_position_by_arena,
                handle_camera_input.run_if(any_camera_control_keys_just_pressed),
                highlight_arena_system,
            ),
        );
    }
}
