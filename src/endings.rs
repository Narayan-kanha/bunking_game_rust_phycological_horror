// Silence dead_code warnings until FinalBell and is_primary are used by the Final Bell flow.
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEnding {
    TrueWake,
    CycleBreaker,
    Legend,
    Puppetmaster,
    FragmentedMind,
    SunkLegend,
    FinalBell, // Will be used by the meta-ending soon
}

impl GameEnding {
    pub fn is_primary(&self) -> bool {
        !matches!(self, GameEnding::FinalBell)
    }

}