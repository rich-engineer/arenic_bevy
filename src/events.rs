use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum ActionEnum {
    KeyW,
    KeyS,
    KeyA,
    KeyD,
}

#[derive(Debug, Clone, Event)]
pub struct ActionEvent {
    pub action: ActionEnum,
    pub timestamp: f64,
}

#[derive(Component, Default)]
pub struct EventTimeline {
    pub events: Vec<ActionEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum RecordMode {
    Empty,
    Recording,
    Playback,
    Pending,
}

impl Default for RecordMode {
    fn default() -> Self {
        RecordMode::Empty
    }
}
