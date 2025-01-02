use crate::arenas::ArenaEnum;
use crate::state::{GameState, GlobalState};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Resource)]
struct ArenaCameraPositions(Vec<(ArenaEnum, Vec2)>);

impl Default for ArenaCameraPositions {
    fn default() -> Self {
        Self(vec![
            (ArenaEnum::Labyrinth, Vec2::new(1.45, -0.35)),
            (ArenaEnum::GuildHouse, Vec2::new(0.5, -0.35)),
            (ArenaEnum::Sanctum,  Vec2::new(-0.45, -0.35)),
            (ArenaEnum::Mountain, Vec2::new(1.45, 0.5)),
            (ArenaEnum::Bastion, Vec2::new(0.5, 0.5)),
            (ArenaEnum::Pawnshop,  Vec2::new(-0.45, 0.5)),
            (ArenaEnum::Crucible, Vec2::new(1.45, 1.35)),
            (ArenaEnum::Casino, Vec2::new(0.5, 1.35)),
            (ArenaEnum::Gala, Vec2::new(-0.45, 1.35)),
        ])
    }
}

pub fn setup_scene(mut commands: Commands, arena_camera_position: Res<ArenaCameraPositions>, global_state: Res<GlobalState>,) {
    let viewport_origin = arena_camera_position.0[global_state.current_arena as usize].1;
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            near: -1000.0,
            scale: if global_state.active_menu { 3.0 } else { 1.0 },
            far: 1000.0,
            viewport_origin,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
    ));
}


fn move_arenas_controls(
    keyboard_input:  Res<ButtonInput<KeyCode>>,
    mut global_state: ResMut<GlobalState>,
    arena_camera_positions: Res<ArenaCameraPositions>,
    mut camera_animation: ResMut<CameraAnimation>,
    // We need to query the current camera so we know its current viewport_origin
    // to set as the 'start' for our animation
    camera_query: Query<&OrthographicProjection>,
) {
    let total_arenas = arena_camera_positions.0.len() as u8;

    // Check for left bracket '['
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        info!("Left bracket pressed");
        global_state.current_arena = (global_state.current_arena + total_arenas - 1) % total_arenas;

        if let Ok(projection) = camera_query.get_single() {
            // new viewport target
            let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].1;

            // Set up animation resource
            camera_animation.start = projection.viewport_origin;
            camera_animation.end = new_viewport_origin;
            camera_animation.timer.reset();
            camera_animation.animating = true;
        }
    }

    // Check for right bracket ']'
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        info!("Right bracket pressed");
        global_state.current_arena = (global_state.current_arena + 1) % total_arenas;

        if let Ok(projection) = camera_query.get_single() {
            // new viewport target
            let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].1;

            // Set up animation resource
            camera_animation.start = projection.viewport_origin;
            camera_animation.end = new_viewport_origin;
            camera_animation.timer.reset();
            camera_animation.animating = true;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyP) {

        global_state.active_menu = !global_state.active_menu;

        if let Ok(projection) = camera_query.get_single() {
            // todo(harwood) Right here I need to figure out how to zoom to center and zoom out the camera.scale
            info!("P pressed");
            // // Set up animation resource
            // camera_animation.start = projection.viewport_origin;
            // camera_animation.end = new_viewport_origin;
            // camera_animation.timer.reset();
            // camera_animation.animating = true;
        }
    }
}

// Add this new system to update the camera viewport
fn update_camera_viewport(
    mut camera_query: Query<&mut OrthographicProjection>,
    arena_camera_positions: Res<ArenaCameraPositions>,
    global_state: Res<GlobalState>,
) {
    if let Ok(mut projection) = camera_query.get_single_mut() {
        // Get the new viewport origin from ArenaCameraPositions based on current_arena
        let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].1;

        // Update the viewport origin
        projection.viewport_origin = new_viewport_origin;
    }
}

use bevy::prelude::*;
use std::f32::consts::PI;

// SineIn(t) = 1.0 - cos(t * PI/2.0)

fn animate_camera_viewport(
    time: Res<Time>,
    mut camera_animation: ResMut<CameraAnimation>,
    mut camera_query: Query<&mut OrthographicProjection>,
) {
    if !camera_animation.animating {
        return;
    }

    // Advance the timer
    camera_animation.timer.tick(time.delta());
    let elapsed = camera_animation.timer.elapsed_secs();
    let duration = camera_animation.timer.duration().as_secs_f32();
    // clamp so we don’t go beyond 1.0
    let mut t = (elapsed / duration).clamp(0.0, 1.0);

    // --- Apply SineIn easing ---
    // f(t) = 1.0 - cos(t * π/2.0)
    let eased_t = 1.0 - (t * PI * 0.5).cos();

    // Interpolate between start and end
    let new_position = camera_animation.start.lerp(camera_animation.end, eased_t);

    // Update camera
    if let Ok(mut projection) = camera_query.get_single_mut() {
        projection.viewport_origin = new_position;
    }

    // End animation if timer done
    if camera_animation.timer.finished() {
        camera_animation.animating = false;
    }
}





use bevy::prelude::*;

// Stores the data needed to smoothly animate the camera position
#[derive(Resource)]
pub struct CameraAnimation {
    pub start: Vec2,       // Where the animation started (old position)
    pub end: Vec2,         // The target (new position)
    pub timer: Timer,      // 200ms timer
    pub animating: bool,   // Whether we're currently animating
}

impl Default for CameraAnimation {
    fn default() -> Self {
        Self {
            start: Vec2::ZERO,
            end: Vec2::ZERO,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            animating: false,
        }
    }
}


pub struct CamerasPlugin;

    impl Plugin for CamerasPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<ArenaCameraPositions>()
                .init_resource::<CameraAnimation>()
                .add_systems(Startup, setup_scene)
                // Correct run_if syntax
                .add_systems(
                    Update,
                    (
                        move_arenas_controls,
                        animate_camera_viewport,
                    )
                        .chain()
                        .run_if(in_state(GameState::Start))
                );
        }
    }
