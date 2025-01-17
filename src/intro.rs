use crate::arenas::{Arena, SelectedHero};
use crate::characters::{CachedState, CharacterClass, CharacterClassEnum, CharacterName, CharacterType, CharacterTypeEnum, ParentArena, Selected};
use crate::constants::{
    ARENA_CENTER, ARENA_HEIGHT, ARENA_WIDTH, BOTTOM_BOUND, BOTTOM_ROW, HALF_TILE_SIZE, LEFT_BOUND,
    LEFT_COL, RECORD_TIME_SECONDS, RIGHT_BOUND, RIGHT_COL, TILE_SIZE, TOP_BOUND, TOP_ROW,
    TOTAL_COLS,
};
use crate::events::{ActionEnum, ActionEvent, EventTimeline, RecordMode};
use crate::interactions::KeyboardInput;
use crate::state::{GameState, GlobalState};
use bevy::prelude::*;

pub struct IntroPlugin;


impl Plugin for IntroPlugin {
    // TODO If your replay logic should run in a specific order relative to other systems, use .before() / .after() or the new .chain() approach in Bevy 0.11+.
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>();
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(
            OnEnter(GameState::Intro),
            intro_spawn_guildmaster_and_recruit.after(set_camera_pos),
        );
        app.add_systems(
            OnEnter(GameState::Intro),
            select_first_hero_in_current_arena.after(intro_spawn_guildmaster_and_recruit),
        );
        app.add_systems(Update, clear_timeline_on_record_start);
        app.add_systems(
            Update,
            (
                move_selected_hero,
                handle_hero_arena_transition,
                record_selected_character,
                playback_action_events,
                timeline_replay_event_system,
                cycle_hero_selection,
            )
                .chain(),
        );
    }
}

fn set_camera_pos(mut state: ResMut<GlobalState>) {
    state.current_arena = 8;
}


