use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Title,
    IntroCutscene,
    Mission1,
    Caught,
    MissionComplete,
    OpenWorld,
    Paused,
    Settings,
    Demo,
}
