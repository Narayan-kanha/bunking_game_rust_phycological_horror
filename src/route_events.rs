use bevy::prelude::*;
use crate::endings::GameEnding;

/// Fired when player triggers an escape route (physical zone).
#[derive(Event)]
pub struct StartRoute {
    pub route_id: usize,
}

/// Fired when a timeline completes and its ending should be registered.
#[derive(Event)]
pub struct EndingCompleted {
    pub ending: GameEnding,
}

/// Optional: fired when Final Bell becomes unlocked.
#[derive(Event)]
pub struct FinalBellUnlocked;