use crate::arena_components::{ActiveArena, Arena, GuildHouse, InactiveArena};
use crate::state::{GameState, GlobalState};
use bevy::prelude::*;
use crate::character_components::{GuildMaster, Hero, Hunter, Selected, SelectedShadow};
use crate::characters::{CachedState, CharacterName, ParentArena};
use crate::constants::{ARENA_CENTER, TILE_SIZE};
use crate::events::{EventTimeline, RecordMode};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_intro_arena);
        app.add_systems(
            OnEnter(GameState::Intro),
            intro_spawn_guildmaster_and_recruit.after(set_camera_intro_arena)
        );
        app.add_systems(
            Update,
            (
                set_arenas_active_or_inactive
                    .after(intro_spawn_guildmaster_and_recruit)
                    .run_if(in_state(GameState::Intro)),
                // cycle_selected_hero_system
                //     .after(set_arenas_active_or_inactive)
                //     .run_if(in_state(GameState::Intro)),
                // update_shadow_visibility
                //     .after(cycle_selected_hero_system)
                //     .run_if(in_state(GameState::Intro))
            ).chain()
        );

    }
}
fn set_camera_intro_arena(
    mut commands: Commands,
    active_arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
    guild_house: Query<Entity, With<GuildHouse>>,
) {
    if let Ok(gh_entity) = guild_house.get_single() {
        if active_arena_query.get_single().is_err() {
            warn!("No active Arena Inserting GuildHouse.");
            commands.entity(gh_entity).insert(ActiveArena);
        }
    } else {
        warn!("No active Guild House Created");
    }
}

fn set_arenas_active_or_inactive(
    mut commands: Commands,
    arenas_query: Query<(Entity, &Arena), With<Arena>>,
    active_arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
) {
    if let Ok((active_entity, active_arena)) = active_arena_query.get_single() {
        for (entity, arena) in arenas_query.iter() {
            if arena.id == active_arena.id {
                // Add ActiveArena only if it is not already present
                if entity != active_entity {
                    commands.entity(entity).insert(ActiveArena);
                }
            } else {
                // Remove ActiveArena from non-matching entities
                commands.entity(entity).remove::<ActiveArena>();
            }
        }
    } else {
        warn!("All Arenas are Inactive");
    }
}

fn intro_spawn_guildmaster_and_recruit(
    mut commands: Commands,
    guild_house:  Query<Entity, With<GuildHouse>>,
    asset_server: Res<AssetServer>,
    state: ResMut<GlobalState>,
) {

    let Ok(entity) = guild_house.get_single() else {
        warn!("No single GuildHouse entity found or multiple GuildHouse entities exist.");
        return;
    };
    let x = ARENA_CENTER.x;
    let y = ARENA_CENTER.y;
    let texture = asset_server.load("UI/player.png");
    let player_selected = asset_server.load("UI/player_selected.png");
    commands
        .spawn((
            Transform::from_xyz(x - (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            CharacterName("Dean".to_string()),
            Hero,
            GuildMaster,
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(19.0, 19.0)),
                ..default()
            },
            Selected,
            EventTimeline::default(),
            RecordMode::Empty,
            CachedState {
                previous_transform: Transform::IDENTITY,
                previous_arena: ParentArena(state.current_arena),
                record_start_time: Some(0.0),
                playback_start_time: None,
                playback_current_index: 0,
            },
        ))
        .set_parent(entity)
        .with_children(|parent| spawn_shadow(parent, player_selected.clone()));

    commands
        .spawn((
            Transform::from_xyz(x + (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            CharacterName("Matthew".to_string()),
            Hero,
            Hunter,
            ParentArena(state.current_arena),
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(19.0, 19.0)),
                ..default()
            },
            EventTimeline::default(),
            RecordMode::Empty,
            CachedState {
                previous_transform: Transform::IDENTITY,
                previous_arena: ParentArena(state.current_arena),
                record_start_time: Some(0.0),
                playback_start_time: None,
                playback_current_index: 0,
            },
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


// pub fn cycle_selected_hero_system(
//     input: Res<ButtonInput<KeyCode>>,
//     mut commands: Commands,
//     heroes_query: Query<(Entity, &ParentArena, Option<&Selected>), With<Hero>>,
//     mut state: ResMut<GlobalState>,
// ) {
//     // --- 4. Find the arena that matches `state.current_arena` ---
//     let selected_arena_id = state.current_arena;
//
//
//     // --- 5. Gather all heroes that belong to the selected arena ---
//     //    (i.e., heroes whose `ParentArena.0 == selected_arena_id`)
//     let arena_heroes: Vec<(Entity, bool)> = heroes_query
//         .iter()
//         .filter_map(|(hero_entity, parent_arena, selected)| {
//             if parent_arena.0 == selected_arena_id {
//                 Some((hero_entity, selected.is_some()))
//             } else {
//                 None
//             }
//         })
//         .collect();
//
//     // If there are no heroes in this arena, no need to do anything.
//     if arena_heroes.is_empty() {
//         info!("Selected {} has No arena_heroes", selected_arena_id);
//         return;
//     }
//
//     // --- 7. On `Tab` press, cycle selection among `arena_heroes` ---
//     if input.just_pressed(KeyCode::Tab) {
//         // Find which hero is currently selected (if any).
//         let selected_index = arena_heroes
//             .iter()
//             .position(|(_, is_selected)| *is_selected);
//
//         match selected_index {
//             Some(idx) => {
//                 // Remove Selected from the current hero
//                 commands.entity(arena_heroes[idx].0).remove::<Selected>();
//
//                 // Cycle to next hero in the list
//                 let next_idx = (idx + 1) % arena_heroes.len();
//
//                 // Add Selected to the next hero
//                 let hero = commands.entity(arena_heroes[next_idx].0).insert(Selected).id();
//                 state.selected_hero_by_arena[selected_arena_id as usize] = Some(hero);
//             }
//             None => {
//                 let hero = arena_heroes[0].0;
//                 commands.entity(hero).insert(Selected);
//                 // Don't forget to update the state here too
//                 state.selected_hero_by_arena[selected_arena_id as usize] = Some(hero);
//             }
//         }
//     }
// }

fn update_shadow_visibility(
    mut shadow_query: Query<(&Parent, &mut Visibility), With<SelectedShadow>>,
    selected_query: Query<(), With<Selected>>,
) {
    // Iterate through all shadow entities
    for (parent, mut visibility) in shadow_query.iter_mut() {
        // Check if parent entity has Selected component
        if selected_query.get(parent.get()).is_ok() {
            // Parent is selected, show shadow
            *visibility = Visibility::Visible;
        } else {
            // Parent not selected, hide shadow
            *visibility = Visibility::Hidden;
        }
    }
}

fn move_selected_hero(
    state: Res<GlobalState>,
) {
    let selected_arena_id = state.current_arena;
    let selected_hero_by_arena = state.selected_hero_by_arena[selected_arena_id as usize];
    if selected_hero_by_arena.is_none() {
        warn!("No hero selected for the current arena.");
        return;
    }

    let hero = selected_hero_by_arena.unwrap();

}