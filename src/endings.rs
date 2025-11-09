/// Enumeration of all endings. The FinalBell (meta) is excluded from the
/// primary completion set except for unlocking logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEnding {
    TrueWake,
    CycleBreaker,
    Legend,
    Puppetmaster,
    FragmentedMind,
    SunkLegend,
    FinalBell, // Meta unlock sequence
}

impl GameEnding {
    pub fn is_primary(&self) -> bool {
        match self {
            GameEnding::FinalBell => false,
            _ => true,
        }
    }
}