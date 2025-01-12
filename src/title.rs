use bevy::color::palettes::tailwind::{GRAY_200, GRAY_50, GRAY_950};
use bevy::prelude::Display::Flex;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

#[derive(Component)]
pub struct TitleScreenUI;
pub fn title_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        app.add_systems(Startup, title_scene);
    }
}
