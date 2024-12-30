use bevy::prelude::{Component, KeyCode};

#[derive(Clone, Copy)]
pub enum InteractionMode {
    Tap,
    HoldRelease,
    Hold,
}

#[derive(Clone)]
pub enum InputBinding {
    Keyboard(KeyBinding),
    // Gamepad(GamepadBinding),
}

#[derive(Clone)]
pub enum KeyBinding {
    Single(KeyCode),
    Combo(Vec<KeyCode>),
}

// pub enum GamepadBinding {
//     Single(GamepadButtonType),
//     Combo(Vec<GamepadButtonType>),
// }


#[derive(Clone)]
pub struct Interaction {
    pub binding: InputBinding,
    pub mode: InteractionMode,
}