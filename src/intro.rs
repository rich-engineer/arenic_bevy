use bevy::prelude::*;
use crate::arena_components::{ActiveArena, Arena, GuildHouse, InactiveArena};
use crate::state::{GameState, GlobalState};
use crate::character_components::{GuildMaster, Hero, Hunter, Selected, SelectedShadow};
use crate::characters::{CachedState, CharacterName, ParentArena};
use crate::constants::{ARENA_CENTER, BOTTOM_BOUND, BOTTOM_ROW, LEFT_BOUND, LEFT_COL, RECORD_TIME_SECONDS, RIGHT_BOUND, RIGHT_COL, TILE_SIZE, TOP_BOUND, TOP_ROW, TOTAL_COLS};
use crate::events::{ActionEnum, ActionEvent, EventTimeline, RecordMode};

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
                cycle_selected_hero_system
                    .after(intro_spawn_guildmaster_and_recruit)
                    .run_if(in_state(GameState::Intro))
                    .run_if(any_key_just_pressed),
                update_shadow_visibility
                    .after(cycle_selected_hero_system)
                    .run_if(in_state(GameState::Intro)),
                move_selected_hero
                    .after(update_shadow_visibility)
                    .run_if(in_state(GameState::Intro))
                    .run_if(any_key_just_pressed),
                handle_hero_arena_transition
                    .after(move_selected_hero)
                    .run_if(in_state(GameState::Intro))
                    .run_if(any_key_just_pressed),
                record_selected_character
                    .after(handle_hero_arena_transition)
                    .run_if(in_state(GameState::Intro)),
                playback_action_events
                    .after(record_selected_character)
                    .run_if(in_state(GameState::Intro)),
                timeline_replay_event_system
                    .after(playback_action_events)
                    .run_if(in_state(GameState::Intro)),
                clear_timeline_on_record_start
                    .after(timeline_replay_event_system)
                    .run_if(in_state(GameState::Intro))
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

