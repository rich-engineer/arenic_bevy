use crate::arenas::{ActiveArena, Arenas, GuildHouse};
use bevy::prelude::*;
use crate::characters::{ActiveHero, Character,  GuildMaster, Hero, Hunter, SelectedShadow};
use crate::constants::{ARENA_CENTER, TILE_SIZE};
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
    guild_house:  Query<(Entity, &Arenas), With<GuildHouse>>,
    asset_server: Res<AssetServer>,
    // active_arena_query: Query<&Arenas, With<ActiveArena>>,
) {
    // Get the arena that is currently active:
    // let Ok(active_arena) = active_arena_query.get_single() else {
    //     warn!("No active Arena found (or multiple arenas marked as active).");
    //     return;
    // };

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
            Character{name:"Dean".to_string()},
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
            Character{ name: "Matthew".to_string()},
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