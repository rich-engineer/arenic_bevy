use crate::constants::{ARENA_HEIGHT, ARENA_HEIGHT_HALF, ARENA_WIDTH, ARENA_WIDTH_HALF, GRID_HEIGHT, GRID_WIDTH, MENU_POS, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component)]
pub struct ActiveArena;
#[derive(Component)]
pub struct Labyrinth;
#[derive(Component)]
pub struct Sanctum;

#[derive(Component)]
pub struct Pawnshop;

#[derive(Component)]
pub struct Bastion;

#[derive(Component)]
pub struct Mountain;
#[derive(Component)]
pub struct Crucible;
#[derive(Component)]
pub struct Casino;
#[derive(Component)]
pub struct Gala;
#[derive(Component)]
pub struct GuildHouse;

#[derive(Component)]
pub struct Arenas {
    pub(crate) order: usize,
}

#[derive(Component)]
pub struct ArenasContainer;

pub fn setup_arenas(
    mut commands: Commands,
    parent: Query<Entity, With<ArenasContainer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let arenas_container_entity = if let Ok(entity) = parent.get_single() {
        entity // Returns Entity
    } else {
        let y = ARENA_HEIGHT + 70.0;
        commands
            .spawn((
                ArenasContainer,
                Transform::from_xyz(-ARENA_WIDTH, y, 0.0),
                InheritedVisibility::default(),
                GlobalTransform::default(),
            ))
            .id()
    };

    for i in 0..9 {
        let (x, y) = arena_offset(i);
        let color = Color::hsl(360.0, 0.95, 0.7);

        // Spawn the entity, capturing the SpawnCommands so we can insert the marker
        let mut cmd = commands.spawn((
            Arenas { order: i },
            Transform::from_xyz(x, y, 0.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            Mesh2d(meshes.add(Rectangle::new(
                ARENA_WIDTH - TILE_SIZE,
                ARENA_HEIGHT - TILE_SIZE,
            ))),
            MeshMaterial2d(materials.add(color)),
        ));

        // Match on `i` and insert the appropriate marker
        match i {
            0 => {
                cmd.insert(Labyrinth);
            }
            1 => {
                cmd.insert(GuildHouse);
            }
            2 => {
                cmd.insert(Sanctum);
            }
            3 => {
                cmd.insert(Pawnshop);
            }
            4 => {
                cmd.insert(Bastion);
            }
            5 => {
                cmd.insert(Mountain);
            }
            6 => {
                cmd.insert(Crucible);
            }
            7 => {
                cmd.insert(Casino);
            }
            8 => {
                cmd.insert(Gala);
            }
            _ => {}
        }

        cmd.set_parent(arenas_container_entity);
            // .with_children(|tiles_parent| {
            //     setup_tiles(tiles_parent, meshes.as_mut(), materials.as_mut());
            // });
    }
}

fn setup_tiles(
    commands: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Create the base white rectangle and black dot meshes once and reuse them
    let base_rect = meshes.add(Rectangle::new(19.0, 19.0));
    let center_dot = meshes.add(Rectangle::new(1.0, 1.0));

    // Create materials once and reuse them
    let white_material = materials.add(ColorMaterial {
        color: Color::hsla(0.0, 0.0, 1.0, 1.0), // White
        ..default()
    });
    let black_material = materials.add(ColorMaterial {
        color: Color::hsla(0.0, 0.0, 0.0, 1.0), // Black
        ..default()
    });

    for col in 0..GRID_WIDTH {
        for row in 0..GRID_HEIGHT {
            let x = col as f32 * TILE_SIZE - ARENA_WIDTH_HALF + (TILE_SIZE / 2.0);
            let y = -(row as f32 * TILE_SIZE - ARENA_HEIGHT_HALF) + (TILE_SIZE / 2.0);

            // Spawn the parent white rectangle
            commands
                .spawn((
                    Mesh2d(base_rect.clone()),
                    MeshMaterial2d(white_material.clone()),
                    Transform::from_xyz(x, y, 0.0),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Mesh2d(center_dot.clone()),
                        MeshMaterial2d(black_material.clone()),
                        Transform::from_xyz(0.0, 0.0, 0.1),
                    ));
                });
        }
    }
}

pub fn arena_offset(i: usize) -> (f32, f32) {
    let row = i / 3;
    let col = i % 3;
    let x = col as f32 * ARENA_WIDTH;
    let y = row as f32 * -ARENA_HEIGHT;
    (x, y)
}
