use bevy::prelude::*;
use crate::abilities::Cooldown;

#[derive(Component)]
pub struct KeyboardInput {
    pub key: KeyCode,
    pub mode: InteractionMode,
}

#[derive(Component)]
pub struct ComboInput {
    pub keys: Vec<KeyCode>,
    pub mode: InteractionMode,
}

#[derive(Copy, Clone)]
pub enum InteractionMode {
    Tap,
    HoldRelease,
    Hold,
}

fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&KeyboardInput, &mut Cooldown)>,
) {
    for (input, mut cooldown) in &mut query {
        match input.mode {
            InteractionMode::Tap => {
                if keyboard.just_pressed(input.key) && cooldown.remaining <= 0.0 {
                    cooldown.remaining = cooldown.total;
                }
            }
            InteractionMode::Hold => {
                if keyboard.pressed(input.key) && cooldown.remaining <= 0.0 {
                    cooldown.remaining = cooldown.total;
                }
            }
            InteractionMode::HoldRelease => {
                if keyboard.just_released(input.key) && cooldown.remaining <= 0.0 {
                    cooldown.remaining = cooldown.total;
                }
            }
        }
    }
}

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input);
    }
}