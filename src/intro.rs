use bevy::prelude::*;
use crate::arenas::{Arena};
use crate::characters::{CharacterClass, CharacterClassEnum, CharacterName, CharacterType, CharacterTypeEnum, ParentArena};
use crate::constants::{ARENA_CENTER};

use crate::state::{GameState, GlobalState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(OnEnter(GameState::Intro), intro_spawn_guildmaster.after(set_camera_pos));
        app.add_systems(OnEnter(GameState::Intro), select_first_hero_in_current_arena.after(intro_spawn_guildmaster));
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
    query: Query<(Entity, &ParentArena, &CharacterType, &Transform)>,
    state: Res<GlobalState>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("UI/player_selected.png");

    if let Some((hero_entity, p_arena, _, hero_transform)) = query
        .iter()
        .find(|(_, p, c, _)| p.0 == state.current_arena && c.0 == CharacterTypeEnum::Hero)
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








// let ability1 = AbilitySpawner::spawn_ability(
//     &mut commands,
//     AbilityNameEnum::SplitShot.to_display_string(),
//     "Next auto shot will fork",
//     5.0,
//     TargetTypeEnum::Directional,
//     CastTypeEnum::InstantCast,
//     vec![CharacterClassEnum::Hunter],
// );
// let ability2 = AbilitySpawner::spawn_ability(
//     &mut commands,
//     AbilityNameEnum::AutoShot.to_display_string(),
//     "Automatically fires shots in the forward direction",
//     1.0,
//     TargetTypeEnum::Directional,
//     CastTypeEnum::InstantCast,
//     vec![CharacterClassEnum::Hunter],
// );
// let ability3 = AbilitySpawner::spawn_ability(
//     &mut commands,
//     AbilityNameEnum::Trap.to_display_string(),
//     "Places a trap on the grid that deals damage when an enemy steps on it.",
//     1.0,
//     TargetTypeEnum::Directional,
//     CastTypeEnum::InstantCast,
//     vec![CharacterClassEnum::Hunter],
// );
//
// let ability4 = AbilitySpawner::spawn_ability(
//     &mut commands,
//     AbilityNameEnum::Snipe.to_display_string(),
//     "Fires any distance always at the boss",
//     4.0,
//     TargetTypeEnum::BossTarget,
//     CastTypeEnum::InstantCast,
//     vec![CharacterClassEnum::Hunter],
// );

// let key_bindings = vec![ability1, ability2, ability3, ability4]
//     .iter()
//     .enumerate()
//     .map(|(index, &entity)| {
//         let key = match index {
//             0 => KeyCode::Digit1,
//             1 => KeyCode::Digit2,
//             2 => KeyCode::Digit3,
//             3 => KeyCode::Digit4,
//             _ => KeyCode::KeyR,
//         };
//         (entity, key)
//     })
//     .collect();

// CharacterAbilities {
//     abilities: vec![ability1, ability2, ability3, ability4],
// },
// KeyBindingsForAbility {
//     bindings: key_bindings,
// },

// TODO()! set CharacterAbilities as a entity using set_parent() to character
// TODO()! figure out keybindings later