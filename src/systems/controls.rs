use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::utils::ensure_dir;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Controls {
    pub move_forward: KeyCode,
    pub move_back: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub interact: KeyCode,
    pub use_tool: KeyCode,
    pub open_inventory: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_back: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            interact: KeyCode::E,
            use_tool: KeyCode::Space,
            open_inventory: KeyCode::I,
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        let c = load_controls();
        app.insert_resource(c);
    }
}

fn load_controls() -> Controls {
    ensure_dir("config");
    let path = "config/controls.json";
    if let Ok(json) = fs::read_to_string(path) {
        if let Ok(c) = serde_json::from_str::<Controls>(&json) {
            return c;
        }
    }
    Controls::default()
}

fn save_controls(controls: Res<Controls>) {
    if let Ok(json) = serde_json::to_string_pretty(&*controls) {
        let _ = fs::write("config/controls.json", json);
    }
}
