use bevy::prelude::*;
use crate::arenas::{Arena};
use crate::characters::{CharacterClass, CharacterClassEnum, CharacterName, CharacterType, CharacterTypeEnum};
use crate::constants::{TILE_SIZE};

use crate::state::{GameState, GlobalState};

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Intro), set_camera_pos);
        app.add_systems(OnEnter(GameState::Intro), intro_spawn_guildmaster);
    }
}
fn set_camera_pos(mut state: ResMut<GlobalState>) {
    state.current_arena = 4;
}
fn intro_spawn_guildmaster(
    mut commands: Commands,
    query: Query<(Entity, &Arena, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("UI/player_selected.png");
    let target_arena_id = 1;


    if let Some((arena_entity, _, arena_transform)) = query
        .iter()
        .find(|(_, arena, _)| {
            arena.id == target_arena_id
        })
    {
        let x = arena_transform.translation.x * -1.0;
        let y = (arena_transform.translation.y - (TILE_SIZE / 2.0)) * -1.0;

        commands
            .spawn((
                Transform::from_xyz(x, y, 1.0),
                InheritedVisibility::default(),
                GlobalTransform::default(),
                CharacterName("Dean".to_string()),
                CharacterType(CharacterTypeEnum::Hero),
                CharacterClass(CharacterClassEnum::Hunter),
                Sprite {
                    image: texture,
                    custom_size: Some(Vec2::new(19.0, 19.0)),
                    ..default()
                }
            ))
            .set_parent(arena_entity);
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