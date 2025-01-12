use crate::arenas::{ActiveArena, Arenas, GuildHouse};
use crate::characters::{ActiveHero, Character, GuildMaster, Hero, Hunter, SelectedShadow};
use crate::constants::{
    ARENA_CENTER, ARENA_HEIGHT, ARENA_HEIGHT_HALF, ARENA_WIDTH_HALF, BOTTOM_BOUND, BOTTOM_ROW,
    LEFT_BOUND, LEFT_COL, RIGHT_BOUND, RIGHT_COL, TILE_SIZE, TOP_BOUND, TOP_ROW, TOTAL_COLS,
};
use bevy::prelude::*;
// use crate::events::{EventTimeline, RecordMode};

pub fn set_camera_intro_arena(
    mut commands: Commands,
    active_arena_query: Query<(Entity, &Arenas), With<ActiveArena>>,
    guild_house: Query<Entity, With<GuildHouse>>,
) {
    // First check if we already have an active arena
    if let Ok((entity, _)) = active_arena_query.get_single() {
        info!("Active arena exists: {:?}", entity);
        return;
    }

    // If no active arena, try to set the guild house as active
    if let Ok(gh_entity) = guild_house.get_single() {
        info!("Setting guild house as active arena: {:?}", gh_entity);
        commands.entity(gh_entity).insert(ActiveArena);
    } else {
        error!("No guild house entity found!");
    }
}
pub fn intro_spawn_guildmaster_and_recruit(
    mut commands: Commands,
    guild_house: Query<(Entity, &Arenas), With<GuildHouse>>,
    asset_server: Res<AssetServer>,
) {
    let Ok((entity, arena)) = guild_house.get_single() else {
        warn!("No single GuildHouse entity found or multiple GuildHouse entities exist.");
        return;
    };
    info!("Spawning guildmaster {}", arena.order);
    let x = 0.0;
    let y = 0.0;
    let texture = asset_server.load("UI/player.png");
    let player_selected = asset_server.load("UI/player_selected.png");
    commands
        .spawn((
            Transform::from_xyz(x - (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            Character {
                name: "Dean".to_string(),
            },
            Hero,
            GuildMaster,
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(19.0, 19.0)),
                ..default()
            },
            ActiveHero,
            // EventTimeline::default(),
            // RecordMode::Empty,
        ))
        .set_parent(entity)
        .with_children(|parent| spawn_shadow(parent, player_selected.clone()));

    commands
        .spawn((
            Transform::from_xyz(x + (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            Character {
                name: "Matthew".to_string(),
            },
            Hero,
            Hunter,
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(19.0, 19.0)),
                ..default()
            },
            // EventTimeline::default(),
            // RecordMode::Empty,
        ))
        .set_parent(entity)
        .with_children(|parent| spawn_shadow(parent, player_selected.clone()));
}

