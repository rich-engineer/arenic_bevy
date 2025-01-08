use bevy::prelude::*;
use crate::arenas::{Arena};
use crate::characters::{CharacterClass, CharacterClassEnum, CharacterName, CharacterType, CharacterTypeEnum, ParentArena};
use crate::constants::{ARENA_CENTER, ARENA_HEIGHT, ARENA_WIDTH, BOTTOM_BOUND, BOTTOM_ROW, HALF_TILE_SIZE, LEFT_BOUND, LEFT_COL, RIGHT_BOUND, RIGHT_COL, TILE_SIZE, TOP_BOUND, TOP_ROW, TOTAL_COLS};
use crate::interactions::KeyboardInput;
use crate::state::{GameState, GlobalState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(OnEnter(GameState::Intro), intro_spawn_guildmaster.after(set_camera_pos));
        app.add_systems(OnEnter(GameState::Intro), select_first_hero_in_current_arena.after(intro_spawn_guildmaster));
        app.add_systems(Update, (move_selected_hero, handle_hero_arena_transition));
    }
}


fn set_camera_pos(mut state: ResMut<GlobalState>) {
    state.current_arena = 8;
}
fn intro_spawn_guildmaster(
    mut commands: Commands,
    query: Query<(Entity, &Arena)>,
    asset_server: Res<AssetServer>,
    state: Res<GlobalState>
) {
    let texture = asset_server.load("UI/player_selected.png");
    info!( state.current_arena);
    if let Some((arena_entity, _)) = query
        .iter()
        .find(|(_, arena)| {
            arena.id == state.current_arena
        })
    {

        let x = ARENA_CENTER.x;
        let y = ARENA_CENTER.y;
        // TODO()! set CharacterAbilities as a entity using set_parent() to character
        // TODO()! figure out keybindings later
        commands
            .spawn((
                Transform::from_xyz(x, y, 9.0),
                InheritedVisibility::default(),
                GlobalTransform::default(),
                CharacterName("Dean".to_string()),
                CharacterType(CharacterTypeEnum::Hero),
                CharacterClass(CharacterClassEnum::GuildMaster),
                ParentArena(state.current_arena),
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::new(19.0, 19.0)),
                    ..default()
                }
            ))
            .set_parent(arena_entity);
    }
}


/// # References
/// [Using Tags to Connect and Move Entities in a Parent-Child in ECS](https://stealth-startup.youtrack.cloud/issue/A-1/Using-Tags-to-Connect-and-Move-Entities-in-a-Parent-Child-in-ECS)
fn select_first_hero_in_current_arena(
    mut commands: Commands,
    query: Query<(Entity, &ParentArena, &CharacterType)>,
    state: Res<GlobalState>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("UI/player_selected.png");

    if let Some((hero_entity, p_arena, _)) = query
        .iter()
        .find(|(_, p, c)| p.0 == state.current_arena && c.0 == CharacterTypeEnum::Hero)
    {
        /// # References
        /// [Parent/Child Sprite Layering in Bevy 2D](https://stealth-startup.youtrack.cloud/issue/A-2/Understanding-Parent-Child-Sprite-Layering-in-Bevy-2D)
        commands
            .spawn((
                Transform::from_xyz(0.0, 0.0, -1.0),
                GlobalTransform::default(),
                Sprite {
                    image: texture.clone(),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.25),
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..default()
                },
            ))
            .set_parent(hero_entity);
    } else {
        info!("No Hero found in arena {}", state.current_arena);
    }
}



/// # Reference
/// [Mut Queries](https://stealth-startup.youtrack.cloud/issue/A-3/How-to-Fix-Transform-Mutations-in-Bevy-ECS)
fn move_selected_hero(
    mut query: Query<(Entity, &ParentArena, &CharacterType, &mut Transform)>,
    state: Res<GlobalState>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Use `.iter_mut()` to get a mutable iterator
    if let Some((hero_entity, p_arena, _, mut hero_transform)) = query
        .iter_mut()
        .find(|(_, p, c, _)| p.0 == state.current_arena && c.0 == CharacterTypeEnum::Hero)
    {

        if input.just_pressed(KeyCode::KeyW) {
            if hero_transform.translation.y >= (TOP_BOUND - TILE_SIZE) && state.is_in_current_arena(&TOP_ROW)  {
                hero_transform.translation.y = TOP_BOUND;
            } else {
                hero_transform.translation.y += TILE_SIZE;
            }

        }

        if input.just_pressed(KeyCode::KeyA) {
            if hero_transform.translation.x < (LEFT_BOUND + TILE_SIZE) && state.is_in_current_arena(&LEFT_COL) {
                hero_transform.translation.x = LEFT_BOUND;
            } else {
                hero_transform.translation.x -= TILE_SIZE;
            }
        }
        if input.just_pressed(KeyCode::KeyS) {
            if hero_transform.translation.y < (BOTTOM_BOUND + TILE_SIZE) && state.is_in_current_arena(&BOTTOM_ROW) {
                hero_transform.translation.y = BOTTOM_BOUND;
            } else {
                hero_transform.translation.y -= TILE_SIZE;
            }
        }
        if input.just_pressed(KeyCode::KeyD) {
            if hero_transform.translation.x > (RIGHT_BOUND - TILE_SIZE) && state.is_in_current_arena(&RIGHT_COL) {
                hero_transform.translation.x = RIGHT_BOUND;
            } else {
                hero_transform.translation.x += TILE_SIZE;
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

    let Some((hero_entity, mut hero_arena_tag, hero_type, hero_transform)) = hero_query
        .iter_mut()
        .find(|(_, p_arena, ctype, _)| {
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
    } else if hero_y < BOTTOM_BOUND && state.is_current_arena_not_in(&BOTTOM_ROW)  {
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

        commands.entity(hero_entity).set_parent(new_arena_entity).insert(Transform {
            translation: new_arena_translation,
            ..Default::default()
        });

    }
}

