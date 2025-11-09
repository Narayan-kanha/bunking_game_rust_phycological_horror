#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEnding {
    TrueWake,
    CycleBreaker,
    Legend,
    Puppetmaster,
    FragmentedMind,
    SunkLegend,
    FinalBell,
}

impl GameEnding {
    pub fn is_primary(&self) -> bool {
        !matches!(self, GameEnding::FinalBell)
    }
}