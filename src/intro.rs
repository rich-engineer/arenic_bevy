use bevy::prelude::*;
use crate::arenas::{Arena};
use crate::characters::{CharacterClass, CharacterClassEnum, CharacterName, CharacterType, CharacterTypeEnum, ParentArena};
use crate::constants::{ARENA_CENTER, TILE_SIZE};
use crate::interactions::KeyboardInput;
use crate::state::{GameState, GlobalState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(OnEnter(GameState::Intro), intro_spawn_guildmaster.after(set_camera_pos));
        app.add_systems(OnEnter(GameState::Intro), select_first_hero_in_current_arena.after(intro_spawn_guildmaster));
        app.add_systems(Update, move_selected_hero);
    }
}
fn set_camera_pos(mut state: ResMut<GlobalState>) {
    state.current_arena = 4;
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
            // Now this compiles—mutable access to Transform
            hero_transform.translation.y += TILE_SIZE;
        }

        if input.just_pressed(KeyCode::KeyA) {
            // Now this compiles—mutable access to Transform
            hero_transform.translation.x -= TILE_SIZE;
        }

        if input.just_pressed(KeyCode::KeyS) {
            // Now this compiles—mutable access to Transform
            hero_transform.translation.y -= TILE_SIZE;
        }

        if input.just_pressed(KeyCode::KeyD) {
            // Now this compiles—mutable access to Transform
            hero_transform.translation.x += TILE_SIZE;
        }
    }
}

