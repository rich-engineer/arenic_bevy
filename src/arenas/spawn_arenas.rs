// use super::components::{ArenaComponent, ArenaNames, ArenaRootComponent, TileComponent};
// use crate::arenas::setup_scene::ScreenDimensions;
// use bevy::prelude::*;
//
// fn get_arena_colors() -> [Color; 8] {
//     [
//         Color::hsla(0.0, 1.0, 0.5, 1.0),     // Red
//         Color::hsla(120.0, 1.0, 0.5, 1.0),   // Green
//         Color::hsla(240.0, 1.0, 0.5, 1.0),   // Blue
//         Color::hsla(30.0, 1.0, 0.5, 1.0),    // Orange
//         Color::hsla(300.0, 1.0, 0.25, 1.0),  // Purple
//         Color::hsla(208.0, 1.0, 0.5, 1.0),   // Alice Blue
//         Color::hsla(60.0, 1.0, 0.5, 1.0),    // Yellow
//         Color::hsla(350.0, 0.47, 0.82, 1.0), // Pink
//     ]
// }
//
// pub fn spawn_arenas(mut commands: Commands) {
//     // Root component that says we want 2 rows x 4 columns of arenas.
//     let arenas = ArenaRootComponent {
//         arena_id: 1,
//         arena_total_columns: 4,
//         arena_total_rows: 2,
//     };
//
//     // Add a transform to center them if you wish
//     let offset_x = -1000.0;
//     let offset_y = 1280.0;
//
//     // Colors to cycle through
//     let colors = get_arena_colors();
//
//     // 1) Spawn a single "arenas_entity"
//     let arenas_entity = commands
//         .spawn((
//             arenas,
//             Transform::default(),
//             GlobalTransform::default(),
//             Visibility::default(),
//             InheritedVisibility::default(),
//         ))
//         .id();
//
//     // We'll define a large offset for each arena.
//     // Each arena is 54×16 wide = 864, 114×16 tall = 1824.
//     // Add a gap so they don't overlap.
//     let arena_width = 54.0 * 16.0;
//     let arena_height = 114.0 * 16.0;
//     let gap_x = 50.0;
//     let gap_y = 50.0;
//
//     let total_arenas = 2 * 4; // 8 total arenas
//     for arena_index in 0..total_arenas {
//         // Pick a color for this arena by cycling through the 8 color array
//         let color_index = arena_index % colors.len();
//         let debug_color = colors[color_index];
//
//         // Row and col from the 0..8 index
//         let row = arena_index / 4;
//         let col = arena_index % 4;
//
//         // Construct your ArenaComponent
//         let arena = ArenaComponent {
//             arena_id: arena_index, // or as u32, your choice
//             arena_name: ArenaNames::Hunter,
//             total_tiles_y: 114,
//             total_tiles_x: 54,
//             tile_width: 16.0,
//             tile_height: 16.0,
//             arena_debug_tile_color: debug_color,
//         };
//
//         // This transform places each arena in a grid
//         let arena_transform = Transform::from_xyz(
//             offset_x + col as f32 * (arena_width + gap_x),
//             offset_y - row as f32 * (arena_height + gap_y),
//             0.0,
//         );
//
//         // 2) Spawn the "arena_entity" **as a child** of arenas_entity
//         commands.entity(arenas_entity).with_children(|parent| {
//             parent
//                 .spawn((
//                     arena,
//                     arena_transform,
//                     GlobalTransform::default(),
//                     Visibility::default(),
//                     InheritedVisibility::default(),
//                 ))
//                 // 3) Now, immediately nest `.with_children(...)` to spawn the tiles
//                 .with_children(|tile_parent| {
//                     let total_tiles_y = 114;
//                     let total_tiles_x = 54;
//                     let tile_width = 16.0;
//                     let tile_height = 16.0;
//
//                     // Spawn all tiles
//                     for y in 0..total_tiles_y {
//                         for x in 0..total_tiles_x {
//                             tile_parent.spawn((
//                                 TileComponent {
//                                     x,
//                                     y,
//                                     // If your TileComponent expects a `usize`, do:
//                                     parent_arena_id: arena_index,
//                                     // If your TileComponent expects a `u32`, do:
//                                     // parent_arena_id: arena_index as u32,
//                                 },
//                                 Sprite {
//                                     custom_size: Some(Vec2::new(tile_width, tile_height)),
//                                     color: debug_color,
//                                     ..default()
//                                 },
//                                 Transform::from_xyz(
//                                     x as f32 * tile_width,
//                                     -(y as f32 * tile_height),
//                                     0.0,
//                                 ),
//                                 GlobalTransform::default(),
//                                 Visibility::default(),
//                                 InheritedVisibility::default(),
//                             ));
//                         }
//                     }
//                 });
//         });
//     }
// }
