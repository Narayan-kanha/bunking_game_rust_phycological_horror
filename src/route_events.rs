use bevy::prelude::*;
use crate::endings::GameEnding;

#[derive(Event)]
pub struct StartRoute {
    pub route_id: usize,
}

#[derive(Event)]
pub struct EndingCompleted {
    pub ending: GameEnding,
}

#[derive(Event)]
pub struct FinalBellUnlocked;