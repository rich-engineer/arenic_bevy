use crate::state::GameState;
use bevy::{
    app::{App, Plugin},
    color::palettes::tailwind::{GRAY_100, GRAY_200, GRAY_50, GRAY_950},
    prelude::*,
    ui::{Display::Flex, FocusPolicy, JustifyContent},
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};

#[derive(Component)]
struct TitleScreenUI;

fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Migra-Extrabold.ttf");
    let font_light = asset_server.load("fonts/Migra-Extralight.ttf");

    commands
        .spawn((
            TitleScreenUI,
            Node {
                display: Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::Srgba(GRAY_50)),
        ))
        .with_children(|div| {
            div.spawn((
                TitleScreenUI,
                Node {
                    display: Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                Text::new("Arenic"),
                TextFont {
                    font,
                    font_size: 182.0,
                    ..default()
                },
                TextColor(Color::Srgba(GRAY_950)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            div.spawn((
                TitleScreenUI,
                Node {
                    padding: UiRect {
                        left: Val::Px(36.0),
                        right: Val::Px(36.0),
                        top: Val::Px(16.0),
                        bottom: Val::Px(16.0),
                    },
                    border: UiRect {
                        left: Val::Px(1.0),
                        right: Val::Px(1.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(1.0),
                    },
                    display: Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::Srgba(GRAY_950)),
                BorderRadius::all(Val::Px(4.0)),
                BackgroundColor(Color::Srgba(GRAY_200)),
                Interaction::default(),
                // Blocks focus, so we can see hovering behavior:
                FocusPolicy::Block,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TitleScreenUI,
                    Text::new("Start"),
                    TextFont {
                        font: font_light,
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(Color::Srgba(GRAY_950)),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
            });
        });
}

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_cursor_icons)
            .add_systems(OnEnter(GameState::Title), setup_title)
            .add_systems(
                Update,
                start_button_system.run_if(in_state(GameState::Title)),
            )
            .add_systems(OnExit(GameState::Title), cleanup_title);
    }
}

fn cleanup_title(mut commands: Commands, query: Query<Entity, With<TitleScreenUI>>) {
    for entity in &query {
        // Remove this entity and all its children
        commands.entity(entity).despawn_recursive();
    }
}

fn start_button_system(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
    cursor_icons: Res<CursorIcons>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::Srgba(GRAY_200));
                commands.entity(*window).insert(cursor_icons.0[1].clone());
                // next_state.set(GameState::Start)
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::Srgba(GRAY_100));
                commands.entity(*window).insert(cursor_icons.0[1].clone());
            }
            Interaction::None => {
                *color = BackgroundColor(Color::Srgba(GRAY_200));
                commands.entity(*window).insert(cursor_icons.0[0].clone());
            }
        }
    }
}

#[derive(Resource)]
struct CursorIcons(Vec<CursorIcon>);

fn init_cursor_icons(mut commands: Commands) {
    commands.insert_resource(CursorIcons(vec![
        SystemCursorIcon::Default.into(),
        SystemCursorIcon::Pointer.into(),
        SystemCursorIcon::Wait.into(),
        SystemCursorIcon::Text.into(),
    ]));
}
