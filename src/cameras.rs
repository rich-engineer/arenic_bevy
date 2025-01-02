
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::utils::HashMap;
use crate::arenas::ArenaEnum;

#[derive(Resource)]
struct ArenaCameraPositions(HashMap<ArenaEnum, Vec2>);

impl Default for ArenaCameraPositions {
    fn default() -> Self {
        Self(HashMap::from([
            (ArenaEnum::Labyrinth, Vec2::ZERO),
            (ArenaEnum::Sanctum, Vec2::ZERO),
            (ArenaEnum::Pawnshop, Vec2::ZERO),
            (ArenaEnum::Bastion, Vec2::new(0.5, 0.5)),
            (ArenaEnum::Mountain, Vec2::ZERO),
            (ArenaEnum::Crucible, Vec2::ZERO),
            (ArenaEnum::Casino, Vec2::ZERO),
            (ArenaEnum::Gala, Vec2::ZERO),
            (ArenaEnum::GuildHouse, Vec2::ZERO),
        ]))
    }
}
pub fn setup_scene(mut commands: Commands, arena_camera_position: Res<ArenaCameraPositions>) {
    let bastion_pos = arena_camera_position.0.get(&ArenaEnum::Bastion).copied().unwrap_or(Vec2::ZERO);
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            near: -1000.0,
            scale: 1.0,
            far: 1000.0,
            viewport_origin:bastion_pos,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
    ));
}

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArenaCameraPositions>()
            .add_systems(Startup, setup_scene);
    }
}
