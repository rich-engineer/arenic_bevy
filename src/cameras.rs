use crate::arenas::ArenaEnum;
use crate::state::{GameState, GlobalState, SelectedArenaUpdatedEvent};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use std::f32::consts::PI;
use bevy::color::palettes::tailwind::GRAY_50;
use crate::characters::CharacterClassEnum;

#[derive(Resource)]
pub struct ArenaCameraPositions(pub(crate) Vec<(ArenaEnum, CharacterClassEnum, Vec2)>);


impl Default for ArenaCameraPositions {
    fn default() -> Self {
        Self(vec![
            (ArenaEnum::Labyrinth, CharacterClassEnum::Hunter, Vec2::new(1.45, -0.305)),
            (ArenaEnum::GuildHouse, CharacterClassEnum::GuildMaster, Vec2::new(0.5, -0.305)),
            (ArenaEnum::Sanctum, CharacterClassEnum::Cardinal,  Vec2::new(-0.45, -0.305)),
            (ArenaEnum::Mountain,CharacterClassEnum::Forager, Vec2::new(1.45, 0.54)),
            (ArenaEnum::Bastion, CharacterClassEnum::Warrior, Vec2::new(0.5, 0.54)),
            (ArenaEnum::Pawnshop,CharacterClassEnum::Thief,  Vec2::new(-0.45, 0.54)),
            (ArenaEnum::Crucible,CharacterClassEnum::Alchemist, Vec2::new(1.45, 1.385)),
            (ArenaEnum::Casino,CharacterClassEnum::Merchant, Vec2::new(0.5, 1.385)),
            (ArenaEnum::Gala,CharacterClassEnum::Bard, Vec2::new(-0.45, 1.385)),
        ])
    }
}

pub fn setup_scene(mut commands: Commands, arena_camera_position: Res<ArenaCameraPositions>, global_state: Res<GlobalState>,) {
    let viewport_origin = arena_camera_position.0[global_state.current_arena as usize].2;
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::from(GRAY_50)),
            ..Default::default()
        },
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
    keyboard_input: Res<ButtonInput<KeyCode>>, // Corrected from ButtonInput<KeyCode>
    mut global_state: ResMut<GlobalState>,
    arena_camera_positions: Res<ArenaCameraPositions>,
    mut camera_animation: ResMut<CameraAnimation>,
    camera_query: Query<&OrthographicProjection>,
    mut event_writer: EventWriter<SelectedArenaUpdatedEvent>,
) {
    let total_arenas = arena_camera_positions.0.len() as u8;

    // Handle Left Bracket '[' Press
    if keyboard_input.just_pressed(KeyCode::BracketLeft) {
        info!("Left bracket pressed");
        global_state.current_arena = (global_state.current_arena + total_arenas - 1) % total_arenas;

        if let Ok(projection) = camera_query.get_single() {
            let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].2;
            let new_scale = if global_state.active_menu { 3.0 } else { 1.0 };

            // Set up animation for position and scale
            camera_animation.start_position = projection.viewport_origin;
            camera_animation.end_position = new_viewport_origin;
            camera_animation.start_scale = projection.scale;
            camera_animation.end_scale = new_scale;
            camera_animation.timer.reset();
            camera_animation.animating = true;
        }
        // Emit the event so our UI update system can respond
        event_writer.send(SelectedArenaUpdatedEvent);
    }

    // Handle Right Bracket ']' Press
    if keyboard_input.just_pressed(KeyCode::BracketRight) {
        info!("Right bracket pressed");
        global_state.current_arena = (global_state.current_arena + 1) % total_arenas;

        if let Ok(projection) = camera_query.get_single() {
            let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].2;
            let new_scale = if global_state.active_menu { 3.0 } else { 1.0 };

            // Set up animation for position and scale
            camera_animation.start_position = projection.viewport_origin;
            camera_animation.end_position = new_viewport_origin;
            camera_animation.start_scale = projection.scale;
            camera_animation.end_scale = new_scale;
            camera_animation.timer.reset();
            camera_animation.animating = true;
        }
        // Emit the event so our UI update system can respond
        event_writer.send(SelectedArenaUpdatedEvent);
    }

    // Handle 'P' Key Press for Toggling Menu and Animating
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        info!("P pressed");
        global_state.active_menu = !global_state.active_menu;
        global_state.current_arena = 4; // Assuming arena 4 is the menu arena

        if let Ok(projection) = camera_query.get_single() {
            let new_viewport_origin = arena_camera_positions.0[global_state.current_arena as usize].2;
            let new_scale = if global_state.active_menu { 3.0 } else { 1.0 };

            // Set up animation for position and scale
            camera_animation.start_position = projection.viewport_origin;
            camera_animation.end_position = new_viewport_origin;
            camera_animation.start_scale = projection.scale;
            camera_animation.end_scale = new_scale;
            camera_animation.timer.reset();
            camera_animation.animating = true;
        }
    }
}




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
    let t = (elapsed / duration).clamp(0.0, 1.0);

    // Apply SineIn easing
    let eased_t = 1.0 - (t * PI * 0.5).cos();

    // Interpolate between start and end positions
    let new_position = camera_animation.start_position.lerp(camera_animation.end_position, eased_t);
    // Interpolate between start and end scales
    let new_scale = camera_animation.start_scale * (1.0 - eased_t) + camera_animation.end_scale * eased_t;

    // Update the camera's viewport_origin and scale
    if let Ok(mut projection) = camera_query.get_single_mut() {
        projection.viewport_origin = new_position;
        projection.scale = new_scale;
    }

    // End animation if timer is finished
    if camera_animation.timer.finished() {
        camera_animation.animating = false;
    }
}


// Stores the data needed to smoothly animate the camera position
#[derive(Resource)]
pub struct CameraAnimation {
    pub start_position: Vec2, // Starting viewport_origin
    pub end_position: Vec2,   // Target viewport_origin
    pub start_scale: f32,     // Starting scale
    pub end_scale: f32,       // Target scale
    pub timer: Timer,         // Animation timer (200ms)
    pub animating: bool,      // Whether an animation is in progress
}

impl Default for CameraAnimation {
    fn default() -> Self {
        Self {
            start_position: Vec2::ZERO,
            end_position: Vec2::ZERO,
            start_scale: 1.0,
            end_scale: 1.0,
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
                .add_event::<SelectedArenaUpdatedEvent>()
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