fn intro_spawn_guildmaster_and_recruit(
    mut commands: Commands,
    guild_house:  Query<Entity, With<GuildHouse>>,
    asset_server: Res<AssetServer>,
    active_arena_query: Query<&Arena, With<ActiveArena>>,
) {
    // Get the arena that is currently active:
    let Ok(active_arena) = active_arena_query.get_single() else {
        warn!("No active Arena found (or multiple arenas marked as active).");
        return;
    };

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
                previous_arena: ParentArena(active_arena.id),
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
            ParentArena(active_arena.id),
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::new(19.0, 19.0)),
                ..default()
            },
            EventTimeline::default(),
            RecordMode::Empty,
            CachedState {
                previous_transform: Transform::IDENTITY,
                previous_arena: ParentArena(active_arena.id),
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


pub fn cycle_selected_hero_system(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    active_arena_query: Query<Entity, With<ActiveArena>>,
    heroes_query: Query<(Entity, &Parent, Option<&Selected>), With<Hero>>,
) {
    // 1. Find the single "active arena" entity
    let Ok(active_arena_entity) = active_arena_query.get_single() else {
        warn!("Cycle: No active arena found or multiple arenas marked as active!");
        return;
    };

    // 2. Gather all heroes that belong to the active arena.
    //    We do this by checking if hero's parent == the active arena entity.
    let arena_heroes: Vec<(Entity, bool)> = heroes_query
        .iter()
        .filter_map(|(hero_entity, parent, selected)| {
            if parent.get() == active_arena_entity {
                Some((hero_entity, selected.is_some()))
            } else {
                None
            }
        })
        .collect();

    // If there are no heroes in this active arena, do nothing.
    if arena_heroes.is_empty() {
        info!("Active arena has no heroes.");
        // TODO Select the first
        return;
    }

    // 3. On `Tab` press, cycle selection among `arena_heroes`.
    if input.just_pressed(KeyCode::Tab) {
        // Find which hero is currently selected (if any).
        let selected_index = arena_heroes
            .iter()
            .position(|(_, is_selected)| *is_selected);

        match selected_index {
            // If some hero is selected:
            Some(idx) => {
                // Remove Selected from the currently selected hero
                commands.entity(arena_heroes[idx].0).remove::<Selected>();

                // Move to the next hero in the list
                let next_idx = (idx + 1) % arena_heroes.len();

                // Add Selected to the new hero
                commands.entity(arena_heroes[next_idx].0).insert(Selected);
            }
            // If no hero is selected, select the first hero in the list.
            None => {
                let hero = arena_heroes[0].0;
                commands.entity(hero).insert(Selected);
            }
        }
    }
}


fn update_shadow_visibility(
    mut shadow_query: Query<(&Parent, &mut Visibility), With<SelectedShadow>>,
    selected_query: Query<(), With<Selected>>,
) {
    // TODO maybe update (last q/a https://chatgpt.com/share/6781fd52-8218-800c-b63a-f1a23e493650)
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

fn any_key_just_pressed(input: Res<ButtonInput<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::KeyS)
        || input.just_pressed(KeyCode::KeyW)
        || input.just_pressed(KeyCode::KeyA)
        || input.just_pressed(KeyCode::KeyD)
        || input.just_pressed(KeyCode::Tab)
        || input.just_pressed(KeyCode::KeyR)
}

fn key_r_just_pressed(input: Res<ButtonInput<KeyCode>>) -> bool {
    input.just_pressed(KeyCode::KeyR)
}

fn move_selected_hero(
    mut commands: Commands,
    active_arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
    mut heroes_query: Query<(
        &Parent,
        &mut Transform,
        &mut EventTimeline,
        &RecordMode,
        &CachedState,
    ), With<Selected>>,
    state: Res<GlobalState>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((active_arena_entity, arena)) = active_arena_query.get_single() else {
        warn!("Move: No active arena found or multiple arenas marked as active!");
        return;
    };

    // Only grab the *first* hero whose parent == active_arena_entity
    let Some((parent, mut hero_transform, mut timeline, record_mode, cached_state)) =
        heroes_query
            .iter_mut()
            .find(|(parent, ..)| parent.get() == active_arena_entity) else {
        warn!("Move: No selected hero found in the active arena!");
        return;
    };
    if !matches!(record_mode, RecordMode::Empty | RecordMode::Recording) {
        return;
    }

    let should_record =
        *record_mode == RecordMode::Recording && cached_state.record_start_time.is_some();
    let current_relative_time = if let Some(start) = cached_state.record_start_time {
        time.elapsed_secs_f64() - start
    } else {
        0.0
    }; if !matches!(record_mode, RecordMode::Empty | RecordMode::Recording) {
        return;
    }

    let should_record =
        *record_mode == RecordMode::Recording && cached_state.record_start_time.is_some();
    let current_relative_time = if let Some(start) = cached_state.record_start_time {
        time.elapsed_secs_f64() - start
    } else {
        0.0
    };

    if input.just_pressed(KeyCode::KeyS) {

        if hero_transform.translation.y < (BOTTOM_BOUND + TILE_SIZE)
            && BOTTOM_ROW.contains(&arena.id)
        {
            hero_transform.translation.y = BOTTOM_BOUND;
        } else {
            hero_transform.translation.y -= TILE_SIZE;
        }

        if should_record {
            timeline.events.push(ActionEvent {
                action: ActionEnum::KeyS,
                timestamp: current_relative_time,
            });
        }
    }

    if input.just_pressed(KeyCode::KeyW) {
        // If we’re at top boundary AND the arena is in TOP_ROW
        if hero_transform.translation.y >= (TOP_BOUND - TILE_SIZE)
            && TOP_ROW.contains(&arena.id)
        {
            hero_transform.translation.y = TOP_BOUND;
        } else {
            hero_transform.translation.y += TILE_SIZE;
        }

        if should_record {
            timeline.events.push(ActionEvent {
                action: ActionEnum::KeyW,
                timestamp: current_relative_time,
            });
        }
    }

    // Move Left (A)
    if input.just_pressed(KeyCode::KeyA) {
        if hero_transform.translation.x < (LEFT_BOUND + TILE_SIZE)
            && LEFT_COL.contains(&arena.id)
        {
            hero_transform.translation.x = LEFT_BOUND;
        } else {
            hero_transform.translation.x -= TILE_SIZE;
        }

        if should_record {
            timeline.events.push(ActionEvent {
                action: ActionEnum::KeyA,
                timestamp: current_relative_time,
            });
        }
    }




    // // Move Right (D)
    if input.just_pressed(KeyCode::KeyD) {
        if hero_transform.translation.x > (RIGHT_BOUND - TILE_SIZE)
            && RIGHT_COL.contains(&arena.id)
        {
            hero_transform.translation.x = RIGHT_BOUND;
        } else {
            hero_transform.translation.x += TILE_SIZE;
        }

        if should_record {
            timeline.events.push(ActionEvent {
                action: ActionEnum::KeyD,
                timestamp: current_relative_time,
            });
        }
    }
}


pub fn handle_hero_arena_transition(
    mut commands: Commands,
    active_arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
    mut hero_query: Query<(Entity, &Parent, &mut Transform), With<Selected>>,
    arena_query: Query<(Entity, &Arena)>,
) {
    let Ok((active_arena_entity, active_arena)) = active_arena_query.get_single() else {
        warn!("Transition:  No active arena found or multiple arenas marked as active!");
        return;
    };

    let Some((hero_entity, hero_parent, mut hero_transform)) =
        hero_query
            .iter_mut()
            .find(|(_, parent, _)| parent.get() == active_arena_entity)
    else {
        info!("Transition: No active Here Entity!");
        return;
    };

    let hero_x = hero_transform.translation.x;
    let hero_y = hero_transform.translation.y;

    // 3) Determine if the hero is crossing a boundary to an adjacent arena.
    let mut new_arena_id: Option<u8> = None;
    let mut new_arena_translation = hero_transform.translation;

    // Move left → arena.id - 1
    if hero_x < LEFT_BOUND && !LEFT_COL.contains(&active_arena.id) {
        new_arena_id = Some(active_arena.id - 1);
        new_arena_translation.x = RIGHT_BOUND - TILE_SIZE;
    }
    // Move right → arena.id + 1
    else if hero_x > (RIGHT_BOUND - TILE_SIZE) && !RIGHT_COL.contains(&active_arena.id) {
        new_arena_id = Some(active_arena.id + 1);
        new_arena_translation.x = LEFT_BOUND;
    }
    // Move up → arena.id - TOTAL_COLS
    else if hero_y > TOP_BOUND && !TOP_ROW.contains(&active_arena.id) {
        new_arena_id = Some(active_arena.id - TOTAL_COLS);
        new_arena_translation.y = BOTTOM_BOUND;
    }
    // Move down → arena.id + TOTAL_COLS
    else if hero_y < BOTTOM_BOUND && !BOTTOM_ROW.contains(&active_arena.id) {
        new_arena_id = Some(active_arena.id + TOTAL_COLS);
        new_arena_translation.y = TOP_BOUND;
    }

    // 4) If we found a new arena to move to, re-parent the hero.
    if let Some(next_arena_id) = new_arena_id {
        // Find the new arena entity by its `Arena.id`.
        let Some((new_arena_entity, _)) = arena_query
            .iter()
            .find(|(_, arena)| arena.id == next_arena_id)
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

fn playback_action_events(
    mut query: Query<(&mut Transform, &RecordMode, &Parent), With<Selected>>,
    mut event_reader: EventReader<ActionEvent>,
    arena_query: Query<&Arena>,
) {
    // 1) Get the single selected hero. If none or multiple, just clear events.
    let Ok((mut hero_transform, record_mode, parent)) = query.get_single_mut() else {
        event_reader.clear();
        return;
    };

    // 2) If not in playback mode, clear events and exit.
    if *record_mode != RecordMode::Playback {
        event_reader.clear();
        return;
    }

    // 3) Determine which arena the hero is in by reading the parent's Arena component
    let Ok(arena) = arena_query.get(parent.get()) else {
        event_reader.clear();
        return;
    };
    let arena_id = arena.id;

    // 4) Process each ActionEvent
    for event in event_reader.read() {
        match event.action {
            ActionEnum::KeyW => {
                if hero_transform.translation.y >= (TOP_BOUND - TILE_SIZE)
                    && TOP_ROW.contains(&arena_id)
                {
                    hero_transform.translation.y = TOP_BOUND;
                } else {
                    hero_transform.translation.y += TILE_SIZE;
                }
            }
            ActionEnum::KeyA => {
                if hero_transform.translation.x < (LEFT_BOUND + TILE_SIZE)
                    && LEFT_COL.contains(&arena_id)
                {
                    hero_transform.translation.x = LEFT_BOUND;
                } else {
                    hero_transform.translation.x -= TILE_SIZE;
                }
            }
            ActionEnum::KeyS => {
                if hero_transform.translation.y < (BOTTOM_BOUND + TILE_SIZE)
                    && BOTTOM_ROW.contains(&arena_id)
                {
                    hero_transform.translation.y = BOTTOM_BOUND;
                } else {
                    hero_transform.translation.y -= TILE_SIZE;
                }
            }
            ActionEnum::KeyD => {
                if hero_transform.translation.x > (RIGHT_BOUND - TILE_SIZE)
                    && RIGHT_COL.contains(&arena_id)
                {
                    hero_transform.translation.x = RIGHT_BOUND;
                } else {
                    hero_transform.translation.x += TILE_SIZE;
                }
            }
        }
    }
}


fn record_selected_character(
    active_arena_query: Query<(Entity, &Arena), With<ActiveArena>>,
    mut hero_query: Query<(
        Entity,
        &Parent,
        &mut RecordMode,
        &mut Transform,
        &mut CachedState,
        &mut EventTimeline,
    ), With<Selected>>,  // <-- only selected heroes
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // 1) Identify the active arena
    let Ok((active_arena_entity, arena)) = active_arena_query.get_single() else {
        warn!("No active arena found or multiple arenas marked as active!");
        return;
    };

    // 2) Identify the selected hero that belongs to `active_arena_entity`
    let Some((hero_entity, parent, mut record_mode, mut hero_transform, mut cached_state, mut timeline)) =
        hero_query
            .iter_mut()
            .find(|(_, parent, ..)| parent.get() == active_arena_entity)
    else {
        // No selected hero in the active arena
        return;
    };

    // 3) If the hero changed arenas, reset record mode if needed
    if *record_mode != RecordMode::Empty && cached_state.previous_arena.0 != arena.id {
        *record_mode = RecordMode::Empty;
        info!("RecordMode reset to Empty because hero left the arena");
    }

    // 4) Handle time-based logic (max record length, etc.)
    if *record_mode == RecordMode::Recording {
        if let Some(start_time) = cached_state.record_start_time {
            let elapsed = time.elapsed_secs_f64() - start_time;
            if elapsed >= RECORD_TIME_SECONDS {
                *record_mode = RecordMode::Pending;
                info!("Recording time is up! Switching to Pending.");
            }
        }
    }

    // 5) If R is pressed, cycle record modes
    if input.just_pressed(KeyCode::KeyR) {
        *record_mode = match *record_mode {
            RecordMode::Empty => {
                info!("RecordMode -> Recording");
                RecordMode::Recording
            }
            RecordMode::Recording => {
                info!("RecordMode -> Playback");
                RecordMode::Playback
            }
            RecordMode::Playback => {
                info!("RecordMode -> Pending");
                RecordMode::Pending
            }
            RecordMode::Pending | _ => {
                info!("RecordMode -> Empty");
                RecordMode::Empty
            }
        };
    }
}

fn clear_timeline_on_record_start(
    time: Res<Time>,
    mut query: Query<
        (
            &RecordMode,
            &mut EventTimeline,
            &mut CachedState,
            &mut Transform
        ),
        Changed<RecordMode>,
    >,
) {
    for (record_mode, mut timeline, mut cached_state, mut hero_transform) in query.iter_mut() {
        if *record_mode == RecordMode::Recording {
            timeline.events.clear();
            cached_state.playback_start_time = None;
            cached_state.previous_transform = *hero_transform;
            // cached_state.previous_arena = p_arena.clone(); // if you want to remove `ParentArena`, remove this line
            cached_state.record_start_time = Some(time.elapsed_secs_f64());
        }
        if *record_mode == RecordMode::Playback {
            *hero_transform = cached_state.previous_transform;
        }
    }
}

fn timeline_replay_event_system(
    time: Res<Time>,
    mut query: Query<(&mut EventTimeline, &mut RecordMode, &mut CachedState), With<Hero>>,
    mut event_writer: EventWriter<ActionEvent>,
) {
    for (mut timeline, mut record_mode, mut cached_state) in query.iter_mut() {
        if *record_mode != RecordMode::Playback {
            continue;
        }

        // If playback hasn't started, sort and set playback start
        if cached_state.playback_start_time.is_none() {
            timeline
                .events
                .sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
            cached_state.playback_start_time = Some(time.elapsed_secs_f64());
            cached_state.playback_current_index = 0;
        }

        let playback_elapsed = time.elapsed_secs_f64() - cached_state.playback_start_time.unwrap();

        if playback_elapsed > RECORD_TIME_SECONDS {
            *record_mode = RecordMode::Pending;
            cached_state.playback_start_time = None;
            cached_state.playback_current_index = 0;
            continue;
        }

        // Emit events that are "due"
        while cached_state.playback_current_index < timeline.events.len() {
            let e = &timeline.events[cached_state.playback_current_index];
            if e.timestamp <= playback_elapsed {
                event_writer.send(ActionEvent {
                    action: e.action.clone(),
                    timestamp: e.timestamp,
                });
                cached_state.playback_current_index += 1;
            } else {
                break;
            }
        }
    }
}