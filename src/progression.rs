use std::collections::HashSet;
use bevy::prelude::*;
use crate::endings::GameEnding;

#[derive(Debug, Resource)]
pub struct GameProgress {
    pub completed: HashSet<GameEnding>,
    pub final_bell_unlocked: bool,
}

impl Default for GameProgress {
    fn default() -> Self {
        Self {
            completed: HashSet::new(),
            final_bell_unlocked: false,
        }
    }
}

impl GameProgress {
    pub fn mark_completed(&mut self, ending: GameEnding) {
        self.completed.insert(ending);
        self.update_unlock();
    }

    pub fn all_primary_completed(&self) -> bool {
        use GameEnding::*;
        let required = [
            TrueWake,
            CycleBreaker,
            Legend,
            Puppetmaster,
            FragmentedMind,
            SunkLegend,
        ];
        required.iter().all(|e| self.completed.contains(e))
    }

    fn update_unlock(&mut self) {
        if self.all_primary_completed() {
            self.final_bell_unlocked = true;
        }
    }
}