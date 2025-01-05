use bevy::color::palettes::tailwind::{ RED_200};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;


pub struct HighlightRectPlugin;

impl Plugin for HighlightRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
        app.add_systems(Startup, create_ui);
        app.add_systems(Startup, (setup_all_arenas));
        app.add_systems(Update, (highlight_arena_system));
    }
}

pub fn camera_setup(mut commands: Commands, window_q: Query<&Window>, asset_server: Res<AssetServer>) {
    // let viewport_origin = arena_camera_position.0[global_state.current_arena as usize].2;
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::from(RED_200)),
            ..Default::default()
        },
        OrthographicProjection {
            near: -1000.0,
            scale: 3.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5,0.5),
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        },
        // Magic Numbers that you must just live with
        Transform::from_xyz(-19.0, -54.0, 0.0),
    ));
}

fn create_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    )).with_children(create_top_navigation)
      .with_children(create_inner_container)
      .with_children(create_bottom_bar);
}

fn create_top_navigation(mut commands: &mut ChildBuilder) {
    let top_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
       Node {
           position_type: PositionType::Relative,
           width: Val::Percent(100.0),
           height: Val::Percent(5.9),
           ..default()
       },
       BackgroundColor(top_bar_color),
    ));
}
fn create_inner_container(mut commands: &mut ChildBuilder) {
    // no color, it only for spacing
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(92.0),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).with_children(create_left_navigation)
        .with_children(create_right_navigation);
}
fn create_left_navigation(mut commands: &mut ChildBuilder) {
    let left_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(left_bar_color),
    ));
}
fn create_right_navigation(mut commands: &mut ChildBuilder) {
    let left_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(1.71875),
            ..default()
        },
        BackgroundColor(left_bar_color),
    ));
}
fn create_bottom_bar(mut commands: &mut ChildBuilder) {
    let bottom_bar_color = Color::hsla(1.0, 1.0, 1.0, 1.0);
    commands.spawn((
        Node {
            height: Val::Percent(14.3),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(bottom_bar_color),
    ));
}
const GRID_WIDTH: usize = 65;
const GRID_HEIGHT: usize = 31;
const TILE_SIZE: f32 = 19.0;
const OFFSET_MATRIX: [Vec2; 9] = [
    Vec2::new(-1.0, 1.0), // 0
    Vec2::new(0.0, 1.0), // 1
    Vec2::new(1.0, 1.0), // 2
    Vec2::new(-1.0, 0.0), // 3
    Vec2::new(0.0, 0.0), // 4
    Vec2::new(1.0, 0.0), // 5
    Vec2::new(-1.0, -1.0), // 6
    Vec2::new(0.0, -1.0), // 7
    Vec2::new(1.0, -1.0), // 8
];
pub fn setup_all_arenas(mut commands:Commands,  asset_server: Res<AssetServer>, mut gizmos: Gizmos) {
    for offset in OFFSET_MATRIX.iter() {
        // Calculate grid offset to center it
        let total_width = GRID_WIDTH as f32 * TILE_SIZE;
        let total_height = GRID_HEIGHT as f32 * TILE_SIZE;
        let texture = asset_server.load("UI/default_tile.png");

        let start_x = -((total_width / 2.0) + (TILE_SIZE / 2.0)) + (total_width * offset.x);
        let start_y = (((total_height / 2.0) + (TILE_SIZE - 1.0)).floor()) +  (total_height * offset.y);
        gizmos.rect_2d(
            Vec2::ZERO,
            Vec2::new(total_width, total_height),
            Color::hsla(0.0, 0.0, 0.0, 1.0),
        );
        commands.spawn((
            Transform::from_xyz(start_x, start_y, 0.0),
            InheritedVisibility::default(),
        ))
            .with_children(|parent| setup_tiles(parent, texture));

    }
}
pub fn setup_tiles(mut commands: &mut ChildBuilder, texture: Handle<Image>) {

    for col in 0..GRID_WIDTH {
        for row in 0..GRID_HEIGHT {
            let x = col as f32 * TILE_SIZE;
            let y = - (row as f32 * TILE_SIZE);
            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    image: texture.clone(),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }

    }
}

fn highlight_arena_system(mut commands: Commands, mut gizmos: Gizmos) {

    let total_width = GRID_WIDTH as f32 * TILE_SIZE;
    let total_height = GRID_HEIGHT as f32 * TILE_SIZE;
    let pos = Vec2::new(
        total_width * OFFSET_MATRIX[0].x - TILE_SIZE,
        total_height * OFFSET_MATRIX[0].y + 27.0, // Magic number again >_>
    );
    gizmos.rect_2d(
        pos,
        Vec2::new(total_width, total_height),
        Color::hsla(0.0, 0.0, 0.0, 1.0),
    );
}


// draw a tile in the top left of the cutout viewport
// Position Camera Based on which arena
// Position Camera for Menu
// create top nav
// create first arena
// make toggleanle resoluions