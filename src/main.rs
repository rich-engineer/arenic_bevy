use bevy::prelude::*;
mod abilities;
mod arenas;
mod characters;
mod global_chat;
mod interactions;
mod state;
mod title;
mod highlight_rect;
mod metadata;
mod shared_traits;

use crate::state::{GameState, GlobalState, StatePlugin};
use characters::{CharacterClassEnum, CharacterSpawner, CharacterTypeEnum, CharactersPlugin};
use global_chat::GlobalChatPlugin;
use interactions::InteractionsPlugin;
use title::TitlePlugin;
use abilities::{Ability, AbilitiesPlugin, AbilitySpawner, CastTypeEnum, TargetTypeEnum};
use highlight_rect::HighlightRectPlugin;
use shared_traits::EnumDisplay;

const RESOLUTION: (f32, f32) = (1280.0, 720.0);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Arenic".to_string(),
                    resolution: RESOLUTION.into(),
                    // resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_plugins(StatePlugin)

        .add_plugins(TitlePlugin)
        // .add_plugins(AbilitiesPlugin)


        // .add_plugins(CharactersPlugin)
        // .add_plugins(InteractionsPlugin)
        // .add_plugins(GlobalChatPlugin)
        // .add_systems(Startup, start_game)
        .add_plugins(HighlightRectPlugin)
        // .add_systems(
        //     Update,
        //     spawn_selected_character.run_if(in_state(GameState::Start)),
        // )
        .run();
}

fn start_game(mut commands: Commands, mut global_state: ResMut<GlobalState>) {
    let ability1 = AbilitySpawner::spawn_ability(
        &mut commands,
        Ability::SplitShot.to_display_string(),
        "Next auto shot will fork",
        5.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    let ability2 = AbilitySpawner::spawn_ability(
        &mut commands,
        Ability::AutoShot.to_display_string(),
        "Automatically fires shots in the forward direction",
        1.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    let ability3 = AbilitySpawner::spawn_ability(
        &mut commands,
        Ability::Trap.to_display_string(),
        "Places a trap on the grid that deals damage when an enemy steps on it.",
        1.0,
        TargetTypeEnum::Directional,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );

    let ability4 = AbilitySpawner::spawn_ability(
        &mut commands,
        Ability::Snipe.to_display_string(),
        "Fires any distance always at the boss",
        4.0,
        TargetTypeEnum::BossTarget,
        CastTypeEnum::InstantCast,
        vec![CharacterClassEnum::Hunter],
    );
    // help me set this character to a new resource below
    let guild_master = CharacterSpawner::spawn_character(
        &mut commands,
        "Dean",
        CharacterTypeEnum::Hero,
        CharacterClassEnum::Hunter,
        vec![ability1, ability2, ability3, ability4],
    );

    global_state.selected_character = Some(guild_master);
}

fn spawn_selected_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    global_state: Res<GlobalState>,
) {
    if global_state.selected_character.is_some() {
        commands.spawn((
            Sprite {
                image: asset_server.load("UI/player_selected.png"),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    }
}