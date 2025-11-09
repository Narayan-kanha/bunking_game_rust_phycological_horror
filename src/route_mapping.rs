use crate::endings::GameEnding;

pub fn route_timeline_path(id: usize) -> Option<&'static str> {
    match id {
        1 => Some("assets/narrative/path1_true_wake.yaml"),
        2 => Some("assets/narrative/path2_sunk_legend.yaml"),
        3 => Some("assets/narrative/path3_cycle_breaker.yaml"),
        4 => Some("assets/narrative/path4_fragmented_mind.yaml"),
        5 => Some("assets/narrative/path5_puppetmaster.yaml"),
        6 => Some("assets/narrative/path6_legend.yaml"),
        _ => None,
    }
}

pub fn route_result_ending(id: usize) -> Option<GameEnding> {
    use GameEnding::*;
    match id {
        1 => Some(TrueWake),
        2 => Some(SunkLegend),
        3 => Some(CycleBreaker),
        4 => Some(FragmentedMind),
        5 => Some(Puppetmaster),
        6 => Some(Legend),
        _ => None,
    }
}