fn intro_spawn_guildmaster_and_recruit(
    mut commands: Commands,
    query: Query<(Entity, &Arena)>,
    asset_server: Res<AssetServer>,
    state: Res<GlobalState>,
) {
    let texture_selected = asset_server.load("UI/player_selected.png");
    let texture_unselected = asset_server.load("UI/player.png");
    let Some((arena_entity, _)) = query.iter().find(|(_, arena)| arena.id == state.current_arena) else {
        return;
    };

    let x = ARENA_CENTER.x;
    let y = ARENA_CENTER.y;

    let guildmaster_entity = commands
        .spawn((
            Transform::from_xyz(x - (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            CharacterName("Dean".to_string()),
            CharacterType(CharacterTypeEnum::Hero),
            CharacterClass(CharacterClassEnum::GuildMaster),
            ParentArena(state.current_arena),
            Sprite {
                image: texture_selected,
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
        .set_parent(arena_entity).id();

    commands
        .spawn((
            Transform::from_xyz(x + (TILE_SIZE * 4.0), y, 9.0),
            InheritedVisibility::default(),
            GlobalTransform::default(),
            CharacterName("Matthew".to_string()),
            CharacterType(CharacterTypeEnum::Hero),
            CharacterClass(CharacterClassEnum::Hunter),
            ParentArena(state.current_arena),
            Sprite {
                image: texture_unselected,
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
        .set_parent(arena_entity);
}

/// # References
/// [Using Tags to Connect and Move Entities in a Parent-Child in ECS](https://stealth-startup.youtrack.cloud/issue/A-1/Using-Tags-to-Connect-and-Move-Entities-in-a-Parent-Child-in-ECS)
fn select_first_hero_in_current_arena(
    mut commands: Commands,
    arena_query: Query<(Entity, &Arena, &SelectedHero)>,
    hero_query: Query<Entity, With<CharacterType>>,
    state: Res<GlobalState>,
    asset_server: Res<AssetServer>,
) {
    // Load the texture used for the hero selection marker
    // let selection_texture = asset_server.load("UI/player_selected.png");

    // Find the arena whose ID matches the current arena
    let Some((arena_entity, _, selected_hero)) = arena_query
        .iter()
        .find(|(_, arena, _)| arena.id == state.current_arena)
    else {
        return;
    };

    // If there's already a selected hero, use that; otherwise pick the first available hero
    // let hero_to_highlight = if let Some(hero_entity) = selected_hero.0 {
    //     hero_entity
    // } else if let Some(first_hero) = hero_query.iter().next() {
    //     first_hero
    // } else {
    //     // No heroes to select
    //     return;
    // };
    //
    // // Spawn the marker as a child of the selected hero
    // commands
    //     .spawn((
    //         Transform::from_xyz(0.0, 0.0, -1.0),
    //         GlobalTransform::default(),
    //         Sprite {
    //             image: selection_texture,
    //             color: Color::srgba(0.0, 0.0, 0.0, 0.25),
    //             custom_size: Some(Vec2::new(24.0, 24.0)),
    //             ..default()
    //         },
    //     ))
    //     .set_parent(hero_to_highlight);
}


/// # Reference
/// [Mut Queries](https://stealth-startup.youtrack.cloud/issue/A-3/How-to-Fix-Transform-Mutations-in-Bevy-ECS)
fn move_selected_hero(
    mut commands: Commands,
    mut query: Query<(
        &ParentArena,
        &CharacterType,
        &mut Transform,
        &mut EventTimeline,
        &RecordMode,
        &CachedState,

    ), With<Selected>>,
    state: Res<GlobalState>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Find hero in current arena
    let Some((parent_arena, _, mut hero_transform, mut timeline, record_mode, cached_state)) =
        query
            .iter_mut()
            .find(|(p, c, ..)| p.0 == state.current_arena && c.0 == CharacterTypeEnum::Hero)
    else {
        return;
    };

    // Early return if not in correct recording mode
    if !matches!(record_mode, RecordMode::Empty | RecordMode::Recording) {
        return;
    }

    // Only record events if we're in Recording mode AND have a valid start time
    let should_record =
        *record_mode == RecordMode::Recording && cached_state.record_start_time.is_some();
    let current_relative_time = if let Some(start) = cached_state.record_start_time {
        time.elapsed_secs_f64() - start
    } else {
        0.0
    };

    if input.just_pressed(KeyCode::KeyW) {
        if hero_transform.translation.y >= (TOP_BOUND - TILE_SIZE)
            && state.is_in_current_arena(&TOP_ROW)
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

    if input.just_pressed(KeyCode::KeyA) {
        if hero_transform.translation.x < (LEFT_BOUND + TILE_SIZE)
            && state.is_in_current_arena(&LEFT_COL)
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
    if input.just_pressed(KeyCode::KeyS) {
        if hero_transform.translation.y < (BOTTOM_BOUND + TILE_SIZE)
            && state.is_in_current_arena(&BOTTOM_ROW)
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
    if input.just_pressed(KeyCode::KeyD) {
        if hero_transform.translation.x > (RIGHT_BOUND - TILE_SIZE)
            && state.is_in_current_arena(&RIGHT_COL)
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


fn cycle_hero_selection(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    // Query to find current arena and its selected hero
    arena_query: Query<(Entity, &Arena, &SelectedHero)>,
    // Query to find all heroes in the current arena
    heroes_query: Query<(Entity, &ParentArena), With<CharacterType>>,
    mut state: ResMut<GlobalState>,
    asset_server: Res<AssetServer>,
) {
    // Only run this system when Tab is pressed
    if !keyboard.just_pressed(KeyCode::Tab) {
        return;
    }

    // Find the current arena
    let Some((arena_entity, arena, selected_hero)) = arena_query
        .iter()
        .find(|(_, arena, _)| arena.id == state.current_arena)
    else {
        return;
    };

    // Get all heroes in current arena
    let heroes: Vec<Entity> = heroes_query
        .iter()
        .filter(|(_, parent_arena)| parent_arena.0 == arena.id)
        .map(|(entity, _)| entity)
        .collect();

    // If there are no heroes, return early
    if heroes.is_empty() {
        return;
    }

    // Find index of currently selected hero
    let current_index = if let Some(current_hero) = selected_hero.0 {
        heroes.iter().position(|&e| e == current_hero).unwrap_or(0)
    } else {
        0
    };
    info!("Pressed Tab {} ", current_index);
    // Calculate next index, wrapping around to 0 if we reach the end
    let next_index = (current_index + 1) % heroes.len();
    let next_hero = heroes[next_index];
    for hero in heroes.iter() {
        commands.entity(*hero).remove::<Selected>();
    }
    commands.entity(next_hero).insert(Selected);
}

fn playback_action_events(
    mut query: Query<(&mut Transform, &RecordMode), With<Selected>>,
    mut event_reader: EventReader<ActionEvent>,
    state: Res<GlobalState>,
) {
    // Identify the hero in the current arena
    let Ok((mut hero_transform, record_mode)) = query.get_single_mut() else {
        event_reader.clear();
        return;
    };

    // Early return if not in playback mode
    if *record_mode != RecordMode::Playback {
        event_reader.clear();
        return;
    }

    for event in event_reader.read() {
        match event.action {
            ActionEnum::KeyW => {
                // Example: Move up
                if hero_transform.translation.y >= (TOP_BOUND - TILE_SIZE)
                    && state.is_in_current_arena(&TOP_ROW)
                {
                    hero_transform.translation.y = TOP_BOUND;
                } else {
                    hero_transform.translation.y += TILE_SIZE;
                }
            }
            ActionEnum::KeyA => {
                if hero_transform.translation.x < (LEFT_BOUND + TILE_SIZE)
                    && state.is_in_current_arena(&LEFT_COL)
                {
                    hero_transform.translation.x = LEFT_BOUND;
                } else {
                    hero_transform.translation.x -= TILE_SIZE;
                }
            }
            ActionEnum::KeyS => {
                if hero_transform.translation.y < (BOTTOM_BOUND + TILE_SIZE)
                    && state.is_in_current_arena(&BOTTOM_ROW)
                {
                    hero_transform.translation.y = BOTTOM_BOUND;
                } else {
                    hero_transform.translation.y -= TILE_SIZE;
                }
            }
            ActionEnum::KeyD => {
                if hero_transform.translation.x > (RIGHT_BOUND - TILE_SIZE)
                    && state.is_in_current_arena(&RIGHT_COL)
                {
                    hero_transform.translation.x = RIGHT_BOUND;
                } else {
                    hero_transform.translation.x += TILE_SIZE;
                }
            }
        }
    }
}

fn handle_hero_arena_transition(
    mut commands: Commands,
    mut hero_query: Query<(Entity, &mut ParentArena, &CharacterType, &Transform)>,
    mut arena_query: Query<(Entity, &Arena, &GlobalTransform)>,
    mut state: ResMut<GlobalState>,
) {
    let Some((hero_entity, mut hero_arena_tag, hero_type, hero_transform)) =
        hero_query.iter_mut().find(|(_, p_arena, ctype, _)| {
            p_arena.0 == state.current_arena && ctype.0 == CharacterTypeEnum::Hero
        })
    else {
        return;
    };
    let hero_x = hero_transform.translation.x;
    let hero_y = hero_transform.translation.y;
    let mut new_arena_translation = Vec3::new(0.0, 0.0, 9.0);

    let mut new_arena_id = None;
    if hero_x < LEFT_BOUND && state.is_current_arena_not_in(&LEFT_COL) {
        new_arena_id = Some(state.current_arena - 1);
        new_arena_translation = Vec3::new(RIGHT_BOUND - TILE_SIZE, hero_y, 9.0);
    } else if hero_x > (RIGHT_BOUND - TILE_SIZE) && state.is_current_arena_not_in(&RIGHT_COL) {
        new_arena_id = Some(state.current_arena + 1);
        new_arena_translation = Vec3::new(LEFT_BOUND, hero_y, 9.0);
    } else if hero_y > TOP_BOUND && state.is_current_arena_not_in(&TOP_ROW) {
        new_arena_id = Some(state.current_arena - TOTAL_COLS);
        new_arena_translation = Vec3::new(hero_x, BOTTOM_BOUND, 9.0);
    } else if hero_y < BOTTOM_BOUND && state.is_current_arena_not_in(&BOTTOM_ROW) {
        new_arena_id = Some(state.current_arena + TOTAL_COLS);
        new_arena_translation = Vec3::new(hero_x, TOP_BOUND, 9.0);
    }

    if let Some(next_arena_id) = new_arena_id {
        // Find the new arena entity
        let Some((new_arena_entity, _, _)) = arena_query
            .iter_mut()
            .find(|(_, arena, _)| arena.id == next_arena_id)
        else {
            warn!("No arena found with id = {next_arena_id}");
            return;
        };

        hero_arena_tag.0 = next_arena_id;
        state.current_arena = next_arena_id;
        commands
            .entity(hero_entity)
            .set_parent(new_arena_entity)
            .insert(Transform {
                translation: new_arena_translation,
                ..Default::default()
            });
    }
}

fn record_selected_character(
    mut query: Query<(
        Entity,
        &mut RecordMode,
        &ParentArena,
        &CharacterType,
        &mut Transform,
        &mut CachedState,
        &mut EventTimeline,
    )>,
    mut state: ResMut<GlobalState>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // find the hero in the current arena
    if let Some((
        _,
        mut hero_record_mode,
        p_arena,
        c_type,
        mut hero_transform,
        mut cached_state,
        mut timeline,
    )) = query
        .iter_mut()
        .find(|(_, _, p, c, _, _, _)| p.0 == state.current_arena && c.0 == CharacterTypeEnum::Hero)
    {
        if (*hero_record_mode != RecordMode::Empty && cached_state.previous_arena != *p_arena) {
            *hero_record_mode = RecordMode::Empty;
            info!("RecordMode Transitioned because left Arena");
        }

        if *hero_record_mode == RecordMode::Recording {
            if let Some(start_time) = cached_state.record_start_time {
                let elapsed = time.elapsed_secs_f64() - start_time;
                if elapsed >= RECORD_TIME_SECONDS {
                    // Switch to Pending
                    *hero_record_mode = RecordMode::Pending;
                    info!("Recording time is up! Switching to Pending.");
                } else {
                    info!("elapsed {} / {}", elapsed, RECORD_TIME_SECONDS);
                }
            }
        }

        if input.just_pressed(KeyCode::KeyR) {
            // cycle the record mode
            match *hero_record_mode {
                RecordMode::Empty => {
                    *hero_record_mode = RecordMode::Recording;
                    info!("RecordMode changed to: Recording");
                }
                RecordMode::Recording => {
                    *hero_record_mode = RecordMode::Playback;
                    info!("RecordMode changed to: Playback");
                }
                RecordMode::Playback => {
                    *hero_record_mode = RecordMode::Pending;
                    info!("RecordMode changed to: Pending");
                }
                RecordMode::Pending => {
                    *hero_record_mode = RecordMode::Empty;
                    info!("RecordMode changed to: Empty");
                }
                _ => {
                    // or handle other transitions
                    *hero_record_mode = RecordMode::Empty;
                    info!("RecordMode changed to: Empty (fallback)");
                }
            }
        }
    }
}

fn timeline_replay_event_system(
    time: Res<Time>,
    mut query: Query<(&mut EventTimeline, &mut RecordMode, &mut CachedState), With<CharacterType>>,
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
fn clear_timeline_on_record_start(
    time: Res<Time>,
    mut query: Query<
        (
            &RecordMode,
            &mut EventTimeline,
            &mut CachedState,
            &mut Transform,
            &mut ParentArena,
        ),
        Changed<RecordMode>,
    >,
) {
    for (record_mode, mut timeline, mut cached_state, mut hero_transform, mut p_arena) in
        query.iter_mut()
    {
        if *record_mode == RecordMode::Recording {
            info!("************************************a");
            timeline.events.clear();
            cached_state.playback_start_time = None;
            cached_state.previous_transform = *hero_transform;
            cached_state.previous_arena = p_arena.clone();
            cached_state.record_start_time = Some(time.elapsed_secs_f64());
        }
        if *record_mode == RecordMode::Playback {
            *hero_transform = cached_state.previous_transform;
        }
    }
}