fn spawn_shadow(commands: &mut ChildBuilder, texture: Handle<Image>) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -0.01),
        GlobalTransform::default(),
        SelectedShadow,
        Sprite {
            image: texture.clone(),
            color: Color::srgba(0.0, 0.0, 0.0, 0.25),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Visibility::Hidden,
    ));
}
fn move_active_hero(
    active_arena_query: Query<(Entity, &Arenas), (With<ActiveArena>)>,
    mut heroes_query: Query<(&Parent, &mut Transform), With<ActiveHero>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((active_arena_entity, arena)) = active_arena_query.get_single() else {
        warn!("Move: No active arena found or multiple arenas marked as active!");
        return;
    };
    let Some((_, mut hero_transform)) = heroes_query
        .iter_mut()
        .find(|(parent, ..)| parent.get() == active_arena_entity)
    else {
        warn!("Move: No selected hero found in the active arena!");
        return;
    };
    let hero_x = hero_transform.translation.x;
    let hero_y = hero_transform.translation.y;
    if input.just_pressed(KeyCode::KeyS) {
        if BOTTOM_ROW.contains(&arena.order) && hero_y < BOTTOM_BOUND {
            info!("BOT_ROW selected! {}", ARENA_WIDTH_HALF - 1.0);
        } else {
            hero_transform.translation.y -= TILE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::KeyA) {
        if LEFT_COL.contains(&arena.order) && hero_x < LEFT_BOUND {
            info!("LEFT_COL selected! {}", ARENA_WIDTH_HALF - 1.0);
        } else {
            hero_transform.translation.x -= TILE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::KeyD) {
        if RIGHT_COL.contains(&arena.order) && hero_x > RIGHT_BOUND {
            info!("RIGHT_COL selected! {}", ARENA_WIDTH_HALF - 1.0);
        } else {
            hero_transform.translation.x += TILE_SIZE;
        }
    }
    if input.just_pressed(KeyCode::KeyW) {
        if TOP_ROW.contains(&arena.order) && hero_y > TOP_BOUND {
            info!("TOP_ROW selected! {}", ARENA_HEIGHT / 2.0);
        } else {
            hero_transform.translation.y += TILE_SIZE;
        }
    }
}
fn transition_hero_to_new_active_arena(
    mut commands: Commands,
    active_arena_query: Query<(Entity, &Arenas), With<ActiveArena>>,
    mut hero_query: Query<(Entity, &Parent, &mut Transform), With<ActiveHero>>,
    arena_query: Query<(Entity, &Arenas)>,
) {
    let Ok((active_arena_entity, active_arena)) = active_arena_query.get_single() else {
        warn!("Transition:  No active arena found or multiple arenas marked as active!");
        return;
    };
    let Some((hero_entity, _, hero_transform)) = hero_query
        .iter_mut()
        .find(|(_, parent, _)| parent.get() == active_arena_entity)
    else {
        info!("Transition: No active Here Entity!");
        return;
    };
    let hero_x = hero_transform.translation.x;
    let hero_y = hero_transform.translation.y;
    let mut new_arena_id: Option<usize> = None;
    let mut new_arena_translation = hero_transform.translation;

    if hero_x < LEFT_BOUND && !LEFT_COL.contains(&active_arena.order) {
        new_arena_id = Some(active_arena.order - 1);
        new_arena_translation.x = RIGHT_BOUND - TILE_SIZE;
    }
    // Move right → arena.order + 1
    else if hero_x > (RIGHT_BOUND - TILE_SIZE) && !RIGHT_COL.contains(&active_arena.order) {
        new_arena_id = Some(active_arena.order + 1);
        new_arena_translation.x = LEFT_BOUND;
    }
    // Move up → arena.order - TOTAL_COLS
    else if hero_y > TOP_BOUND && !TOP_ROW.contains(&active_arena.order) {
        new_arena_id = Some(active_arena.order - TOTAL_COLS as usize);
        new_arena_translation.y = BOTTOM_BOUND;
    }
    // Move down → arena.order + TOTAL_COLS
    else if hero_y < BOTTOM_BOUND && !BOTTOM_ROW.contains(&active_arena.order) {
        new_arena_id = Some(active_arena.order + TOTAL_COLS as usize);
        new_arena_translation.y = TOP_BOUND;
    }
    if let Some(next_arena_id) = new_arena_id {
        // Find the new arena entity by its `Arena.id`.
        let Some((new_arena_entity, _)) = arena_query
            .iter()
            .find(|(_, arena)| arena.order == next_arena_id)
        else {
            warn!("No arena found with id = {next_arena_id}");
            return;
        };
        commands.entity(active_arena_entity).remove::<ActiveArena>();
        commands.entity(new_arena_entity).insert(ActiveArena);
        // Re-parent the hero to the new arena and move them appropriately.
        commands
            .entity(hero_entity)
            .set_parent(new_arena_entity)
            .insert(Transform {
                translation: new_arena_translation,
                ..Default::default()
            });
    }
}
fn move_keys_pressed(input: Res<ButtonInput<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::KeyS)
        || input.just_pressed(KeyCode::KeyW)
        || input.just_pressed(KeyCode::KeyA)
        || input.just_pressed(KeyCode::KeyD)
        || input.just_pressed(KeyCode::Tab)
}
pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((move_active_hero, transition_hero_to_new_active_arena).run_if(move_keys_pressed)),
        );
    }
}
