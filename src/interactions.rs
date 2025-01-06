use bevy::prelude::*;

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

#[derive(Component)]
pub struct KeyBindingsForAbility {
    pub bindings: Vec<(Entity, KeyCode)>,
}

// pub struct InteractionsPlugin;
//
// impl Plugin for InteractionsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, handle_keyboard_input);
//     }
// }
